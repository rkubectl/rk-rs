use clap::{Args, Parser, Subcommand};

use super::*;

pub use command::ApiResource;
pub use command::ApiResources;
pub use command::Auth;
pub use command::Command;
pub use command::Config;
pub use command::Get;

mod command;

#[derive(Debug, Parser)]
pub struct Cli {
    #[arg(short, long, value_enum, global = true)]
    pub output: Option<OutputFormat>,

    /// Debug on/off
    #[arg(short, long, global = true)]
    pub debug: bool,

    /// If present, list the requested object(s) across all namespaces.
    /// Namespace in current context is ignored even if specified with --namespace.
    #[arg(short = 'A', long, global = true)]
    pub all_namespaces: bool,

    /// If present, the namespace scope for this CLI request
    #[arg(short = 'n', long, global = true)]
    pub namespace: Option<String>,

    /// The name of the kubeconfig cluster to use
    #[arg(long, global = true)]
    pub cluster: Option<String>,

    /// The name of the kubeconfig context to use
    #[arg(long, global = true)]
    pub context: Option<String>,

    /// The name of the kubeconfig user to use
    #[arg(long, global = true)]
    pub user: Option<String>,

    #[command(subcommand)]
    pub command: Command,
}

impl Cli {
    pub fn new() -> Self {
        Self::parse()
    }

    pub fn inspect(self) -> Self {
        tracing::debug!(cli = ?self, "rk");
        self
    }

    pub async fn exec(self) -> kube::Result<()> {
        let kubectl = self.kubectl().await?;
        match self.command {
            Command::ApiResources(api_resources) => api_resources.exec(&kubectl).await,
            Command::Auth(auth) => auth.exec(&kubectl).await,
            Command::ApiVersions => kubectl.api_versions().await,
            Command::Config(config) => config.exec(),
            Command::Get(get) => get.exec(&kubectl).await,
            Command::Features => kubectl.features().await,
            Command::Info => kubectl.info().await,
            Command::Version => kubectl.version().await,
        }
    }

    async fn kubectl(&self) -> kube::Result<Kubectl> {
        let namespace = self.namespace();
        let output = self.output.unwrap_or_default();
        let config = Kubectl::config(
            self.context.as_deref(),
            self.cluster.as_deref(),
            self.user.as_deref(),
            self.debug,
        )
        .await?;

        let kubectl = Kubectl::with_config(config, self.debug)
            .inspect(|kubectl| tracing::info!(?kubectl))?
            .with_namespace(namespace)
            .with_output(output);

        Ok(kubectl)
    }

    fn namespace(&self) -> Namespace {
        Namespace::new(self.all_namespaces, self.namespace.as_deref())
    }
}

impl Default for Cli {
    fn default() -> Self {
        Self::new()
    }
}

#[expect(dead_code)]
fn metadata(name: impl ToString) -> metav1::ObjectMeta {
    let name = Some(name.to_string());
    metav1::ObjectMeta {
        name,
        // annotations: todo!(),
        // creation_timestamp: todo!(),
        // deletion_grace_period_seconds: todo!(),
        // deletion_timestamp: todo!(),
        // finalizers: todo!(),
        // generate_name: todo!(),
        // generation: todo!(),
        // labels: todo!(),
        // managed_fields: todo!(),
        // namespace: todo!(),
        // owner_references: todo!(),
        // resource_version: todo!(),
        // self_link: todo!(),
        // uid: todo!(),
        ..default()
    }
}
