use super::*;

/// Create a new secret for use with Docker registries.
///
///         Dockercfg secrets are used to authenticate against Docker registries.
///
///         When using the Docker command line to push images, you can authenticate to a given registry by running:
///         '$ docker login DOCKER_REGISTRY_SERVER --username=DOCKER_USER --password=DOCKER_PASSWORD --email=DOCKER_EMAIL'.
///
///  That produces a ~/.dockercfg file that is used by subsequent 'docker push' and 'docker pull' commands to authenticate
/// to the registry. The email address is optional.
///
///         When creating applications, you may have a Docker registry that requires authentication.  In order for the
///         nodes to pull images on your behalf, they must have the credentials.  You can provide this information
///         by creating a dockercfg secret and attaching it to your service account.
///
/// Examples:
///   # If you do not already have a .dockercfg file, create a dockercfg secret directly
///   kubectl create secret docker-registry my-secret --docker-server=DOCKER_REGISTRY_SERVER --docker-username=DOCKER_USER --docker-password=DOCKER_PASSWORD --docker-email=DOCKER_EMAIL
///
///   # Create a new secret named my-secret from ~/.docker/config.json
///   kubectl create secret docker-registry my-secret --from-file=path/to/.docker/config.json
///
#[derive(Clone, Debug, Args)]
#[command(arg_required_else_help(true), verbatim_doc_comment)]
pub struct CreateDockerRegistrySecret {
    /// Secret name
    name: String,

    #[command(flatten)]
    args: CreateDockerRegistrySecretArgs,
}

impl CreateDockerRegistrySecret {
    pub async fn exec(
        &self,
        kubectl: &Kubectl,
        pp: &api::PostParams,
    ) -> kube::Result<corev1::Secret> {
        trace!(?kubectl, ?pp, name = self.name);
        let secret = self.args.exec(&self.name).await?;
        // Create secret here
        Ok(secret)
    }
}

#[derive(Clone, Debug, Default, Args)]
#[group(multiple = false, required = true)]
struct CreateDockerRegistrySecretArgs {
    #[command(flatten)]
    direct: Option<DirectDockerRegistry>,
    // direct: DirectDockerRegistry,
    #[command(flatten)]
    file: Option<FromFileDockerRegistry>,
}

impl CreateDockerRegistrySecretArgs {
    async fn exec(&self, name: &str) -> kube::Result<corev1::Secret> {
        let secret = if let Some(file) = self.file.as_ref() {
            file.load(name)?
        } else if let Some(direct) = self.direct.as_ref() {
            direct.load(name)
        } else {
            unreachable!()
        };
        Ok(secret)
    }
}

#[derive(Clone, Debug, Args)]
// #[group(id = "direct")]
struct DirectDockerRegistry {
    /// Server location for Docker registry
    #[arg(
        long,
        value_name = "DOCKER_REGISTRY_SERVER",
        default_value = "https://index.docker.io/v1/"
    )]
    docker_server: String,

    /// Username for Docker registry authentication
    #[arg(long)]
    docker_username: String,

    /// Password for Docker registry authentication
    #[arg(long)]
    docker_password: String,

    /// Email for Docker registry
    #[arg(long)]
    docker_email: Option<String>,
}

impl DirectDockerRegistry {
    fn load(&self, name: &str) -> corev1::Secret {
        let registry = &self.docker_server;
        let username = &self.docker_username;
        let password = &self.docker_password;
        corev1::Secret::image_pull_secret(name, registry, username, password)
    }
}

#[derive(Clone, Debug, Args)]
// #[group(conflicts_with = "direct")]
struct FromFileDockerRegistry {
    /// Key files can be specified using their file path, in which case a default name
    /// of .dockerconfigjson will be given to them, or optionally with a name and file path,
    /// in which case the given name will be used.
    /// Specifying a directory will iterate each named file in the directory that is a valid secret key.
    /// For this command, the key should always be .dockerconfigjson.
    #[arg(long, value_parser = File::validating_value_parser(FromFileDockerRegistry::validate))]
    from_file: File,
}

impl FromFileDockerRegistry {
    fn validate(file: File) -> Result<File, String> {
        match file.key() {
            Some(".dockerconfigjson") => Ok(file),
            Some(other) => Err(format!("key '{other}' is invalid for this operation")),
            None => Ok(file),
        }
    }

    fn load(&self, name: &str) -> kube::Result<corev1::Secret> {
        let (key, config) = self.load_data()?.into_pair();
        debug_assert_eq!(key, corev1::Secret::SECRET_TYPE_DOCKER_CONFIG_JSON);
        let secret = corev1::Secret::docker_config_json_base64_encoded(name, config);
        Ok(secret)
    }

    fn load_data(&self) -> kube::Result<KeyValue<k8s::ByteString>> {
        let items = self
            .from_file
            .load()
            .map_err(serde::ser::Error::custom)
            .map_err(kube::Error::SerdeError)?;
        let Result::<[_; 1], _>::Ok([item]) = items.try_into() else {
            return Err(kube::Error::LinesCodecMaxLineLengthExceeded);
        };
        Ok(item.base64_encoded())
    }
}
