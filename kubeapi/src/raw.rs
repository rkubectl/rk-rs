use super::*;

impl Kubeapi {
    /// Perform a raw GET request to the Kubernetes API server.
    /// The `name` parameter is the path to the resource, e.g., "metrics".
    pub async fn raw_get(&self, name: &str) -> kube::Result<String> {
        let gp = self.get_params();
        let request = api::Request::new("")
            .get(name, &gp)
            .map_err(kube::Error::BuildRequest)?;
        self.client()?.request_text(request).await
    }
}
