//! This crate provides a higher-level API for interacting with Kubernetes clusters.
//! It builds on top of the `kube` crate and adds features like caching, namespace
//! management, and easier access to common Kubernetes resources.

use std::collections::BTreeSet;
use std::fmt;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::time;

use k8s_openapi_ext as k8s;
use kube::api;
use kube::discovery;
use kube_client_ext::KubeClientExt;
use serde_json as json;
use serde_yaml as yaml;
use tracing::debug;
use tracing::error;
use tracing::trace;

use k8s::authenticationv1;
use k8s::authorizationv1;
use k8s::corev1;
use k8s::metav1;
use k8s::rbacv1;

use rkubectl_features::Feature;

pub use cache::Cache;
pub use cascade::Cascade;
pub use dryrun::DryRun;
pub use namespace::Namespace;
pub use options::KubeConfigOptions;
pub use options::KubeapiOptions;

mod cache;
mod cascade;
mod dryrun;
mod features;
mod info;
mod kubeconfig;
mod namespace;
mod options;
mod raw;
mod server;
mod version;

/// Kubeapi is a higher-level Kubernetes API client that provides additional features
/// such as caching, namespace management, and easier access to common Kubernetes resources.
#[derive(Debug)]
pub struct Kubeapi {
    config: kube::Config,
    kubeconfig: kube::config::Kubeconfig,
    cache: Cache,
    namespace: Namespace,
    debug: bool,
    options: KubeapiOptions,
}

impl Kubeapi {
    pub async fn new(
        config: &KubeConfigOptions,
        options: &KubeapiOptions,
        debug: bool,
    ) -> kube::Result<Self> {
        let config_options = config.kube_config_options();
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

    pub fn cluster_url(&self) -> String {
        self.config.cluster_url.to_string()
    }

    pub fn debug(&self, item: impl fmt::Debug) {
        if self.debug {
            println!("{item:?}")
        }
    }

    /// Create a kube::Client from the current configuration.
    pub fn client(&self) -> kube::Result<kube::Client> {
        kube::Client::try_from(self.config.clone())
    }

    /// Returns the path to the cache file based on the current kubeconfig context.
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

    /// Set the namespace for the Kubeapi instance.
    /// This method returns a new instance with the updated namespace.
    pub fn with_namespace(self, namespace: Namespace) -> Self {
        Self { namespace, ..self }
    }

    /// Get the current namespace of the Kubeapi instance.
    pub fn namespace(&self) -> &Namespace {
        &self.namespace
    }

    pub fn show_namespace(&self) -> bool {
        matches!(self.namespace, Namespace::All)
    }

    pub fn cached_server_api_resources(&self) -> Vec<metav1::APIResourceList> {
        self.cache.api_resources().unwrap_or_default()
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

    pub async fn api_versions(&self) -> kube::Result<()> {
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
        scope: discovery::Scope,
        dyntype: &discovery::ApiResource,
    ) -> kube::Result<api::Api<api::DynamicObject>> {
        trace!(?scope, ?dyntype, "dynamic_object_api");
        let client = self.client()?;
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

    // pub fn dynamic_object_api0(
    //     &self,
    //     resource: &Resource,
    // ) -> kube::Result<api::Api<api::DynamicObject>> {
    //     let client = self.client()?;
    //     let (scope, ref dyntype) = resource.api_resource();

    //     trace!(?dyntype, "dynamic_object_api");

    //     let dynamic_api = match scope {
    //         discovery::Scope::Cluster => api::Api::all_with(client, dyntype),
    //         discovery::Scope::Namespaced => match &self.namespace {
    //             Namespace::All => api::Api::all_with(client, dyntype),
    //             Namespace::Default => api::Api::default_namespaced_with(client, dyntype),
    //             Namespace::Namespace(ns) => api::Api::namespaced_with(client, ns, dyntype),
    //         },
    //     };

    //     Ok(dynamic_api)
    // }

    // pub async fn get(&self, resource: Vec<Resource>, output: OutputFormat) -> kube::Result<()> {
    //     println!("Getting {resource:?} [{output:?}]");
    //     Ok(())
    // }

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
        K: kube::Resource + kube::ResourceExt,
        <K as kube::Resource>::DynamicType: Default,
    {
        let kind = K::kind(&default()).to_lowercase();
        let name = k.name_any();
        format!("{kind}/{name}")
    }
}

impl Kubeapi {
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

fn default<T: Default>() -> T {
    T::default()
}
