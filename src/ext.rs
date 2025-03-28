use super::*;

pub trait APIResourceListExt {
    fn group_version(&self) -> Result<gvk::GroupVersion, gvk::ParseGroupVersionError>;
    fn find(&self, name: &str, gv: &gvk::GroupVersion) -> Option<api::ApiResource>;
}

impl APIResourceListExt for metav1::APIResourceList {
    fn group_version(&self) -> Result<gvk::GroupVersion, gvk::ParseGroupVersionError> {
        self.group_version.parse()
    }

    fn find(&self, name: &str, gv: &gvk::GroupVersion) -> Option<api::ApiResource> {
        self.resources
            .iter()
            .find(|ar| ar.matches_name(name))
            .map(|ar| ar.kube_api_resource(gv))
    }
}

pub trait APIResourceExt {
    fn matches_name(&self, name: &str) -> bool;
    fn kube_api_resource(&self, gv: &gvk::GroupVersion) -> api::ApiResource;
}

impl APIResourceExt for metav1::APIResource {
    fn matches_name(&self, name: &str) -> bool {
        self.name == name
            || self.singular_name == name
            || self
                .short_names
                .as_deref()
                .unwrap_or_default()
                .iter()
                .any(|n| n == name)
    }

    fn kube_api_resource(&self, gv: &gvk::GroupVersion) -> api::ApiResource {
        api::ApiResource {
            group: self.group.clone().unwrap_or_else(|| gv.group.clone()),
            version: self.version.clone().unwrap_or_else(|| gv.version.clone()),
            api_version: gv.api_version(),
            kind: self.kind.to_string(),
            plural: self.name.clone(),
        }
    }
}
