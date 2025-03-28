use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct NamedResource {
    resource: Resource,
    name: String,
}

impl NamedResource {
    pub fn new(resource: impl ToString, name: impl ToString) -> Self {
        let resource = resource.to_string().into();
        Self::with_resource(resource, name)
    }

    pub fn with_resource(resource: Resource, name: impl ToString) -> Self {
        let name = name.to_string();
        Self { resource, name }
    }

    pub fn resource(&self) -> &Resource {
        &self.resource
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
