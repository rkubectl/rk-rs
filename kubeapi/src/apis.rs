use super::*;

/// Common Kubernetes resources
impl Kubeapi {
    /// `corev1::Namespace`
    pub fn namespaces(&self) -> kube::Result<api::Api<corev1::Namespace>> {
        self.cluster_api()
    }

    /// `corev1::Pod`
    pub fn pods(&self) -> kube::Result<api::Api<corev1::Pod>> {
        self.namespaced_api()
    }

    /// `corev1::ConfigMap`
    pub fn configmaps(&self) -> kube::Result<api::Api<corev1::ConfigMap>> {
        self.namespaced_api()
    }

    /// `corev1::Secret`
    pub fn secrets(&self) -> kube::Result<api::Api<corev1::Secret>> {
        self.namespaced_api()
    }

    /// `corev1::ComponentStatus`
    pub fn componentstatuses(&self) -> kube::Result<api::Api<corev1::ComponentStatus>> {
        self.cluster_api()
    }

    /// `corev1::Node`
    pub fn nodes(&self) -> kube::Result<api::Api<corev1::Node>> {
        self.cluster_api()
    }

    /// `rbacv1::ClusterRole`
    pub fn clusterroles(&self) -> kube::Result<api::Api<rbacv1::ClusterRole>> {
        self.cluster_api()
    }

    /// `rbacv1::ClusterRoleBinding`
    pub fn clusterrolebindings(&self) -> kube::Result<api::Api<rbacv1::ClusterRoleBinding>> {
        self.cluster_api()
    }

    /// `authorizationv1::SelfSubjectAccessReview`
    pub fn selfsubjectaccessreviews(
        &self,
    ) -> kube::Result<api::Api<authorizationv1::SelfSubjectAccessReview>> {
        self.cluster_api()
    }

    /// `authorizationv1::SelfSubjectRulesReview`
    pub fn selfsubjectrulesreviews(
        &self,
    ) -> kube::Result<api::Api<authorizationv1::SelfSubjectRulesReview>> {
        self.cluster_api()
    }

    /// `authenticationv1::SelfSubjectReview`
    pub fn selfsubjectreviews(
        &self,
    ) -> kube::Result<api::Api<authenticationv1::SelfSubjectReview>> {
        self.cluster_api()
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
