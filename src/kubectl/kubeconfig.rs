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
}
