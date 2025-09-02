use super::*;

k8s_openapi::k8s_if_1_30! {
    const K8S_VERSION: &str = "1.30";
}
k8s_openapi::k8s_if_1_31! {
    const K8S_VERSION: &str = "1.31";
}
k8s_openapi::k8s_if_1_32! {
    const K8S_VERSION: &str = "1.32";
}
k8s_openapi::k8s_if_1_33! {
    const K8S_VERSION: &str = "1.33";
}
// k8s_openapi::k8s_if_1_34! {
//     const K8S_VERSION: &str = "1.34";
// }

impl Kubectl {
    pub async fn version(&self) -> kube::Result<String> {
        let info = self.client()?.apiserver_version().await?;
        let text = [
            format!("Client k8s version: {K8S_VERSION}"),
            format!("Server k8s version: {}.{}", info.major, info.minor),
            format!("Server git version: {}", info.git_version),
            format!("Server git commit:  {}", info.git_commit),
        ]
        .join("\n");

        Ok(text)
    }
}
