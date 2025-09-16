use clap::Args;

use super::*;

#[derive(Clone, Debug, Default, Args)]
pub struct KubeapiOptions {
    /// Default cache directory
    #[arg(long, global = true)]
    pub cache_dir: Option<PathBuf>,

    /// Username to impersonate for the operation. User could be a regular user or a service account in a namespace.
    #[arg(long = "as", global = true)]
    pub as_user: Option<String>,

    /// Group to impersonate for the operation, this flag can be repeated to specify multiple groups.
    #[arg(long, global = true)]
    pub as_group: Option<Vec<String>>,

    /// UID to impersonate for the operation.
    #[arg(long, global = true)]
    pub as_uid: Option<String>,
}

impl KubeapiOptions {
    pub fn cache_dir(&self) -> PathBuf {
        self.cache_dir
            .clone()
            .or_else(|| etcetera::home_dir().ok())
            .unwrap_or_default()
            .join(".kube")
            .join("cache")
    }

    pub fn discovery_cache_for_config(
        &self,
        config: &kube::Config,
    ) -> Result<PathBuf, kube::config::KubeconfigError> {
        config
            .cluster_url
            .host()
            .map(|server| self.cache_dir().join("discovery").join(server))
            .ok_or(kube::config::KubeconfigError::MissingClusterUrl)
    }
}

/// This struct mirrors `KubeConfigOptions` from `kube::config` crate.
/// It exists here to allow using the same struct in the CLI since
/// `kube::config::KubeConfigOptions` does not derive `clap::Args`.
#[derive(Clone, Debug, Default, Args)]
pub struct KubeConfigOptions {
    /// The name of the kubeconfig cluster to use
    #[arg(long, global = true)]
    pub cluster: Option<String>,

    /// The name of the kubeconfig context to use
    #[arg(long, global = true)]
    pub context: Option<String>,

    /// The name of the kubeconfig user to use
    #[arg(long, global = true)]
    pub user: Option<String>,
}

impl KubeConfigOptions {
    pub fn kube_config_options(&self) -> kube::config::KubeConfigOptions {
        kube::config::KubeConfigOptions {
            context: self.context.clone(),
            cluster: self.cluster.clone(),
            user: self.user.clone(),
        }
    }
}
