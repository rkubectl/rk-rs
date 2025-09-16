use clap::{Args, Parser, Subcommand};
use rkubectl_args::{EnvFile, File, KeyValue};

use super::*;

pub use command::ApiResource;
pub use command::ApiResources;
pub use command::Auth;
pub use command::Command;
pub use command::Config;
pub use command::Create;
pub use command::CreateResource;
pub use command::Delete;
pub use command::Get;
pub use command::Node;
pub use command::Secret;

use context::Context;

mod command;
mod context;

#[derive(Debug, Parser)]
#[command(next_line_help = true, max_term_width = 120)]
pub struct Cli {
    /// Debug on/off
    #[arg(short, long, global = true)]
    pub debug: bool,

    #[command(flatten, next_display_order = 1000)]
    pub options: GlobalKubeapiOptions,

    #[command(flatten, next_display_order = 2000)]
    pub namespace: NamespaceOptions,

    #[command(flatten, next_display_order = 3000)]
    pub config: ConfigOptions,

    #[arg(short, long, value_enum, global = true, display_order = 10000)]
    pub output: Option<OutputFormat>,

    #[command(subcommand)]
    pub command: Command,
}

impl Cli {
    pub fn new() -> Self {
        Self::parse()
    }

    pub fn inspect(self) -> Self {
        debug!(cli = ?self, "rk");
        self
    }

    pub async fn exec(self) -> RkResult<()> {
        let kubeapi = self.kubeapi().await?;
        let ui = self.ui();
        let context = Context::new(kubeapi, ui);
        self.command.exec(&context).await
    }

    async fn kubeapi(&self) -> kube::Result<Kubeapi> {
        let namespace = self.namespace.namespace();
        let config_options = self.config.kube_config_options();
        let kubeapi = Kubeapi::new(config_options, self.debug, &self.options)
            .await
            .inspect(|kubeapi| info!(?kubeapi))?
            .with_namespace(namespace);

        Ok(kubeapi)
    }

    fn ui(&self) -> Ui {
        let output = self.output.unwrap_or_default();
        Ui::new(output)
    }
}

impl Default for Cli {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, Default, Args)]

pub struct NamespaceOptions {
    /// If present, list the requested object(s) across all namespaces.
    /// Namespace in current context is ignored even if specified with --namespace.
    #[arg(short = 'A', long, global = true, conflicts_with = "namespace")]
    pub all_namespaces: bool,

    /// If present, the namespace scope for this CLI request
    #[arg(short = 'n', long, global = true)]
    pub namespace: Option<String>,
}

impl NamespaceOptions {
    fn namespace(&self) -> Namespace {
        Namespace::new(self.all_namespaces, self.namespace.as_deref())
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
