use tabled::settings::Remove;
use tabled::settings::Style;
use tabled::settings::location::ByColumnName;

use super::*;

pub use api_resource::ApiResource;
pub use api_resource::ApiResources;
pub use auth::Auth;
pub use get::Get;

mod api_resource;
mod auth;
mod get;

#[derive(Clone, Debug, Subcommand)]
pub enum Command {
    /// Print the supported API resources on the server.
    ApiResources(ApiResources),

    /// Inspect authorization.
    #[command(subcommand)]
    Auth(Auth),

    /// Print the supported API versions on the server, in the form of "group/version".
    ApiVersions,

    Get(Get),

    /// Print status of the K8s feaures in the current cluster.
    Features,

    /// Print brief server info
    Info,

    /// Print client and server version
    Version,
}
