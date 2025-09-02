use super::*;

impl Kubectl {
    pub async fn info(&self, _output: &OutputFormat) -> kube::Result<()> {
        self.client()?.apiserver_version().await.map(|info| {
            println!("build date:     {}", info.build_date);
            println!("compiler:       {}", info.compiler);
            println!("git_commit:     {}", info.git_commit);
            println!("git_tree_state: {}", info.git_tree_state);
            println!("git_version:    {}", info.git_version);
            println!("go_version:     {}", info.go_version);
            println!("major:          {}", info.major);
            println!("minor:          {}", info.minor);
            println!("platform:       {}", info.platform);
        })
    }
}
