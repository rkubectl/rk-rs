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
    pub async fn version(&self) -> kube::Result<()> {
        let info = self.client()?.apiserver_version().await?;

        println!("Client k8s version: {K8S_VERSION}");
        println!("Server k8s version: {}.{}", info.major, info.minor);
        println!("Server git version: {}", info.git_version);
        println!("Server git commit:  {}", info.git_commit);

        Ok(())
    }
}
