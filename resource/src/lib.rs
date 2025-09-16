use std::fmt;

use k8s_openapi_ext as k8s;
use kube::api;
use kube::discovery;

// use k8s::authenticationv1;
// use k8s::authorizationv1;
use k8s::corev1;
// use k8s::metav1;
// use k8s::rbacv1;

// use rkubectl_ext::APIResourceExt;
use rkubectl_ext::APIResourceListExt;
use rkubectl_kubeapi::Kubeapi;
use rkubectl_ui::Show;

pub use named::NamedResource;

mod named;

#[derive(Clone, Debug, PartialEq)]
pub enum ResourceArg {
    Resource(Resource),
    NamedResource(NamedResource),
}

impl ResourceArg {
    pub fn from_strings(
        resources: &[String],
        kubeapi: &Kubeapi,
    ) -> Result<Vec<Self>, InvalidResourceSpec> {
        // Two possible formats
        // 1. resource/name - in which case all the items should be the same
        // 2. resource[,resource,..] [name] [..]
        if resources.iter().any(|resource| resource.contains('/')) {
            resources
                .iter()
                .map(|text| Self::named_resource(text, kubeapi))
                .collect()
        } else {
            let (resource, names) = resources.split_first().ok_or(InvalidResourceSpec)?;
            let resources = resource
                .split(",")
                .map(|resource| Resource::with_cache(resource, kubeapi).ok_or(InvalidResourceSpec))
                .collect::<Result<Vec<_>, _>>()?;
            let resources = if names.is_empty() {
                // Just resources, no names
                resources.into_iter().map(ResourceArg::Resource).collect()
            } else {
                resources
                    .into_iter()
                    .flat_map(|resource| {
                        names
                            .iter()
                            .map(move |name| NamedResource::with_resource(resource.clone(), name))
                    })
                    .map(Self::NamedResource)
                    .collect()
            };
            Ok(resources)
        }
    }

    fn named_resource(
        text: impl AsRef<str>,
        kubeapi: &Kubeapi,
    ) -> Result<Self, InvalidResourceSpec> {
        let (resource, name) = text.as_ref().split_once("/").ok_or(InvalidResourceSpec)?;
        Resource::with_cache(resource, kubeapi)
            .map(|resource| NamedResource::with_resource(resource, name))
            .map(Self::NamedResource)
            .ok_or(InvalidResourceSpec)
    }

    pub async fn get(&self, kubeapi: &Kubeapi) -> kube::Result<Box<dyn Show>> {
        match self {
            Self::Resource(resource) => resource.list(kubeapi).await,
            Self::NamedResource(named_resource) => {
                named_resource
                    .resource()
                    .get(kubeapi, named_resource.name())
                    .await
            }
        }
    }

    pub async fn delete(
        &self,
        kubeapi: &Kubeapi,
        dp: &api::DeleteParams,
        all: bool,
    ) -> kube::Result<()> {
        match self {
            Self::Resource(resource) if all => {
                todo!("Deleting ALL resources {resource:?} is not implemented yet")
            }
            Self::Resource(resource) => {
                todo!("Deleting SOME resources {resource:?} is not implemented yet")
            }
            Self::NamedResource(resource) => resource.delete(kubeapi, dp).await,
        }
    }

    pub fn resource(&self) -> &Resource {
        match self {
            Self::Resource(resource) => resource,
            Self::NamedResource(named_resource) => named_resource.resource(),
        }
    }

    pub fn name(&self) -> Option<&str> {
        match self {
            Self::Resource(_resource) => None,
            Self::NamedResource(named_resource) => Some(named_resource.name()),
        }
    }
}

impl fmt::Display for ResourceArg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Resource(resource) => resource.fmt(f),
            Self::NamedResource(resource) => resource.fmt(f),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Resource {
    Pods,
    Namespaces,
    Nodes,
    ConfigMaps,
    ComponentStatuses,
    Other {
        scope: discovery::Scope,
        resource: api::ApiResource,
    },
}

impl Resource {
    pub fn with_cache(resource: &str, kubeapi: &Kubeapi) -> Option<Self> {
        Self::well_known(resource).or_else(|| Self::other(resource, kubeapi))
    }

    pub fn well_known(text: &str) -> Option<Self> {
        match text {
            "po" | "pod" | "pods" => Some(Self::Pods),
            "no" | "node" | "nodes" => Some(Self::Nodes),
            "ns" | "namespace" | "namespaces" => Some(Self::Namespaces),
            "cm" | "configmap" | "configmaps" => Some(Self::ConfigMaps),
            "cs" | "componentstatus" | "componentstatuses" => Some(Self::ComponentStatuses),
            _ => None,
        }
    }

    async fn list(&self, kubeapi: &Kubeapi) -> kube::Result<Box<dyn Show>> {
        let lp = kubeapi.list_params();
        match self {
            Self::Pods => {
                let list = kubeapi.pods()?.list(&lp).await?;
                Ok(Box::new(list))
            }
            Self::Namespaces => {
                let list = kubeapi.namespaces()?.list(&lp).await?;
                Ok(Box::new(list))
            }
            Self::Nodes => {
                let list = kubeapi.nodes()?.list(&lp).await?;
                Ok(Box::new(list))
            }
            Self::ConfigMaps => {
                let list = kubeapi.configmaps()?.list(&lp).await?;
                Ok(Box::new(list))
            }
            Self::ComponentStatuses => {
                let list = kubeapi.componentstatuses()?.list(&lp).await?;
                Ok(Box::new(list))
            }
            Self::Other { scope, resource } => {
                todo!("list not implemented yet for {scope:?} {resource:?}")
            }
        }
    }

    async fn get(&self, kubeapi: &Kubeapi, name: &str) -> kube::Result<Box<dyn Show>> {
        match self {
            Self::Pods => {
                let obj = kubeapi.pods()?.get(name).await?;
                Ok(Box::new(obj))
            }
            Self::Namespaces => {
                let obj = kubeapi.namespaces()?.get(name).await?;
                Ok(Box::new(obj))
            }
            Self::Nodes => {
                let obj = kubeapi.nodes()?.get(name).await?;
                Ok(Box::new(obj))
            }
            Self::ConfigMaps => {
                let obj = kubeapi.configmaps()?.get(name).await?;
                Ok(Box::new(obj))
            }
            Self::ComponentStatuses => {
                let obj = kubeapi.componentstatuses()?.get(name).await?;
                Ok(Box::new(obj))
            }
            Self::Other { scope, resource } => {
                todo!("get not implemented yet for {scope:?} {resource:?}")
            }
        }
    }

    // async fn delete(
    //     &self,
    //     kubectl: &Kubectl,
    //     name: &str,
    //     dp: &api::DeleteParams,
    // ) -> kube::Result<()> {
    //     let deleted = |ok| {
    //         ok.map_left(|k| println!("{k:?}"))
    //             .map_right(|status| println!("{status:?}"))
    //     };
    //     let deleted = match self {
    //         Self::Pods => kubectl.pods()?.delete(name, dp).await.map(deleted),
    //         Self::Namespaces => kubectl.namespaces()?.delete(name, dp).await.map(deleted),
    //         Self::Nodes => kubectl.nodes()?.delete(name, dp).await.map(deleted),
    //         Self::ConfigMaps => kubectl.configmaps()?.delete(name, dp).await.map(deleted),
    //         Self::ComponentStatuses => kubectl
    //             .componentstatuses()?
    //             .delete(name, dp)
    //             .await
    //             .map(deleted),
    //         Self::Other(resource) => {
    //             todo!("get not implemented yet for {resource:?}")
    //         }
    //     };

    //     Ok(())
    // }

    pub fn api_resource(&self) -> (discovery::Scope, api::ApiResource) {
        use discovery::Scope::{Cluster, Namespaced};

        match self {
            Self::Pods => (Namespaced, Self::erase::<corev1::Pod>()),
            Self::Namespaces => (Cluster, Self::erase::<corev1::Namespace>()),
            Self::Nodes => (Cluster, Self::erase::<corev1::Node>()),
            Self::ConfigMaps => (Namespaced, Self::erase::<corev1::ConfigMap>()),
            Self::ComponentStatuses => (Namespaced, Self::erase::<corev1::ComponentStatus>()),
            Self::Other { scope, resource } => (scope.clone(), resource.clone()),
        }
    }

    fn cached_dynamic_api_resource(
        kubeapi: &Kubeapi,
        name: &str,
    ) -> Option<(discovery::Scope, api::ApiResource)> {
        kubeapi
            .cached_server_api_resources()
            .into_iter()
            .find_map(|arl| arl.kube_api_resource(name))
    }

    async fn _dynamic_api_resource(
        kubeapi: &Kubeapi,
        name: &str,
    ) -> kube::Result<Option<(discovery::Scope, api::ApiResource)>> {
        let ar = kubeapi
            .server_api_resources()
            .await?
            .into_iter()
            .find_map(|arl| arl.kube_api_resource(name));
        Ok(ar)
    }

    fn erase<K>() -> api::ApiResource
    where
        K: kube::Resource,
        <K as kube::Resource>::DynamicType: Default,
    {
        api::ApiResource::erase::<K>(&<K as kube::Resource>::DynamicType::default())
    }

    fn other(resource: &str, kubeapi: &Kubeapi) -> Option<Self> {
        Self::cached_dynamic_api_resource(kubeapi, resource)
            .map(|(scope, resource)| Self::Other { scope, resource })
    }
}

impl fmt::Display for Resource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pods => "pod".fmt(f),
            Self::Namespaces => "namespace".fmt(f),
            Self::Nodes => "node".fmt(f),
            Self::ConfigMaps => "configmap".fmt(f),
            Self::ComponentStatuses => "componentstatus".fmt(f),
            Self::Other { resource, .. } => resource.kind.to_lowercase().fmt(f),
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error(
    "there is no need to specify a resource type as a separate argument when passing arguments in resource/name form (e.g. 'kubectl get resource/<resource_name>' instead of 'kubectl get resource resource/<resource_name>')"
)]
pub struct InvalidResourceSpec;

#[cfg(test)]
mod tests {
    use super::*;

    fn args(s: &[&str]) -> Result<Vec<ResourceArg>, InvalidResourceSpec> {
        let resources = s.iter().map(ToString::to_string).collect::<Vec<_>>();
        let kubeapi = Kubeapi::local();
        ResourceArg::from_strings(&resources, &kubeapi)
    }

    #[test]
    fn one_resource() {
        let resources = args(&["pod"]).unwrap();
        assert_eq!(resources.len(), 1);
        assert_eq!(resources[0], ResourceArg::Resource(Resource::Pods));
    }

    #[test]
    fn many_resources() {
        let resources = args(&["pod,node"]).unwrap();
        assert_eq!(resources.len(), 2);

        let ResourceArg::Resource(ref pod) = resources[0] else {
            panic!("expecting NamedResource, found something else");
        };
        let ResourceArg::Resource(ref node) = resources[1] else {
            panic!("expecting NamedResource, found something else");
        };

        assert_eq!(pod, &Resource::Pods);
        assert_eq!(node, &Resource::Nodes);
    }

    #[test]
    fn resource_and_name() {
        let resources = args(&["pod", "bazooka"]).unwrap();
        assert_eq!(resources.len(), 1);
        let ResourceArg::NamedResource(ref pod) = resources[0] else {
            panic!("expecting NamedResource, found something else");
        };
        assert_eq!(pod.resource(), &Resource::Pods);
        assert_eq!(pod.name(), "bazooka");
    }

    #[test]
    fn resource_and_many_name() {
        let resources = args(&["pod", "bazooka", "darbooka"]).unwrap();
        assert_eq!(resources.len(), 2);
        let ResourceArg::NamedResource(ref pod1) = resources[0] else {
            panic!("expecting NamedResource, found something else");
        };
        let ResourceArg::NamedResource(ref pod2) = resources[1] else {
            panic!("expecting NamedResource, found something else");
        };
        assert_eq!(pod1.resource(), &Resource::Pods);
        assert_eq!(pod1.name(), "bazooka");
        assert_eq!(pod2.resource(), &Resource::Pods);
        assert_eq!(pod2.name(), "darbooka");
    }

    #[test]
    fn one_named_resource() {
        let resources = args(&["pod/bazooka"]).unwrap();
        assert_eq!(resources.len(), 1);
        let ResourceArg::NamedResource(ref pod) = resources[0] else {
            panic!("expecting NamedResource, found something else");
        };

        assert_eq!(pod.resource(), &Resource::Pods);
        assert_eq!(pod.name(), "bazooka");
    }

    #[test]
    fn many_named_resources() {
        let resources = args(&["pod/bazooka", "node/elephant"]).unwrap();
        assert_eq!(resources.len(), 2);

        let ResourceArg::NamedResource(ref pod) = resources[0] else {
            panic!("expecting NamedResource, found something else");
        };
        let ResourceArg::NamedResource(ref node) = resources[1] else {
            panic!("expecting NamedResource, found something else");
        };

        assert_eq!(pod.resource(), &Resource::Pods);
        assert_eq!(pod.name(), "bazooka");
        assert_eq!(node.resource(), &Resource::Nodes);
        assert_eq!(node.name(), "elephant");
    }

    #[test]
    fn invalid_mix() {
        let _err = args(&["pod/bazooka", "node"]).unwrap_err();
    }
}
