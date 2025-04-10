use kube::config::KubeConfigOptions;

use super::*;

impl Kubectl {
    pub async fn config(
        context: Option<&str>,
        cluster: Option<&str>,
        user: Option<&str>,
        debug: bool,
    ) -> kube::Result<kube::Config> {
        let options = KubeConfigOptions {
            context: context.map(ToString::to_string),
            cluster: cluster.map(ToString::to_string),
            user: user.map(ToString::to_string),
        };

        kube::Config::from_kubeconfig(&options)
            .await
            .inspect(|config| {
                if debug {
                    eprintln!("{config:#?}")
                } else {
                    tracing::debug!(?config)
                }
            })
            .inspect_err(|err| tracing::error!(%err, "from_kubeconfig"))
            .map_err(|_| kube::Error::LinesCodecMaxLineLengthExceeded)
    }
}
