use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct NamedResource {
    resource: Resource,
    name: String,
}

impl NamedResource {
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

    pub async fn delete(&self, kubectl: &Kubectl, dp: &api::DeleteParams) -> kube::Result<()> {
        let resource = self.resource.api_resource();
        kubectl
            .dynamic_api(&resource)
            .delete(&self.name, dp)
            .await
            .inspect_err(|err| eprintln!("{err:?}"))?
            .either(
                |object| println!("{object:?}"),
                |status| println!("{status:?}"),
            );
        Ok(())
    }
}
