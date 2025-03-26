use tabled::Tabled;
use tabled::settings::Remove;
use tabled::settings::Style;
use tabled::settings::location::ByColumnName;

use super::*;

use api_resource::ApiResources;

mod api_resource;

#[derive(Clone, Debug, Subcommand)]
pub enum Command {
    /// Print the supported API resources on the server.
    ApiResources(ApiResources),

    /// Print the supported API versions on the server, in the form of "group/version".
    ApiVersions,

    /// Display one or many resources
    Get {
        #[command(subcommand)]
        resource: Resource,
    },
}
