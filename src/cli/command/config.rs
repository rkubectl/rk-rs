use super::*;

/// Modify kubeconfig files using subcommands like "kubectl config set current-context my-context".
///  The loading order follows these rules:
///   1.  If the --kubeconfig flag is set, then only that file is loaded.
///       The flag may only be set once and no merging takes place.
///   2.  If $KUBECONFIG environment variable is set, then it is used as a list of paths (normal path delimiting rules for your system).
///       These paths are merged. When a value is modified, it is modified in the file that defines the stanza.
///       When a value is created, it is created in the first file that exists.
///       If no files in the chain exist, then it creates the last file in the list.
///   3.  Otherwise, ${HOME}/.kube/config is used and no merging takes place.
///

#[derive(Clone, Debug, Subcommand)]
pub enum Config {
    /// Display the current-context
    CurrentContext,
    /// Delete the specified cluster from the kubeconfig
    DeleteCluster,
    /// Delete the specified context from the kubeconfig
    DeleteContext,
    /// Delete the specified user from the kubeconfig
    DeleteUser,
    /// Display clusters defined in the kubeconfig
    GetClusters,
    /// Describe one or many contexts
    GetContexts,
    /// Display users defined in the kubeconfig
    GetUsers,
    /// Rename a context from the kubeconfig file
    RenameContext,
    /// Set an individual value in a kubeconfig file
    Set,
    /// Set a cluster entry in kubeconfig
    SetCluster,
    /// Set a context entry in kubeconfig
    SetContext,
    /// Set a user entry in kubeconfig
    SetCredentials,
    /// Unset an individual value in a kubeconfig file
    Unset,
    /// Set the current-context in a kubeconfig file
    UseContext,
    /// Display merged kubeconfig settings or a specified kubeconfig file
    View,
}

impl Config {
    pub fn exec(self, kubectl: &Kubectl) -> kube::Result<()> {
        match self {
            Self::CurrentContext => kubectl.current_context(),
            Self::DeleteCluster => todo!(),
            Self::DeleteContext => todo!(),
            Self::DeleteUser => todo!(),
            Self::GetClusters => kubectl.get_clusters(),
            Self::GetContexts => kubectl.get_contexts(),
            Self::GetUsers => kubectl.get_users(),
            Self::RenameContext => todo!(),
            Self::Set => todo!(),
            Self::SetCluster => todo!(),
            Self::SetContext => todo!(),
            Self::SetCredentials => todo!(),
            Self::Unset => todo!(),
            Self::UseContext => todo!(),
            Self::View => todo!(),
        }
    }
}
