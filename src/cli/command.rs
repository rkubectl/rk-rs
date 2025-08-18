use tabled::settings::Padding;
use tabled::settings::Remove;
use tabled::settings::Style;
use tabled::settings::location::ByColumnName;

use k8s::NamespaceExt;

use super::*;

pub use api_resource::ApiResource;
pub use api_resource::ApiResources;
pub use auth::Auth;
pub use basic::Create;
pub use basic::CreateResource;
pub use basic::Created;
pub use cascade::Cascade;
pub use config::Config;
pub use delete::Delete;
pub use dryrun::DryRun;
pub use get::Get;
pub use node::Node;

mod api_resource;
mod auth;
mod basic;
mod cascade;
mod config;
mod delete;
mod dryrun;
mod get;
mod node;

#[derive(Clone, Debug, Subcommand)]
pub enum Command {
    #[command(flatten, next_help_heading = "Basic Commands (Beginner)")]
    Basic(Basic),

    #[command(flatten)]
    Intermediate(Intermediate),

    #[command(flatten)]
    Deploy(Deploy),

    #[command(flatten)]
    ClusterManagement(ClusterManagement),

    /// Print the supported API resources on the server.
    ApiResources(ApiResources),

    /// Inspect authorization.
    #[command(subcommand)]
    Auth(Auth),

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

    /// Print client and server version
    Version,
}

impl Command {
    pub async fn exec(self, kubectl: &Kubectl) -> kube::Result<()> {
        kubectl.debug(&self);
        match self {
            Self::Basic(basic) => basic.exec(kubectl).await,
            Self::Intermediate(intermediate) => intermediate.exec(kubectl).await,
            Self::Deploy(deploy) => deploy.exec(kubectl).await,
            Self::ClusterManagement(cluster_management) => cluster_management.exec(kubectl).await,
            Self::ApiResources(api_resources) => api_resources.exec(kubectl).await,
            Self::Auth(auth) => auth.exec(kubectl).await,
            Self::ApiVersions => kubectl.api_versions().await,
            Self::Config(config) => config.exec(kubectl),
            Self::Features => kubectl.features().await,
            Self::Info => kubectl.info().await,
            Self::Node(node) => node.exec(kubectl).await,
            Self::Version => kubectl.version().await,
        }
    }
}
/// Basic Commands (Beginner)
#[derive(Clone, Debug, Subcommand)]
// #[command(subcommand_help_heading = "Basic Commands (Beginner)")]
pub enum Basic {
    /// Create a resource from a file or from stdin
    Create(Create),

    /// Take a replication controller, service, deployment or pod and expose it as a new Kubernetes service
    Expose,

    /// Run a particular image on the cluster
    Run,

    /// Set specific features on objects
    Set,
}

impl Basic {
    async fn exec(self, kubectl: &Kubectl) -> kube::Result<()> {
        let _client = kubectl.client()?;
        println!("Basic::exec: {self:?}");
        match self {
            Self::Create(create) => create.exec(kubectl).await,
            Self::Expose => Ok(()),
            Self::Run => Ok(()),
            Self::Set => Ok(()),
        }
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
    async fn exec(self, kubectl: &Kubectl) -> kube::Result<()> {
        let _client = kubectl.client()?;
        match self {
            Self::Explain => Ok(()),
            Self::Get(get) => get.exec(kubectl).await,
            Self::Edit => Ok(()),
            Self::Delete(delete) => delete.exec(kubectl).await,
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
    async fn exec(self, kubectl: &Kubectl) -> kube::Result<()> {
        let _client = kubectl.client()?;
        println!("{self:?}");
        match self {
            Self::Rollout => Ok(()),
            Self::Scale => Ok(()),
            Self::Autoscale => Ok(()),
        }
    }
}

/// Cluster Management Commands
#[derive(Clone, Debug, Subcommand)]
pub enum ClusterManagement {
    /// Modify certificate resources
    Certificate,

    /// Display cluster information
    ClusterInfo,

    /// Display resource (CPU/memory) usage
    Top,

    /// Mark node as unschedulable
    Cordon,

    /// Mark node as schedulable
    Uncordon,

    /// Drain node in preparation for maintenance
    Drain,

    /// Update the taints on one or more nodes
    Taint,
}

impl ClusterManagement {
    async fn exec(self, kubectl: &Kubectl) -> kube::Result<()> {
        let _client = kubectl.client()?;
        println!("{self:?} not implemented yet");
        match self {
            Self::Certificate => Ok(()),
            Self::ClusterInfo => Ok(()),
            Self::Top => Ok(()),
            Self::Cordon => Ok(()),
            Self::Uncordon => Ok(()),
            Self::Drain => Ok(()),
            Self::Taint => Ok(()),
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
