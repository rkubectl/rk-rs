use k8s_openapi_ext as k8s;
use kube::api;
use kube::core::gvk;
use kube::discovery;

use k8s::ContainerGetExt;
use k8s::ContainerStatusGetExt;
use k8s::NamespaceGetExt;
use k8s::PodGetExt;

use k8s::corev1;
use k8s::metav1;

pub use apiresource::APIResourceExt;
pub use apiresource::APIResourceListExt;
pub use namespace::NamespaceGetExt2;
pub use pod::PodGetExt2;

mod apiresource;
mod namespace;
mod pod;
