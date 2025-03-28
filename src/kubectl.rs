use super::*;

pub struct Kubectl {
    client: kube::Client,
    debug: bool,
}

impl Kubectl {
    pub async fn new(debug: bool) -> kube::Result<Self> {
        kube::Client::try_default()
            .await
            .map(|client| Self { client, debug })
    }

    pub async fn get_core_api_resources(&self) -> kube::Result<Vec<metav1::APIResourceList>> {
        let versions = self.client.list_core_api_versions().await?.versions;
        let mut list = Vec::with_capacity(versions.len());
        for version in versions {
            let arl = self.client.list_core_api_resources(&version).await?;
            list.push(arl)
        }

        Ok(list)
    }

    pub async fn get_api_resources(&self) -> kube::Result<Vec<metav1::APIResourceList>> {
        let groups = self.client.list_api_groups().await?.groups;
        let mut list = Vec::new();
        for group in groups {
            let apiversion = group
                .preferred_version
                .as_ref()
                .or_else(|| group.versions.first());
            if let Some(apiversion) = apiversion {
                let arl = self
                    .client
                    .list_api_group_resources(&apiversion.group_version)
                    .await?;
                list.push(arl);
            } else {
                continue;
            }
        }

        Ok(list)
    }

    pub async fn api_versions(&self) -> kube::Result<()> {
        let core = self.list_core_api_versions().await?;
        let groups = self.list_api_groups().await?;
        core.versions
            .into_iter()
            .for_each(|version| println!("{version}"));
        groups
            .groups
            .iter()
            .flat_map(|group| group.versions.iter())
            .for_each(|version| println!("{}", version.group_version));
        Ok(())
    }

    pub fn dynamic_api(&self, resource: api::ApiResource) -> api::Api<api::DynamicObject> {
        api::Api::all_with(self.client.clone(), &resource)
    }

    pub async fn get(&self, resource: Vec<Resource>, output: cli::Output) -> kube::Result<()> {
        println!("Getting {resource:?} [{output:?}]");
        Ok(())
    }

    pub fn list_params(&self) -> api::ListParams {
        self.client.list_params()
    }
}

impl std::fmt::Debug for Kubectl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Kubectl")
            .field("client", &"kube::Client")
            .field("debug", &self.debug)
            .finish()
    }
}

impl std::ops::Deref for Kubectl {
    type Target = kube::Client;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}
