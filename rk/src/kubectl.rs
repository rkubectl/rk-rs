use std::collections::BTreeSet;

use futures_util::stream::{self, StreamExt, TryStreamExt};

use super::*;

pub use cache::Cache;

mod cache;
mod features;
mod info;
mod kubeconfig;
mod version;

#[derive(Debug)]
pub struct Kubectl {
    config: kube::Config,
    kubeconfig: kube::config::Kubeconfig,
    cache: Cache,
    namespace: Namespace,
    debug: bool,
    options: GlobalOptions,
}

impl Kubectl {
    pub async fn new(
        config_options: kube::config::KubeConfigOptions,
        debug: bool,
        options: &GlobalOptions,
    ) -> kube::Result<Self> {
        let options = options.clone();
        let namespace = default();
        let cache = cache::Cache::default();
        Self::kubeconfig(config_options, debug)
            .await
            .inspect_err(|err| error!(%err, "from_kubeconfig"))
            .map(|(config, kubeconfig)| Self {
                config,
                kubeconfig,
                cache,
                namespace,
                debug,
                options,
            })
            .and_then(Self::try_load_cache)
            .map_err(|_| kube::Error::LinesCodecMaxLineLengthExceeded)
    }

    pub fn debug(&self, item: impl fmt::Debug) {
        if self.debug {
            println!("{item:?}")
        }
    }

    pub fn client(&self) -> kube::Result<kube::Client> {
        kube::Client::try_from(self.config.clone())
    }

    fn cache_path(&self) -> Result<PathBuf, kube::config::KubeconfigError> {
        self.options.discovery_cache_for_config(&self.config)
    }

    fn try_load_cache(self) -> Result<Self, kube::config::KubeconfigError> {
        let path = self.cache_path()?;
        let cache = self.cache.try_load(path);
        if self.debug {
            println!("Loading cache took {:?}", cache.took());
        }
        Ok(Self { cache, ..self })
    }

    pub fn with_namespace(self, namespace: Namespace) -> Self {
        Self { namespace, ..self }
    }

    pub fn namespace(&self) -> &Namespace {
        &self.namespace
    }

    pub fn show_namespace(&self) -> bool {
        matches!(self.namespace, Namespace::All)
    }

    pub fn cached_server_api_resources(&self) -> Vec<metav1::APIResourceList> {
        self.cache.api_resources().unwrap_or_default()
    }

    pub async fn server_api_resources(&self) -> kube::Result<Vec<metav1::APIResourceList>> {
        if let Some(resources) = self.cache.api_resources() {
            // resources.sort_by_key(|arl| arl.resources[0].group.as_deref());
            Ok(resources)
        } else {
            self.get_server_api_resources().await
        }
    }

    pub async fn server_api_groups(&self) -> kube::Result<metav1::APIGroupList> {
        if let Some(groups) = self.cache.api_groups() {
            Ok(groups)
        } else {
            self.get_server_api_groups().await
        }
    }

    pub async fn server_preferred_resources(&self) -> kube::Result<Vec<metav1::APIResourceList>> {
        let ag = self.server_api_groups().await?;
        let preferred_versions = ag
            .groups
            .into_iter()
            .map(|mut group| {
                group
                    .preferred_version
                    .unwrap_or_else(|| group.versions.remove(0))
            })
            .map(|gv| gv.group_version)
            .collect::<BTreeSet<_>>();
        let resources = self
            .server_api_resources()
            .await?
            .into_iter()
            .filter(|arl| preferred_versions.contains(&arl.group_version))
            .collect();
        Ok(resources)
    }

    async fn get_server_api_groups(&self) -> kube::Result<metav1::APIGroupList> {
        let client = self.client()?;
        let core = client.list_core_api_versions().await?;
        let name = kube::discovery::ApiGroup::CORE_GROUP.to_string();

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

        let mut groups = client.list_api_groups().await?;
        groups.groups.insert(0, core);
        Ok(groups)
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

    pub async fn api_versions(&self, _output: &OutputFormat) -> kube::Result<()> {
        self.server_api_groups()
            .await?
            .groups
            .iter()
            .flat_map(|group| group.versions.iter())
            .for_each(|version| println!("{}", version.group_version));
        Ok(())
    }

    pub fn dynamic_object_api(
        &self,
        resource: &Resource,
    ) -> kube::Result<api::Api<api::DynamicObject>> {
        let client = self.client()?;
        let (scope, ref dyntype) = resource.api_resource();

        trace!(?dyntype, "dynamic_object_api");

        let dynamic_api = match scope {
            discovery::Scope::Cluster => api::Api::all_with(client, dyntype),
            discovery::Scope::Namespaced => match &self.namespace {
                Namespace::All => api::Api::all_with(client, dyntype),
                Namespace::Default => api::Api::default_namespaced_with(client, dyntype),
                Namespace::Namespace(ns) => api::Api::namespaced_with(client, ns, dyntype),
            },
        };

        Ok(dynamic_api)
    }

    pub async fn raw(&self, name: &str) -> kube::Result<String> {
        let gp = self.get_params();
        let request = api::Request::new("")
            .get(name, &gp)
            .map_err(kube::Error::BuildRequest)?;
        self.client()?.request_text(request).await
    }

    pub async fn get(&self, resource: Vec<Resource>, output: OutputFormat) -> kube::Result<()> {
        println!("Getting {resource:?} [{output:?}]");
        Ok(())
    }

    pub fn get_params(&self) -> api::GetParams {
        api::GetParams::default()
    }

    pub fn list_params(&self) -> api::ListParams {
        api::ListParams::default()
    }

    pub fn post_params(&self) -> api::PostParams {
        api::PostParams::default()
    }

    pub fn delete_params(&self, cascade: Cascade, dry_run: DryRun) -> api::DeleteParams {
        let dp = match cascade {
            Cascade::Background => api::DeleteParams::background(),
            Cascade::Foreground => api::DeleteParams::foreground(),
            Cascade::Orphan => api::DeleteParams::orphan(),
        };

        match dry_run {
            DryRun::Server => dp.dry_run(),
            DryRun::None | DryRun::Client => dp,
        }
    }

    pub fn post_params_with_manager(&self, manager: &str) -> api::PostParams {
        api::PostParams {
            field_manager: Some(manager.to_string()),
            ..default()
        }
    }

    pub fn clusterroles(&self) -> kube::Result<api::Api<rbacv1::ClusterRole>> {
        self.cluster_api()
    }

    pub fn namespaces(&self) -> kube::Result<api::Api<corev1::Namespace>> {
        self.cluster_api()
    }

    pub fn pods(&self) -> kube::Result<api::Api<corev1::Pod>> {
        self.namespaced_api()
    }

    pub fn configmaps(&self) -> kube::Result<api::Api<corev1::ConfigMap>> {
        self.namespaced_api()
    }

    pub fn secrets(&self) -> kube::Result<api::Api<corev1::Secret>> {
        self.namespaced_api()
    }

    pub fn componentstatuses(&self) -> kube::Result<api::Api<corev1::ComponentStatus>> {
        self.cluster_api()
    }

    pub fn nodes(&self) -> kube::Result<api::Api<corev1::Node>> {
        self.cluster_api()
    }

    pub fn selfsubjectaccessreviews(
        &self,
    ) -> kube::Result<api::Api<authorizationv1::SelfSubjectAccessReview>> {
        self.cluster_api()
    }

    pub fn selfsubjectrulesreviews(
        &self,
    ) -> kube::Result<api::Api<authorizationv1::SelfSubjectRulesReview>> {
        self.cluster_api()
    }

    pub fn selfsubjectreviews(
        &self,
    ) -> kube::Result<api::Api<authenticationv1::SelfSubjectReview>> {
        self.cluster_api()
    }

    pub fn inspect<K>(&self, k: &K)
    where
        K: serde::Serialize,
    {
        if self.debug {
            let k = yaml::to_string(k).unwrap_or_default();
            println!("{k}");
        }
    }

    pub fn inspect_err(&self, err: &kube::Error) {
        if self.debug {
            println!("{err:?}");
        }
    }

    fn cluster_api<K>(&self) -> kube::Result<api::Api<K>>
    where
        K: kube::Resource<Scope = k8s::openapi::ClusterResourceScope>,
        <K as kube::Resource>::DynamicType: Default,
    {
        self.client().map(|client| client.api())
    }

    fn namespaced_api<K>(&self) -> kube::Result<api::Api<K>>
    where
        K: kube::Resource<Scope = k8s::openapi::NamespaceResourceScope>,
        <K as kube::Resource>::DynamicType: Default,
    {
        let client = self.client()?;
        let api = match &self.namespace {
            Namespace::All => client.api(),
            Namespace::Default => client.default_namespaced_api(),
            Namespace::Namespace(namespace) => client.namespaced_api(namespace),
        };
        Ok(api)
    }

    pub fn full_name<K>(&self, k: &K) -> String
    where
        K: kube::Resource,
        <K as kube::Resource>::DynamicType: Default,
    {
        let kind = K::kind(&default()).to_lowercase();
        let name = k.name_any();
        format!("{kind}/{name}")
    }
}

#[cfg(test)]
impl Kubectl {
    pub fn local() -> Self {
        let config = kube::Config::new("http://localhost:6443".parse().unwrap());
        Self {
            config,
            kubeconfig: default(),
            cache: default(),
            namespace: default(),
            debug: default(),
            options: default(),
        }
    }
}
