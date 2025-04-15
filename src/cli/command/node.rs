use super::*;

use image::ImageInfo;

mod image;

#[derive(Clone, Debug, Subcommand)]
pub enum Node {
    Info,
    ListImages,
}

impl Node {
    pub async fn exec(&self, kubectl: &Kubectl) -> kube::Result<()> {
        match self {
            Self::Info => self.info(kubectl).await,
            Self::ListImages => self.list_images(kubectl).await,
        }
    }

    pub async fn info(&self, kubectl: &Kubectl) -> kube::Result<()> {
        for node in self.nodes(kubectl).await? {
            println!("\n{}\n", node.name_any());
            if let Some(info) = node.status {
                println!("  Node Info: {:?}", info.node_info.unwrap_or_default());
            }
        }

        Ok(())
    }

    pub async fn list_images(&self, kubectl: &Kubectl) -> kube::Result<()> {
        for node in self.nodes(kubectl).await? {
            println!("\n{}\n", node.name_any());
            node.status
                .unwrap_or_default()
                .images
                .unwrap_or_default()
                .into_iter()
                .filter_map(ImageInfo::from_container_image)
                .for_each(|info| info.print());
        }

        Ok(())
    }

    async fn nodes(&self, kubectl: &Kubectl) -> kube::Result<Vec<corev1::Node>> {
        let lp = kubectl.list_params();
        kubectl.nodes()?.list(&lp).await.map(|list| list.items)
    }
}
