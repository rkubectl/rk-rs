use kube::config::KubeConfigOptions;
use kube::config::Kubeconfig;
use kube::config::KubeconfigError;
use kube::config::NamedAuthInfo;
use kube::config::NamedCluster;
use kube::config::NamedContext;

use super::*;

impl Kubectl {
    pub async fn kubeconfig(
        context: Option<&str>,
        cluster: Option<&str>,
        user: Option<&str>,
        debug: bool,
    ) -> Result<(kube::Config, Kubeconfig), KubeconfigError> {
        let kubeconfig = Kubeconfig::read()
            .map(sanitize_kubeconfig)
            .inspect(|kubeconfig| {
                if debug {
                    debug!(kubeconfig = kubeconfig.debug())
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
                    debug!(?config)
                }
            })
            .inspect_err(|err| error!(%err, "from_kubeconfig"))
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

    pub fn view(&self) -> kube::Result<()> {
        yaml::to_string(&self.kubeconfig)
            .map(|config| println!("{config}"))
            .map_err(|_| kube::Error::LinesCodecMaxLineLengthExceeded)
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
