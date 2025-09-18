use super::*;

impl Kubeapi {
    pub async fn info(&self) -> kube::Result<k8s::openapi::apimachinery::pkg::version::Info> {
        self.client()?.apiserver_version().await
    }
}
