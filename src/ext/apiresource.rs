use super::*;

pub trait APIResourceListExt: Sized {
    fn group_version(&self) -> Result<gvk::GroupVersion, gvk::ParseGroupVersionError>;
    fn find(self, name: &str) -> Option<metav1::APIResource>;
    fn kube_api_resource(self, name: &str) -> Option<api::ApiResource> {
        let gv = self.group_version().ok()?;
        self.find(name).map(|ar| ar.kube_api_resource(gv))
    }
}

impl APIResourceListExt for metav1::APIResourceList {
    fn group_version(&self) -> Result<gvk::GroupVersion, gvk::ParseGroupVersionError> {
        self.group_version.parse()
    }

    fn find(self, name: &str) -> Option<metav1::APIResource> {
        self.resources.into_iter().find(|ar| ar.matches_name(name))
    }
}

pub trait APIResourceExt {
    fn matches_name(&self, name: &str) -> bool;
    fn kube_api_resource(self, gv: gvk::GroupVersion) -> api::ApiResource;
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
                .any(|text| text == name)
    }

    fn kube_api_resource(self, gv: gvk::GroupVersion) -> api::ApiResource {
        let api_version = gv.api_version();
        api::ApiResource {
            group: self.group.unwrap_or(gv.group),
            version: self.version.unwrap_or(gv.version),
            api_version,
            kind: self.kind,
            plural: self.name,
        }
    }
}
