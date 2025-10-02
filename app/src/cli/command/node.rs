use std::collections::BTreeMap;
use std::collections::BTreeSet;

use super::*;

use image::ImageInfo;
use resources::Resources;

mod image;
mod resources;

#[derive(Clone, Debug, Subcommand)]
pub enum Node {
    /// Show node info.
    Info,
    /// List node images.
    #[command(visible_aliases = ["images", "img"])]
    ListImages,

    /// Show node resources (allocatable by default or capacity).
    #[command(visible_aliases = ["resource", "res"])]
    Resources {
        /// Show node capacity
        #[arg(short, long)]
        capacity: bool,
    },
}

impl Node {
    pub async fn exec(&self, context: &Context) -> RkResult<()> {
        match self {
            Self::Info => self.info(context).await,
            Self::ListImages => self.list_images(context).await,
            Self::Resources { capacity } => self.resources(context, *capacity).await,
        }
    }

    pub async fn info(&self, context: &Context) -> RkResult<()> {
        for node in self.nodes(context).await? {
            let name = node.name_any();
            if let Some(info) = node_info(node) {
                context.ui().print(format!("\n{name}"));
                context.ui().show(info, &default());
            }
        }

        Ok(())
    }

    pub async fn list_images(&self, context: &Context) -> RkResult<()> {
        for node in self.nodes(context).await? {
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

    async fn resources(&self, context: &Context, capacity: bool) -> RkResult<()> {
        let nodes = self.nodes(context).await?;
        let resources = Resources::from_nodes(nodes, capacity);
        context.ui().show(resources, &default());

        Ok(())
    }

    async fn nodes(&self, context: &Context) -> kube::Result<Vec<corev1::Node>> {
        let kubeapi = context.kubeapi();
        let lp = kubeapi.list_params();
        kubeapi.nodes()?.list(&lp).await.map(|list| list.items)
    }
}

fn node_info(node: corev1::Node) -> Option<corev1::NodeSystemInfo> {
    node.status?.node_info
}
