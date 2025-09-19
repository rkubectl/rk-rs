use super::*;

pub use create::Create;
pub use create::CreateResource;

mod create;

#[expect(clippy::large_enum_variant)]
/// Basic Commands (Beginner)
#[derive(Clone, Debug, Subcommand)]
#[command(subcommand_help_heading = "Basic Commands (Beginner)")]
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
    pub async fn exec(self, context: &Context) -> RkResult<()> {
        match self {
            Self::Create(create) => create.exec(context).await,
            Self::Expose => Err(RkError::todo()),
            Self::Run => Err(RkError::todo()),
            Self::Set => Err(RkError::todo()),
        }
    }
}
