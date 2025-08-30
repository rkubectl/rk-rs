use super::*;

use clusterrole::CreateClusterRole;
use namespace::CreateNamespace;
use secret::CreateSecret;

mod clusterrole;
mod namespace;
mod secret;

/// Create a resource from a file or from stdin.
///
///  JSON and YAML formats are accepted.
///
/// Examples:
///   # Create a pod using the data in pod.json
///   kubectl create -f ./pod.json
///
///   # Create a pod based on the JSON passed into stdin
///   cat pod.json | kubectl create -f -
///
///   # Edit the data in registry.yaml in JSON then create the resource using the edited data
///   kubectl create -f registry.yaml --edit -o json

#[derive(Clone, Debug, Args)]
#[command(arg_required_else_help(true), subcommand_negates_reqs(true))]
pub struct Create {
    /// If client strategy, only print the object that would be sent, without sending it.
    /// If server strategy, submit server-side request without persisting the resource.
    #[arg(
        long,
        value_enum,
        default_value_t = DryRun::None,
        // value_parser = PossibleValuesParser::new(["none", "server", "client"]),
    )]
    dry_run: DryRun,

    /// Edit the API resource before creating
    #[arg(long)]
    edit: bool,

    /// Name of the manager used to track field ownership.
    #[arg(long, default_value = "kubectl-create")]
    field_manager: String,

    /// Filename, directory, or URL to files to use to create the resource
    #[arg(short, long, required = true)]
    filename: Option<String>,

    /// Process the directory used in -f, --filename recursively.
    /// Useful when you want to manage related manifests organized within the same directory.
    #[arg(long, short = 'R', requires = "filename")]
    recursive: bool,

    /// If true, the configuration of current object will be saved in its annotation.
    /// Otherwise, the annotation will be unchanged.
    /// This flag is useful when you want to perform kubectl apply on this object in the future.
    #[arg(long)]
    save_config: bool,

    /// If true, keep the managedFields when printing objects in JSON or YAML format.
    #[arg(long)]
    show_managed_fields: bool,

    #[command(subcommand)]
    command: Option<CreateResource>,
}

#[derive(Clone, Debug, Subcommand)]
#[command(rename_all = "lowercase")]
pub enum CreateResource {
    /// Create a cluster role
    ClusterRole(CreateClusterRole),
    /// Create a cluster role binding for a particular cluster role
    ClusterRoleBinding,
    /// Create a config map from a local file, directory or literal value
    ConfigMap,
    /// Create a cron job with the specified name
    CronJob,
    /// Create a deployment with the specified name
    Deployment,
    /// Create an ingress with the specified name
    Ingress,
    /// Create a job with the specified name
    Job,
    /// Create a namespace with the specified name
    #[command(visible_alias = "ns")]
    Namespace(CreateNamespace),
    /// Create a pod disruption budget with the specified name
    PodDisruptionBudget,
    /// Create a priority class with the specified name
    PriorityClass,
    /// Create a quota with the specified name
    Quota,
    /// Create a role with single rule
    Role,
    /// Create a role binding for a particular role or cluster role
    RoleBinding,
    /// Create a secret using a specified subcommand
    #[command(subcommand)]
    Secret(CreateSecret),
    /// Create a service using a specified subcommand
    Service,
    /// Create a service account with the specified name
    ServiceAccount,
    /// Request a service account token
    Token,
}

impl Create {
    pub async fn exec(self, kubectl: &Kubectl) -> kube::Result<()> {
        let created = if let Some(filename) = &self.filename {
            self.create_from_file(filename, kubectl).await
        } else {
            self.create_resource(kubectl).await
        }?;

        let namespace = kubectl.show_namespace();
        let params = self.params();
        let output = kubectl.output();
        println!("{}", created.output(namespace, &params, output));
        Ok(())
    }

    async fn create_from_file(
        &self,
        filename: &str,
        kubectl: &Kubectl,
    ) -> kube::Result<Box<dyn Show>> {
        let _pp = kubectl.post_params_with_manager(&self.field_manager);
        println!("Creating from {filename}, ({kubectl:?})");
        todo!()
    }

    async fn create_resource(&self, kubectl: &Kubectl) -> kube::Result<Box<dyn Show>> {
        if let Some(command) = &self.command {
            let pp = kubectl.post_params_with_manager(&self.field_manager);
            command.exec(kubectl, &pp).await
        } else {
            unreachable!()
        }
    }

    fn params(&self) -> ShowParams {
        ShowParams {
            show_managed_fields: self.show_managed_fields,
            ..default()
        }
    }
}

impl CreateResource {
    pub async fn exec(
        &self,
        kubectl: &Kubectl,
        pp: &api::PostParams,
    ) -> kube::Result<Box<dyn Show>> {
        match self {
            Self::ClusterRole(cluster_role) => cluster_role.exec(kubectl, pp).await,
            Self::ClusterRoleBinding => todo!(),
            Self::ConfigMap => todo!(),
            Self::CronJob => todo!(),
            Self::Deployment => todo!(),
            Self::Ingress => todo!(),
            Self::Job => todo!(),
            Self::Namespace(namespace) => namespace.exec(kubectl, pp).await,
            Self::PodDisruptionBudget => todo!(),
            Self::PriorityClass => todo!(),
            Self::Quota => todo!(),
            Self::Role => todo!(),
            Self::RoleBinding => todo!(),
            Self::Secret(secret) => secret.exec(kubectl, pp).await,
            Self::Service => todo!(),
            Self::ServiceAccount => todo!(),
            Self::Token => todo!(),
        }
    }
}

#[derive(Debug)]
pub struct Created<K> {
    // pub resource: CreateResource,
    pub k: K,
}
