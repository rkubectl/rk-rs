use tabled::settings::Padding;
use tabled::settings::Remove;
use tabled::settings::Style;
use tabled::settings::location::ByColumnName;

use k8s::ClusterRoleExt;
use k8s::NamespaceExt;
use k8s::PolicyRuleExt;
use k8s::SecretExt;

use super::*;

pub use api_resource::ApiResource;
pub use api_resource::ApiResources;
pub use basic::Basic;
pub use basic::Create;
pub use basic::CreateResource;
pub use cluster::ClusterInfo;
pub use cluster::ClusterManagement;
pub use cluster::Dump;
pub use config::Config;
pub use delete::Delete;
pub use get::Get;
pub use node::Node;
pub use secret::Secret;
pub use troubleshoot::Auth;
pub use troubleshoot::Debug;
pub use troubleshoot::TroubleshootingDebugging;

mod api_resource;
mod basic;
mod cluster;
mod config;
mod delete;
mod get;
mod node;
mod secret;
mod troubleshoot;

#[derive(Clone, Debug, Subcommand)]
pub enum Command {
    #[command(flatten,
        // next_help_heading = "Basic Commands (Beginner)"
    )]
    Basic(Basic),

    #[command(flatten)]
    Intermediate(Intermediate),

    #[command(flatten)]
    Deploy(Deploy),

    #[command(flatten)]
    ClusterManagement(ClusterManagement),

    #[command(flatten)]
    TroubleshootingDebugging(TroubleshootingDebugging),

    /// Print the supported API resources on the server.
    ApiResources(ApiResources),

    /// Print the supported API versions on the server, in the form of "group/version".
    ApiVersions,

    /// Modify kubeconfig file.
    #[command(subcommand, visible_alias = "cfg")]
    Config(Config),

    /// Print status of the K8s feaures in the current cluster.
    Features,

    /// Print brief server info
    Info,

    /// Print Node related info
    #[command(subcommand, visible_aliases = ["no", "nodes"])]
    Node(Node),

    /// Print Secret related info
    #[command(subcommand, visible_aliases = ["se", "sec", "secrets"])]
    Secret(Secret),

    /// Print client and server version
    Version,
}

impl Command {
    pub async fn exec(self, context: &Context) -> RkResult<()> {
        context.kubeapi().debug(&self);
        match self {
            Self::Basic(basic) => basic.exec(context).await,
            Self::Intermediate(intermediate) => intermediate.exec(context).await,
            Self::Deploy(deploy) => deploy.exec(context).await,
            Self::ClusterManagement(cluster_management) => cluster_management.exec(context).await,
            Self::TroubleshootingDebugging(tsd) => tsd.exec(context).await,
            Self::ApiResources(api_resources) => api_resources.exec(context).await,
            Self::ApiVersions => self.api_versions(context).await,
            Self::Config(config) => config.exec(context),
            Self::Features => self.features(context).await,
            Self::Info => self.info(context).await,
            Self::Node(node) => node.exec(context).await,
            Self::Secret(secret) => secret.exec(context).await,
            Self::Version => self.version(context).await,
        }
    }

    async fn api_versions(&self, context: &Context) -> RkResult<()> {
        let ui = context.ui();
        context
            .kubeapi()
            .api_versions()
            .await?
            .for_each(|version| ui.print(version.group_version));
        Ok(())
    }

    async fn features(&self, context: &Context) -> RkResult<()> {
        let output = context.output_deprecated();
        let features = context.kubeapi().features().await?;
        let show_params: ShowParams = default();
        context
            .ui()
            .print(features.output(false, &show_params, output));
        Ok(())
    }

    async fn info(&self, context: &Context) -> RkResult<()> {
        let info = context.kubeapi().info().await?;
        let ui = context.ui();

        ui.print(format!("build date:     {}", info.build_date));
        ui.print(format!("compiler:       {}", info.compiler));
        ui.print(format!("compiler:       {}", info.compiler));
        ui.print(format!("git_commit:     {}", info.git_commit));
        ui.print(format!("git_tree_state: {}", info.git_tree_state));
        ui.print(format!("git_version:    {}", info.git_version));
        ui.print(format!("go_version:     {}", info.go_version));
        ui.print(format!("major:          {}", info.major));
        ui.print(format!("minor:          {}", info.minor));
        ui.print(format!("platform:       {}", info.platform));

        Ok(())
    }

    async fn version(&self, context: &Context) -> RkResult<()> {
        let text = context.kubeapi().version().await?;
        context.ui().print(text);
        Ok(())
    }
}

/// Basic Commands (Intermediate)
#[derive(Clone, Debug, Subcommand)]
// #[command(subcommand_help_heading = "Basic Commands (Intermediate)")]
pub enum Intermediate {
    ///  Get documentation for a resource
    Explain,
    Get(Get),
    /// Edit a resource on the server
    Edit,
    /// Delete resources by file names, stdin, resources and names, or by resources and label selector
    Delete(Delete),
}

impl Intermediate {
    async fn exec(self, context: &Context) -> RkResult<()> {
        match self {
            Self::Explain => Err(RkError::todo()),
            Self::Get(get) => get.exec(context).await,
            Self::Edit => Err(RkError::todo()),
            Self::Delete(delete) => delete.exec(context).await,
        }
    }
}

/// Deploy Commands
#[derive(Clone, Debug, Subcommand)]

pub enum Deploy {
    /// Manage the rollout of a resource
    Rollout,

    /// Set a new size for a deployment, replica set, or replication controller
    Scale,

    /// Auto-scale a deployment, replica set, stateful set, or replication controller
    Autoscale,
}

impl Deploy {
    async fn exec(self, context: &Context) -> RkResult<()> {
        context.ui().not_implemented(&self);
        match self {
            Self::Rollout => Err(RkError::todo()),
            Self::Scale => Err(RkError::todo()),
            Self::Autoscale => Err(RkError::todo()),
        }
    }
}

// Troubleshooting and Debugging Commands:
//   describe        Show details of a specific resource or group of resources
//   logs            Print the logs for a container in a pod
//   attach          Attach to a running container
//   exec            Execute a command in a container
//   port-forward    Forward one or more local ports to a pod
//   proxy           Run a proxy to the Kubernetes API server
//   cp              Copy files and directories to and from containers
//   auth            Inspect authorization
//   debug           Create debugging sessions for troubleshooting workloads and nodes
//   events          List events

// Advanced Commands:
//   diff            Diff the live version against a would-be applied version
//   apply           Apply a configuration to a resource by file name or stdin
//   patch           Update fields of a resource
//   replace         Replace a resource by file name or stdin
//   wait            Experimental: Wait for a specific condition on one or many resources
//   kustomize       Build a kustomization target from a directory or URL

// Settings Commands:
//   label           Update the labels on a resource
//   annotate        Update the annotations on a resource
//   completion      Output shell completion code for the specified shell (bash, zsh, fish, or powershell)

// Subcommands provided by plugins:
//   features      The command features is a plugin installed by the user

// Other Commands:
//   api-resources   Print the supported API resources on the server
//   api-versions    Print the supported API versions on the server, in the form of "group/version"
//   config          Modify kubeconfig files
//   plugin          Provides utilities for interacting with plugins
//   version         Print the client and server version information
