use clap::{Args, Parser, Subcommand};

use super::*;

pub use command::ApiResource;
pub use command::ApiResources;
pub use command::Auth;
pub use command::Command;
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

    #[command(subcommand)]
    pub command: Command,
}

impl Cli {
    pub fn new() -> Self {
        Self::parse()
    }

    pub async fn exec(self, kubectl: Kubectl) -> kube::Result<()> {
        let output = self.output.unwrap_or_default();
        let namespace = Namespace::new(self.all_namespaces, self.namespace);
        let kubectl = kubectl.namespace(namespace);
        match self.command {
            Command::ApiResources(api_resources) => api_resources.exec(&kubectl, output).await,
            Command::Auth(auth) => auth.exec(&kubectl, output).await,
            Command::ApiVersions => kubectl.api_versions().await,
            Command::Get(get) => get.exec(&kubectl, output).await,
        }
    }
}

impl Default for Cli {
    fn default() -> Self {
        Self::new()
    }
}
