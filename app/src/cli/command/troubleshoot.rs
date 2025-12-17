use super::*;

pub use auth::Auth;
pub use debug::Debug;

mod auth;
mod debug;

/// Troubleshooting and Debugging Commands
#[derive(Clone, Debug, Subcommand)]
pub enum TroubleshootingDebugging {
    /// Show details of a specific resource or group of resources
    Describe,
    /// Print the logs for a container in a pod
    Logs,
    /// Attach to a running container
    Attach,
    /// Execute a command in a container
    Exec,
    /// Forward one or more local ports to a pod
    PortForward,
    /// Run a proxy to the Kubernetes API server
    Proxy,
    /// Copy files and directories to and from containers
    Cp,

    /// Inspect authorization
    #[command(subcommand)]
    Auth(Auth),

    /// Create debugging sessions for troubleshooting workloads and nodes
    Debug,
    /// List events
    Events,
}

impl TroubleshootingDebugging {
    pub async fn exec(self, context: &Context) -> RkResult<()> {
        match self {
            Self::Describe => Err(RkError::todo()),
            Self::Logs => Err(RkError::todo()),
            Self::Attach => Err(RkError::todo()),
            Self::Exec => Err(RkError::todo()),
            Self::PortForward => Err(RkError::todo()),
            Self::Proxy => Err(RkError::todo()),
            Self::Cp => Err(RkError::todo()),
            Self::Auth(auth) => auth.exec(context).await,
            Self::Debug => Err(RkError::todo()),
            Self::Events => Err(RkError::todo()),
        }
    }
}
