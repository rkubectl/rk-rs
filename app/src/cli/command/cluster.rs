use std::iter;

use ext::ServiceGetExt2;
use kube::Resource;

use super::*;

/// Cluster Management Commands
#[derive(Clone, Debug, Subcommand)]
pub enum ClusterManagement {
    /// Modify certificate resources
    Certificate,

    ClusterInfo(ClusterInfo),

    /// Display resource (CPU/memory) usage
    Top,

    /// Mark node as unschedulable
    Cordon,

    /// Mark node as schedulable
    Uncordon,

    /// Drain node in preparation for maintenance
    Drain,

    /// Update the taints on one or more nodes
    Taint,
}

impl ClusterManagement {
    pub async fn exec(self, context: &Context) -> RkResult<()> {
        // context.ui().not_implemented(&self);
        match self {
            Self::Certificate => Err(RkError::todo()),
            Self::ClusterInfo(cluster_info) => cluster_info.exec(context).await,
            Self::Top => Err(RkError::todo()),
            Self::Cordon => Err(RkError::todo()),
            Self::Uncordon => Err(RkError::todo()),
            Self::Drain => Err(RkError::todo()),
            Self::Taint => Err(RkError::todo()),
        }
    }
}

/// Display cluster information

#[derive(Clone, Debug, Args)]
pub struct ClusterInfo {
    #[command(subcommand)]
    dump: Option<Dump>,
}

impl ClusterInfo {
    const _CLUSTER_SERVICE_LABEL: &str = "kubernetes.io/cluster-service=true";

    pub async fn exec(self, context: &Context) -> RkResult<()> {
        match self.dump {
            Some(dump) => dump.exec(context).await,
            None => self.info(context).await,
        }
    }

    async fn info(&self, context: &Context) -> RkResult<()> {
        let kubeapi = context.kubeapi().clone().with_namespace(Namespace::All);
        let lp = kubeapi
            .list_params()
            .labels_from(&ClusterService::selector());
        let services = kubeapi
            .services()?
            .list(&lp)
            .await?
            .into_iter()
            .map(|svc| ClusterService::from_svc(&kubeapi, svc));

        iter::once(ClusterService::control_plane(&kubeapi))
            .chain(services)
            .for_each(|svc| context.show(svc));

        Ok(())
    }
}

/// Dump cluster info to stdout
#[derive(Clone, Copy, Debug, Subcommand)]
pub enum Dump {
    Dump,
}

impl Dump {
    async fn exec(&self, context: &Context) -> RkResult<()> {
        context.not_implemented(self);
        Ok(())
    }
}

#[derive(Debug)]
struct ClusterService {
    name: String,
    url: String,
    svc: Option<corev1::Service>,
}

impl ClusterService {
    fn selector() -> kube::core::Selector {
        [(k8s::label::CLUSTER_SERVICE, "true")]
            .into_iter()
            .collect()
    }

    fn control_plane(kubeapi: &Kubeapi) -> Self {
        Self {
            name: "Kubernetes Control Plane".to_string(),
            url: kubeapi.cluster_url(),
            svc: None,
        }
    }

    fn from_svc(kubeapi: &Kubeapi, svc: corev1::Service) -> Self {
        let name = svc
            .labels()
            .get(k8s::label::KUBERNETES_IO_NAME)
            .cloned()
            .unwrap_or_else(|| svc.name_any());

        let url = if let Some(ingress) = svc.ingress() {
            todo!("Handle ingress point: {ingress:?}")
        } else {
            let cluster_url = kubeapi.cluster_url();
            let url = cluster_url.trim_end_matches("/");
            format!(
                "{url}{}",
                corev1::Service::url_path(&(), svc.namespace().as_deref())
            )
        };

        Self {
            name,
            url,
            svc: Some(svc),
        }
    }
}

impl Show for ClusterService {
    fn output(&self, _namespace: bool, params: ShowParams, output: OutputFormat) -> String {
        match output {
            OutputFormat::Normal => format!("{} is running on {}", self.name, self.url),
            OutputFormat::Json => self.json(params),
            OutputFormat::Yaml => self.yaml(params),
            OutputFormat::Name => self.name(),
            OutputFormat::GoTemplate => todo!(),
            OutputFormat::GoTemplateFile => todo!(),
            OutputFormat::Template => todo!(),
            OutputFormat::TemplateFile => todo!(),
            OutputFormat::JsonPath => todo!(),
            OutputFormat::JsonPathAsJson => todo!(),
            OutputFormat::JsonPathFile => todo!(),
            OutputFormat::CustomColumns => todo!(),
            OutputFormat::CustomColumnsFile => todo!(),
            OutputFormat::Wide => todo!(),
        }
    }

    fn header(&self, output: OutputFormat) -> Vec<String> {
        self.svc
            .as_ref()
            .map(|svc| svc.header(output))
            .unwrap_or_default()
    }

    fn data(&self, params: ShowParams, output: OutputFormat) -> Vec<String> {
        self.svc
            .as_ref()
            .map(|svc| svc.data(params, output))
            .unwrap_or_default()
    }

    fn json(&self, params: ShowParams) -> String {
        self.svc
            .as_ref()
            .map(|svc| svc.json(params))
            .unwrap_or_default()
    }

    fn yaml(&self, params: ShowParams) -> String {
        self.svc
            .as_ref()
            .map(|svc| svc.yaml(params))
            .unwrap_or_default()
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}
