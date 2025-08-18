use super::*;

#[derive(Clone, Debug, Args)]
pub struct CreateGenericSecret {
    name: String,
    /// Specify the path to a file to read lines of key=val pairs to create a secret.
    #[arg(long)]
    from_env_file: Vec<String>,

    /// Key files can be specified using their file path,
    /// in which case a default name will be given to them,
    /// or optionally with a name and file path,
    /// in which case the given name will be used.
    /// Specifying a directory will iterate each named file in the directory that is a valid secret key.
    #[arg(long)]
    from_file: Vec<String>,

    /// Specify a key and literal value to insert in secret (i.e. mykey=somevalue)
    #[arg(long)]
    from_literal: Vec<String>,
}

impl CreateGenericSecret {
    pub async fn exec(
        &self,
        kubectl: &Kubectl,
        pp: &api::PostParams,
    ) -> kube::Result<corev1::Secret> {
        trace!(?kubectl, ?pp, name = self.name);
        let data = corev1::Secret::new(&self.name);
        let k = data;
        Ok(k)
    }
}
