use k8s_openapi_ext as k8s;
use kube::api;
use kube::core::gvk;
use kube::discovery;

// use kube_client_ext::KubeClientExt;
// use serde_json as json;
// use serde_yaml as yaml;
// use tracing::debug;
// use tracing::error;
// use tracing::info;
// use tracing::trace;

// use k8s::authenticationv1;
// use k8s::authorizationv1;
// use k8s::corev1;
use k8s::metav1;
// use k8s::rbacv1;

pub use apiresource::APIResourceExt;
pub use apiresource::APIResourceListExt;

mod apiresource;
