use kube::config::KubeConfigOptions;

use super::*;

impl Kubectl {
    pub async fn kubeconfig(
        context: Option<&str>,
        cluster: Option<&str>,
        user: Option<&str>,
        debug: bool,
    ) -> Result<(kube::Config, kube::config::Kubeconfig), kube::config::KubeconfigError> {
        let kubeconfig = kube::config::Kubeconfig::read().inspect(|kubeconfig| {
            if debug {
                eprintln!("{kubeconfig:#?}")
            } else {
                tracing::debug!(?kubeconfig)
            }
        })?;

        let options = KubeConfigOptions {
            context: context.map(ToString::to_string),
            cluster: cluster.map(ToString::to_string),
            user: user.map(ToString::to_string),
        };

        kube::Config::from_custom_kubeconfig(kubeconfig.clone(), &options)
            .await
            .inspect(|config| {
                if debug {
                    eprintln!("{config:#?}")
                } else {
                    tracing::debug!(?config)
                }
            })
            .inspect_err(|err| tracing::error!(%err, "from_kubeconfig"))
            .map(|config| (config, kubeconfig))
    }

    pub fn current_context(&self) -> kube::Result<()> {
        println!(
            "{}",
            self.kubeconfig
                .current_context
                .as_deref()
                .unwrap_or_default()
        );
        Ok(())
    }

    pub fn get_clusters(&self) -> kube::Result<()> {
        self.clusters()
            .iter()
            .for_each(|cluster| println!("{}", cluster.name));
        Ok(())
    }

    pub fn get_contexts(&self) -> kube::Result<()> {
        self.contexts()
            .iter()
            .for_each(|ctx| println!("{}", ctx.name));
        Ok(())
    }

    pub fn get_users(&self) -> kube::Result<()> {
        self.authinfo()
            .iter()
            .for_each(|ai| println!("{}", ai.name));
        Ok(())
    }

    fn clusters(&self) -> &[kube::config::NamedCluster] {
        &self.kubeconfig.clusters
    }

    fn contexts(&self) -> &[kube::config::NamedContext] {
        &self.kubeconfig.contexts
    }

    fn authinfo(&self) -> &[kube::config::NamedAuthInfo] {
        &self.kubeconfig.auth_infos
    }
}
