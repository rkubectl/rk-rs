#![warn(deprecated)]
#![warn(rust_2018_idioms)]
#![warn(rust_2018_compatibility)]
#![warn(rust_2021_compatibility)]
#![warn(rust_2024_compatibility)]
#![warn(future_incompatible)]
#![warn(clippy::use_self)]
#![warn(clippy::map_unwrap_or)]

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

use k8s::corev1;
use k8s::metav1;

pub use cli::Cli;
pub use ext::APIResourceExt;
pub use ext::APIResourceListExt;
pub use kubectl::Kubectl;
pub use namespace::Namespace;
pub use resource::Resource;
pub use resource::ResourceArg;
pub use show::Output;
pub use show::Show;

mod cli;
mod ext;
mod kubectl;
mod namespace;
mod resource;
mod show;

fn default<T: Default>() -> T {
    T::default()
}
