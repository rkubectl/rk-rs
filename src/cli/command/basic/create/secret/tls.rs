use super::*;

// Create a TLS secret from the given public/private key pair.

//  The public/private key pair must exist beforehand. The public key certificate must be .PEM encoded and match the given
// private key.

// Examples:
//   # Create a new TLS secret named tls-secret with the given key pair
//   kubectl create secret tls tls-secret --cert=path/to/tls.crt --key=path/to/tls.key

#[derive(Clone, Debug, Args)]
#[command(arg_required_else_help(true), verbatim_doc_comment)]
pub struct CreateTlsSecret {
    /// Secret name
    name: String,

    /// Path to PEM encoded public key certificate.
    #[arg(long)]
    cert: PathBuf,

    /// Path to private key associated with given certificate.
    #[arg(long)]
    key: PathBuf,
}

impl CreateTlsSecret {
    pub fn secret(&self) -> kube::Result<corev1::Secret> {
        let cert = self.load(&self.cert)?;
        let key = self.load(&self.key)?;
        let data = [
            (corev1::Secret::TLS_CERT_KEY, cert),
            (corev1::Secret::TLS_PRIVATE_KEY_KEY, key),
        ];
        let secret = corev1::Secret::new(&self.name)
            .r#type(corev1::Secret::SECRET_TYPE_TLS)
            .string_data(data);
        Ok(secret)
    }

    fn load(&self, path: impl AsRef<Path>) -> kube::Result<String> {
        fs::read_to_string(path).map_err(|_err| kube::Error::LinesCodecMaxLineLengthExceeded)
    }
}
