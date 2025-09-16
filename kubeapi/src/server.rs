use futures_util::stream;
use futures_util::stream::StreamExt;
use futures_util::stream::TryStreamExt;

use super::*;

impl Kubeapi {
    pub async fn server_api_groups(&self) -> kube::Result<metav1::APIGroupList> {
        if let Some(groups) = self.cache.api_groups() {
            Ok(groups)
        } else {
            self.get_server_api_groups().await
        }
    }

    pub async fn server_api_resources(&self) -> kube::Result<Vec<metav1::APIResourceList>> {
        if let Some(resources) = self.cache.api_resources() {
            // resources.sort_by_key(|arl| arl.resources[0].group.as_deref());
            Ok(resources)
        } else {
            self.get_server_api_resources().await
        }
    }

    async fn get_server_api_resources(&self) -> kube::Result<Vec<metav1::APIResourceList>> {
        let client = self.client()?;
        let core = client.list_core_api_versions().await?;
        let core = stream::iter(&core.versions)
            .then(|version| client.list_core_api_resources(version))
            .try_collect::<Vec<_>>()
            .await?;

        let groups = client.list_api_groups().await?.groups;
        let apiversions = groups.iter().filter_map(|group| {
            group
                .preferred_version
                .as_ref()
                .or_else(|| group.versions.first())
        });
        let groups = stream::iter(apiversions)
            .then(|apiversion| client.list_api_group_resources(&apiversion.group_version))
            .try_collect::<Vec<_>>()
            .await?;

        Ok(core.into_iter().chain(groups).collect())
    }

    async fn get_server_api_groups(&self) -> kube::Result<metav1::APIGroupList> {
        let client = self.client()?;
        let core = self.get_server_core_api_group().await?;
        let mut groups = client.list_api_groups().await?;
        groups.groups.insert(0, core);
        Ok(groups)
    }

    async fn get_server_core_api_group(&self) -> kube::Result<metav1::APIGroup> {
        let client = self.client()?;
        let name = kube::discovery::ApiGroup::CORE_GROUP.to_string();
        let core = client.list_core_api_versions().await?;

        let versions = core
            .versions
            .into_iter()
            .map(|version| metav1::GroupVersionForDiscovery {
                group_version: format!("{name}{version}"),
                version,
            })
            .collect::<Vec<_>>();

        let core = metav1::APIGroup {
            name,
            preferred_version: Some(versions[0].clone()),
            server_address_by_client_cidrs: Some(core.server_address_by_client_cidrs),
            versions,
        };

        Ok(core)
    }
}
