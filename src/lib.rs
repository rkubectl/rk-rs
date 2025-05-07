// #![cfg_attr(feature = "pedantic", warn(clippy::pedantic))]
#![warn(deprecated)]
#![warn(rust_2018_idioms)]
#![warn(rust_2018_compatibility)]
#![warn(rust_2021_compatibility)]
#![warn(rust_2024_compatibility)]
#![warn(future_incompatible)]
#![warn(deprecated_in_future)]
#![warn(noop_method_call)]
#![warn(unreachable_pub)]
#![warn(missing_debug_implementations)]
#![warn(unused)]
#![warn(clippy::use_self)]
#![warn(clippy::map_unwrap_or)]
#![warn(clippy::map_flatten)]
#![deny(warnings)]

use std::path::{Path, PathBuf};
use std::str;

use kube::Resource as _;
use kube::ResourceExt;
use kube::api;
use kube::core::gvk;
// use kube::discovery;
// use kube::discovery::verbs;
use k8s_openapi_ext as k8s;
// use tokio::time;
use kube_client_ext::KubeClientExt;
use serde_json as json;
use serde_yaml as yaml;

use k8s::authenticationv1;
use k8s::authorizationv1;
use k8s::corev1;
use k8s::metav1;

pub use cli::ApiResource;
pub use cli::ApiResources;
pub use cli::Auth;
pub use cli::Cli;
pub use cli::Command;
pub use cli::Config;
pub use cli::Get;
pub use cli::GlobalOptions;
pub use cli::Node;
pub use kubectl::Cache;
pub use kubectl::Feature;
pub use kubectl::Kubectl;
pub use namespace::Namespace;
pub use resource::NamedResource;
pub use resource::Resource;
pub use resource::ResourceArg;
pub use show::OutputFormat;
pub use show::Show;
pub use show::ShowParams;

pub use ext::*;

mod cli;
mod ext;
mod kubectl;
mod namespace;
mod resource;
mod show;

fn default<T: Default>() -> T {
    T::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert();
    }
}
