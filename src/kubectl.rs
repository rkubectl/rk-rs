use futures_util::stream::{self, StreamExt, TryStreamExt};

use super::*;

pub use features::Feature;

mod features;
mod info;
mod kubeconfig;
mod version;

#[derive(Debug)]
pub struct Kubectl {
    config: kube::Config,
    kubeconfig: kube::config::Kubeconfig,
    namespace: Namespace,
    output: OutputFormat,
    debug: bool,
}

impl Kubectl {
    pub fn client(&self) -> kube::Result<kube::Client> {
        kube::Client::try_from(self.config.clone())
    }

    pub async fn new(
        context: Option<&str>,
        cluster: Option<&str>,
        user: Option<&str>,
        debug: bool,
    ) -> kube::Result<Self> {
        let namespace = default();
        let output = default();
        Self::kubeconfig(context, cluster, user, debug)
            .await
            .inspect_err(|err| tracing::error!(%err, "from_kubeconfig"))
            .map_err(|_| kube::Error::LinesCodecMaxLineLengthExceeded)
            .map(|(config, kubeconfig)| Self {
                config,
                kubeconfig,
                namespace,
                output,
                debug,
            })
    }

    pub fn with_namespace(self, namespace: Namespace) -> Self {
        Self { namespace, ..self }
    }

    pub fn with_output(self, output: OutputFormat) -> Self {
        Self { output, ..self }
    }

    pub fn namespace(&self) -> &Namespace {
        &self.namespace
    }

    pub fn output(&self) -> &OutputFormat {
        &self.output
    }

    pub fn show_namespace(&self) -> bool {
        matches!(self.namespace, Namespace::All)
    }

    pub async fn get_core_api_resources(&self) -> kube::Result<Vec<metav1::APIResourceList>> {
        let client = self.client()?;
        let versions = client.list_core_api_versions().await?.versions;
        stream::iter(&versions)
            .then(|version| client.list_core_api_resources(version))
            .try_collect()
            .await
    }

    pub async fn get_api_resources(&self) -> kube::Result<Vec<metav1::APIResourceList>> {
        let client = self.client()?;
        let groups = client.list_api_groups().await?.groups;
        let apiversions = groups.iter().filter_map(|group| {
            group
                .preferred_version
                .as_ref()
                .or_else(|| group.versions.first())
        });
        stream::iter(apiversions)
            .then(|apiversion| client.list_api_group_resources(&apiversion.group_version))
            .try_collect()
            .await
    }

    pub async fn api_versions(&self) -> kube::Result<()> {
        let client = self.client()?;
        let core = client.list_core_api_versions().await?;
        let groups = client.list_api_groups().await?;
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

    pub fn dynamic_api(&self, resource: &api::ApiResource) -> api::Api<api::DynamicObject> {
        println!("{resource:?}");
        let client = self.client().unwrap();
        match &self.namespace {
            Namespace::All => api::Api::all_with(client, resource),
            Namespace::Default => api::Api::default_namespaced_with(client, resource),
            Namespace::Namespace(ns) => api::Api::namespaced_with(client, ns, resource),
        }
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

    pub fn pods(&self) -> kube::Result<api::Api<corev1::Pod>> {
        self.namespaced_api()
    }

    pub fn configmaps(&self) -> kube::Result<api::Api<corev1::ConfigMap>> {
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
}

// impl std::fmt::Debug for Kubectl {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("Kubectl")
//             .field("client", &"kube::Client")
//             .field("namespace", &self.namespace)
//             .field("debug", &self.debug)
//             .finish()
//     }
// }
