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

    pub async fn delete(&self, kubeapi: &Kubeapi, dp: &api::DeleteParams) -> kube::Result<()> {
        let (scope, ref dyntype) = self.resource.api_resource();
        kubeapi
            .dynamic_object_api(scope, dyntype)?
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

impl fmt::Display for NamedResource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format_args!("{}/{}", self.resource, self.name).fmt(f)
    }
}
