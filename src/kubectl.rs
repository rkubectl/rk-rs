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

    pub async fn get(&self, resource: cli::Resource, output: cli::Output) -> kube::Result<()> {
        println!("Getting {resource:?} [{output:?}]");
        Ok(())
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
