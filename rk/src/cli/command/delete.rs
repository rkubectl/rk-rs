use clap::builder::ArgPredicate;

use super::*;

/// Delete resources by file names, stdin, resources and names, or by resources and label selector.

//  JSON and YAML formats are accepted. Only one type of argument may be specified: file names, resources and names, or
// resources and label selector.

//  Some resources, such as pods, support graceful deletion. These resources define a default period before they are
// forcibly terminated (the grace period) but you may override that value with the --grace-period flag, or pass --now to
// set a grace-period of 1. Because these resources often represent entities in the cluster, deletion may not be
// acknowledged immediately. If the node hosting a pod is down or cannot reach the API server, termination may take
// significantly longer than the grace period. To force delete a resource, you must specify the --force flag. Note: only a
// subset of resources support graceful deletion. In absence of the support, the --grace-period flag is ignored.

//  IMPORTANT: Force deleting pods does not wait for confirmation that the pod's processes have been terminated, which can
// leave those processes running until the node detects the deletion and completes graceful deletion. If your processes use
// shared storage or talk to a remote API and depend on the name of the pod to identify themselves, force deleting those
// pods may result in multiple processes running on different machines using the same identification which may lead to data
// corruption or inconsistency. Only force delete pods when you are sure the pod is terminated, or if your application can
// tolerate multiple copies of the same pod running at once. Also, if you force delete pods, the scheduler may place new
// pods on those nodes before the node has released those resources and causing those pods to be evicted immediately.

//  Note that the delete command does NOT do resource version checks, so if someone submits an update to a resource right
// when you submit a delete, their update will be lost along with the rest of the resource.

//  After a CustomResourceDefinition is deleted, invalidation of discovery cache may take up to 6 hours. If you don't want
// to wait, you might want to run "kubectl api-resources" to refresh the discovery cache.

// Examples:
//   # Delete a pod using the type and name specified in pod.json
//   kubectl delete -f ./pod.json

//   # Delete resources from a directory containing kustomization.yaml - e.g. dir/kustomization.yaml
//   kubectl delete -k dir

//   # Delete resources from all files that end with '.json'
//   kubectl delete -f '*.json'

//   # Delete a pod based on the type and name in the JSON passed into stdin
//   cat pod.json | kubectl delete -f -

//   # Delete pods and services with same names "baz" and "foo"
//   kubectl delete pod,service baz foo

//   # Delete pods and services with label name=myLabel
//   kubectl delete pods,services -l name=myLabel

//   # Delete a pod with minimal delay
//   kubectl delete pod foo --now

//   # Force delete a pod on a dead node
//   kubectl delete pod foo --force

//   # Delete all pods
//   kubectl delete pods --all

//   # Delete all pods only if the user confirms the deletion
//   kubectl delete pods --all --interactive

// Options:
//     --all=false:
//         Delete all resources, in the namespace of the specified resource types.

//     -A, --all-namespaces=false:
//         If present, list the requested object(s) across all namespaces. Namespace in current context is ignored even
//         if specified with --namespace.

//     --cascade='background':
//         Must be "background", "orphan", or "foreground". Selects the deletion cascading strategy for the dependents
//         (e.g. Pods created by a ReplicationController). Defaults to background.

//     --dry-run='none':
//         Must be "none", "server", or "client". If client strategy, only print the object that would be sent, without
//         sending it. If server strategy, submit server-side request without persisting the resource.

//     --field-selector='':
//         Selector (field query) to filter on, supports '=', '==', and '!='.(e.g. --field-selector
//         key1=value1,key2=value2). The server only supports a limited number of field queries per type.

//     -f, --filename=[]:
//         containing the resource to delete.

//     --force=false:
//         If true, immediately remove resources from API and bypass graceful deletion. Note that immediate deletion of
//         some resources may result in inconsistency or data loss and requires confirmation.

//     --grace-period=-1:
//         Period of time in seconds given to the resource to terminate gracefully. Ignored if negative. Set to 1 for
//         immediate shutdown. Can only be set to 0 when --force is true (force deletion).

//     --ignore-not-found=false:
//         Treat "resource not found" as a successful delete. Defaults to "true" when --all is specified.

//     -i, --interactive=false:
//         If true, delete resource only when user confirms.

//     -k, --kustomize='':
//         Process a kustomization directory. This flag can't be used together with -f or -R.

//     --now=false:
//         If true, resources are signaled for immediate shutdown (same as --grace-period=1).

//     -o, --output='':
//         Output mode. Use "-o name" for shorter output (resource/name).

//     --raw='':
//         Raw URI to DELETE to the server.  Uses the transport specified by the kubeconfig file.

//     -R, --recursive=false:
//         Process the directory used in -f, --filename recursively. Useful when you want to manage related manifests
//         organized within the same directory.

//     -l, --selector='':
//         Selector (label query) to filter on, supports '=', '==', '!=', 'in', 'notin'.(e.g. -l
//         key1=value1,key2=value2,key3 in (value3)). Matching objects must satisfy all of the specified label
//         constraints.

//     --timeout=0s:
//         The length of time to wait before giving up on a delete, zero means determine a timeout from the size of the
//         object

//     --wait=true:
//         If true, wait for resources to be gone before returning. This waits for finalizers.

// Usage:
//   kubectl delete ([-f FILENAME] | [-k DIRECTORY] | TYPE [(NAME | -l label | --all)]) [options]

// Use "kubectl options" for a list of global command-line options (applies to all commands).

#[derive(Clone, Debug, Args)]
#[command(arg_required_else_help(true))]
pub struct Delete {
    /// Selects the deletion cascading strategy for the dependents
    /// Must be "background", "orphan", or "foreground".
    /// (e.g. Pods created by a ReplicationController). Defaults to background.
    #[arg(long, value_enum, default_value_t = Cascade::Background)]
    cascade: Cascade,

    /// If client strategy, only print the object that would be sent, without sending it.
    /// If server strategy, submit server-side request without persisting the resource.
    #[arg(
        long,
        value_enum,
        default_value_t = DryRun::None,
        // value_parser = PossibleValuesParser::new(["none", "server", "client"]),
    )]
    dry_run: DryRun,

    /// Filename, directory, or URL to files to use to create the resource
    #[arg(short, long, required_unless_present("TYPE"))]
    filename: Option<String>,

    /// If true, immediately remove resources from API and bypass graceful deletion.
    /// Note that immediate deletion of some resources may result in inconsistency
    /// or data loss and requires confirmation.
    #[arg(long)]
    force: bool,

    /// Treat "resource not found" as a successful delete. Defaults to "true" when --all is specified.
    #[arg(long, default_value_if("all", ArgPredicate::IsPresent, "true"))]
    ignore_not_found: bool,

    /// Process the directory used in -f, --filename recursively.
    /// Useful when you want to manage related manifests organized within the same directory.
    #[arg(long, short = 'R', requires = "filename")]
    recursive: bool,

    /// Delete all resources, in the namespace of the specified resource types.
    #[arg(long)]
    all: bool,

    /// If true, wait for resources to be gone before returning.
    /// This waits for finalizers.
    #[arg(long, default_value_t = true)]
    wait: bool,

    #[arg(id = "TYPE", value_name = "TYPE", required_unless_present("filename"))]
    resources: Option<Vec<String>>,
}

impl Delete {
    pub async fn exec(self, context: &Context) -> kube::Result<()> {
        let kubeapi = context.kubeapi();
        if let Some(filename) = &self.filename {
            let dp = api::DeleteParams::default();
            todo!("Deleting from {filename} ({dp:?}");
        } else {
            self.delete_resources(kubeapi).await
        }
    }

    async fn delete_resources(&self, kubeapi: &Kubeapi) -> kube::Result<()> {
        let dp = kubeapi.delete_params(self.cascade, self.dry_run);
        for resource in self.resources(kubeapi)? {
            if self.dry_run == DryRun::Client {
                println!("{resource} deleted (dry run)");
            } else {
                resource
                    .delete(kubeapi, &dp, self.all)
                    .await
                    .or_else(|err| self.ignore_not_found(err))?;
            }
        }

        Ok(())
    }

    fn resources(&self, kubeapi: &Kubeapi) -> kube::Result<Vec<ResourceArg>> {
        let resources = self.resources.as_deref().unwrap_or_default();
        ResourceArg::from_strings(resources, kubeapi)
            .map_err(|_err| kube::Error::LinesCodecMaxLineLengthExceeded)
    }

    fn ignore_not_found(&self, err: kube::Error) -> kube::Result<()> {
        if self.ignore_not_found
            && matches!(
                err,
                kube::Error::Api(kube::error::ErrorResponse { code: 404, .. })
            )
        {
            Ok(())
        } else {
            Err(err)
        }
    }
}
