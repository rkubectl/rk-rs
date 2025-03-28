use std::str;

// use kube::Resource;
// use kube::ResourceExt;
use kube::api;
// use kube::discovery;
// use kube::discovery::verbs;
use k8s_openapi_ext as k8s;
// use tokio::time;

use k8s::metav1;

pub use cli::Cli;
pub use kubectl::Kubectl;
pub use resource::Resource;

mod cli;
mod kubectl;
mod resource;
