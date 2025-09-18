use kube::config::KubeConfigOptions;
use kube::config::Kubeconfig;
use kube::config::KubeconfigError;
use kube::config::NamedAuthInfo;
use kube::config::NamedCluster;
use kube::config::NamedContext;

use super::*;

impl Kubeapi {
    pub async fn kubeconfig(
        options: KubeConfigOptions,
        debug: bool,
    ) -> Result<(kube::Config, Kubeconfig), KubeconfigError> {
        let kubeconfig = Kubeconfig::read()
            .map(sanitize_kubeconfig)
            .inspect(|kubeconfig| {
                if debug {
                    debug!(kubeconfig = kubeconfig.debug())
                }
            })?;

        kube::Config::from_custom_kubeconfig(kubeconfig.clone(), &options)
            .await
            .inspect(|config| {
                if debug {
                    debug!(?config)
                }
            })
            .inspect_err(|err| error!(%err, "from_kubeconfig"))
            .map(|config| (config, kubeconfig))
    }

    pub fn current_context(&self) -> Option<&str> {
        self.kubeconfig.current_context.as_deref()
    }

    pub fn get_clusters(&self) -> &[kube::config::NamedCluster] {
        self.clusters()
    }

    pub fn get_contexts(&self) -> &[kube::config::NamedContext] {
        self.contexts()
    }

    pub fn get_users(&self) -> &[kube::config::NamedAuthInfo] {
        self.authinfo()
    }

    pub fn view(&self) -> yaml::Result<String> {
        yaml::to_string(&self.kubeconfig)
    }

    fn clusters(&self) -> &[NamedCluster] {
        &self.kubeconfig.clusters
    }

    fn contexts(&self) -> &[NamedContext] {
        &self.kubeconfig.contexts
    }

    fn authinfo(&self) -> &[NamedAuthInfo] {
        &self.kubeconfig.auth_infos
    }
}

fn sanitize_kubeconfig(mut kubeconfig: Kubeconfig) -> Kubeconfig {
    if kubeconfig.current_context().is_none() {
        let context = kubeconfig.contexts.first().map(|ctx| ctx.name.clone());
        debug!(
            first = context,
            "Using first context instead of invalid current_context"
        );
        kubeconfig.current_context = context;
    }
    kubeconfig
}

trait KubeconfigExt {
    fn debug(&self) -> String;
    fn get_context(&self, context: &str) -> Option<&NamedContext>;
    fn current_context(&self) -> Option<&NamedContext>;
}

impl KubeconfigExt for Kubeconfig {
    fn debug(&self) -> String {
        yaml::to_string(self).unwrap_or_default()
    }

    fn get_context(&self, context: &str) -> Option<&NamedContext> {
        self.contexts.iter().find(|ctx| ctx.name == context)
    }

    fn current_context(&self) -> Option<&NamedContext> {
        let context = self.current_context.as_deref()?;
        self.get_context(context)
    }
}
