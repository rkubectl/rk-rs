use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::str;

use k8s_openapi_ext as k8s;
use kube::ResourceExt;
use kube::api;
use kube::discovery;
use tracing::debug;
use tracing::info;
use tracing::trace;

use k8s::authenticationv1;
use k8s::authorizationv1;
use k8s::corev1;
use k8s::metav1;
use k8s::rbacv1;

use rk_kubeapi::Cascade;
use rk_kubeapi::ConfigOptions;
use rk_kubeapi::DryRun;
use rk_kubeapi::GlobalKubeapiOptions;
use rk_kubeapi::Kubeapi;
use rk_kubeapi::Namespace;
use rk_resource::InvalidResourceSpec;
use rk_resource::ResourceArg;
use rk_ui::OutputFormat;
use rk_ui::Show;
use rk_ui::ShowParams;
use rk_ui::Ui;

pub use cli::*;

mod cli;

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
