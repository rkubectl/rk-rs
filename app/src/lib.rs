use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;
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

use rkubectl_kubeapi::Cascade;
use rkubectl_kubeapi::ConfigOptions;
use rkubectl_kubeapi::DryRun;
use rkubectl_kubeapi::GlobalKubeapiOptions;
use rkubectl_kubeapi::Kubeapi;
use rkubectl_kubeapi::Namespace;
use rkubectl_resource::InvalidResourceSpec;
use rkubectl_resource::ResourceArg;
use rkubectl_ui::OutputFormat;
use rkubectl_ui::Show;
use rkubectl_ui::ShowParams;
use rkubectl_ui::Ui;

pub use cli::*;
pub use error::RkError;

pub type Result<T, E = RkError> = ::std::result::Result<T, E>;
pub type RkResult<T> = self::Result<T>;

mod cli;
mod error;

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
