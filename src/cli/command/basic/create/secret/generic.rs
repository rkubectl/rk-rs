use super::*;

/// Create a secret based on a file, directory, or specified literal value.
///
///  A single secret may package one or more key/value pairs.
///
///  When creating a secret based on a file, the key will default to the basename of the file, and the value will default to
/// the file content. If the basename is an invalid key or you wish to chose your own, you may specify an alternate key.
///
///  When creating a secret based on a directory, each file whose basename is a valid key in the directory will be packaged
/// into the secret. Any directory entries except regular files are ignored (e.g. subdirectories, symlinks, devices, pipes,
/// etc).
///
/// Examples:
///   # Create a new secret named my-secret with keys for each file in folder bar
///   kubectl create secret generic my-secret --from-file=path/to/bar
///
///   # Create a new secret named my-secret with specified keys instead of names on disk
///   kubectl create secret generic my-secret --from-file=ssh-privatekey=path/to/id_rsa --from-file=ssh-publickey=path/to/id_rsa.pub
///
///   # Create a new secret named my-secret with key1=supersecret and key2=topsecret
///   kubectl create secret generic my-secret --from-literal=key1=supersecret --from-literal=key2=topsecret
///
///   # Create a new secret named my-secret using a combination of a file and a literal
///   kubectl create secret generic my-secret --from-file=ssh-privatekey=path/to/id_rsa --from-literal=passphrase=topsecret
///
///   # Create a new secret named my-secret from env files
///   kubectl create secret generic my-secret --from-env-file=path/to/foo.env --from-env-file=path/to/bar.env
///
#[derive(Clone, Debug, Args)]
#[command(arg_required_else_help(true), verbatim_doc_comment)]
pub struct CreateGenericSecret {
    /// Secret name
    name: String,
    /// Specify the path to a file to read lines of key=val pairs to create a secret.
    #[arg(long, value_parser = EnvFile::value_parser())]
    from_env_file: Vec<EnvFile>,

    /// Key files can be specified using their file path, in which case a default name will be given to them,
    /// or optionally with a name and file path, in which case the given name will be used.
    /// Specifying a directory will iterate each named file in the directory that is a valid secret key.
    #[arg(long, value_parser = File::value_parser())]
    from_file: Vec<File>,

    /// Specify a key and literal value to insert in secret (i.e. mykey=somevalue)
    #[arg(long, value_parser = KeyValue::value_parser())]
    from_literal: Vec<KeyValue<String>>,
}

impl CreateGenericSecret {
    pub async fn secret(&self) -> kube::Result<corev1::Secret> {
        let data = self
            .all_inputs()
            .map_err(|_err| kube::Error::LinesCodecMaxLineLengthExceeded)?
            .map(|kv| kv.into_pair());
        let secret = corev1::Secret::opaque(&self.name).data(data);
        Ok(secret)
    }

    fn all_inputs(&self) -> io::Result<impl Iterator<Item = KeyValue<k8s::ByteString>>> {
        let literal = self.literal();
        let file = self.file()?;
        let enf_file = self.env_file()?;
        Ok(literal.chain(file).chain(enf_file))
    }

    fn env_file(&self) -> io::Result<impl Iterator<Item = KeyValue<k8s::ByteString>>> {
        let items = self
            .from_env_file
            .iter()
            .map(EnvFile::load)
            .collect::<io::Result<Vec<_>>>()?
            .into_iter()
            .flatten()
            .map(|kv| kv.base64_encoded());
        Ok(items)
    }

    fn file(&self) -> io::Result<impl Iterator<Item = KeyValue<k8s::ByteString>>> {
        let items = self
            .from_file
            .iter()
            .map(|file| file.load())
            .collect::<io::Result<Vec<_>>>()?
            .into_iter()
            .flatten()
            .map(|kv| kv.base64_encoded());
        Ok(items)
    }

    fn literal(&self) -> impl Iterator<Item = KeyValue<k8s::ByteString>> {
        self.from_literal
            .iter()
            .map(KeyValue::<String>::base64_encoded)
    }
}
