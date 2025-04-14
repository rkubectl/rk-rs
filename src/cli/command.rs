use tabled::settings::Remove;
use tabled::settings::Style;
use tabled::settings::location::ByColumnName;

use super::*;

pub use api_resource::ApiResource;
pub use api_resource::ApiResources;
pub use auth::Auth;
pub use config::Config;
pub use get::Get;
pub use node::Node;

mod api_resource;
mod auth;
mod config;
mod get;
mod node;

#[derive(Clone, Debug, Subcommand)]
pub enum Command {
    /// Print the supported API resources on the server.
    ApiResources(ApiResources),

    /// Inspect authorization.
    #[command(subcommand)]
    Auth(Auth),

    /// Print the supported API versions on the server, in the form of "group/version".
    ApiVersions,

    /// Modify kubeconfig file.
    #[command(subcommand)]
    Config(Config),

    Get(Get),

    /// Print status of the K8s feaures in the current cluster.
    Features,

    /// Print brief server info
    Info,

    /// Print Node related info
    #[command(subcommand)]
    Node(Node),

    /// Print client and server version
    Version,
}
