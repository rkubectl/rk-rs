use std::collections::BTreeMap;
use std::collections::BTreeSet;

use super::*;

use image::ImageInfo;
use resources::Resources;

mod image;
mod resources;

#[derive(Clone, Debug, Subcommand)]
pub enum Node {
    Info,
    ListImages,
    Resources {
        /// Also show allocatable resources
        #[arg(short, long)]
        allocatable: bool,
    },
}

impl Node {
    pub async fn exec(&self, context: &Context) -> RkResult<()> {
        match self {
            Self::Info => self.info(context).await,
            Self::ListImages => self.list_images(context).await,
            Self::Resources { allocatable } => self.resources(context, *allocatable).await,
        }
    }

    pub async fn info(&self, context: &Context) -> RkResult<()> {
        for node in self.nodes(context).await? {
            let name = node.name_any();
            if let Some(info) = node_info(node) {
                let info = [
                    ("Architecture", info.architecture),
                    ("Boot ID", info.boot_id),
                    ("Container Runtime Version", info.container_runtime_version),
                    ("Kernel Version", info.kernel_version),
                    ("Kube Proxy Version", info.kube_proxy_version),
                    ("Kubelet Version", info.kubelet_version),
                    ("Machine ID", info.machine_id),
                    ("Operating System", info.operating_system),
                    ("OS Image", info.os_image),
                    ("System UUID", info.system_uuid),
                ]
                .into_iter()
                .collect::<BTreeMap<_, _>>();

                let mut table = tabled::builder::Builder::from(info).build();
                table.with(tabled::settings::Style::blank());
                println!("\n{name}\n");
                println!("{table}");
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

    async fn resources(&self, context: &Context, allocatable: bool) -> RkResult<()> {
        let nodes = self.nodes(context).await?;
        let resources = Resources::from_nodes(nodes, allocatable);
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
