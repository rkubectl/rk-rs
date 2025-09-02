use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::str;

use kube::ResourceExt;
use kube::api;
use kube::discovery;
// use kube::discovery::verbs;
use k8s_openapi_ext as k8s;
// use tokio::time;
use kube_client_ext::KubeClientExt;
use serde_json as json;
use serde_yaml as yaml;
use tracing::debug;
use tracing::error;
use tracing::info;
use tracing::trace;

use k8s::authenticationv1;
use k8s::authorizationv1;
use k8s::corev1;
use k8s::metav1;
use k8s::rbacv1;

// use rk_ext::APIResourceExt;
use rk_ext::APIResourceListExt;
use rk_features::Feature;
use rk_ui::OutputFormat;
use rk_ui::Show;
use rk_ui::ShowParams;
use rk_ui::Ui;

pub use kubectl::Cache;
pub use kubectl::Kubectl;
pub use namespace::Namespace;
pub use resource::NamedResource;
pub use resource::Resource;
pub use resource::ResourceArg;

pub use cli::*;

mod cli;
mod kubectl;
mod namespace;
mod resource;

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
