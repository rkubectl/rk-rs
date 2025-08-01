use super::*;

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
        default_value = "none",
        value_parser = PossibleValuesParser::new(["none", "server", "client"]),
)]
    dry_run: String,

    /// Edit the API resource before creating
    #[arg(long)]
    edit: bool,

    /// Name of the manager used to track field ownership.
    #[arg(long, default_value = "kubectl-create")]
    field_manager: String,

    /// Filename, directory, or URL to files to use to create the resource
    #[arg(short, long, required(true))]
    filename: Option<String>,

    /// Process the directory used in -f, --filename recursively.
    /// Useful when you want to manage related manifests organized within the same directory.
    #[arg(long, short = 'R')]
    recursive: bool,

    /// If true, the configuration of current object will be saved in its annotation.
    /// Otherwise, the annotation will be unchanged.
    /// This flag is useful when you want to perform kubectl apply on this object in the future.
    #[arg(long)]
    save_config: bool,

    #[command(subcommand)]
    command: Option<CreateResource>,
}

#[derive(Clone, Debug, Subcommand)]
#[command(rename_all = "lowercase")]
pub enum CreateResource {
    /// Create a cluster role
    ClusterRole,
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
    Namespace,
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
    Secret,
    /// Create a service using a specified subcommand
    Service,
    /// Create a service account with the specified name
    ServiceAccount,
    /// Request a service account token
    Token,
}

impl Create {
    pub async fn exec(self, _kubectl: &Kubectl) -> kube::Result<()> {
        println!("{self:?}");
        Ok(())
    }
}
