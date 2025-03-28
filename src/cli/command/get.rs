use super::*;

/// Display one or many resources
#[derive(Clone, Debug, Args)]

pub struct Get {
    /// If present, list the resource type for the requested object(s).
    #[arg(long)]
    show_kind: bool,

    /// When printing, show all labels as the last column (default hide labels column)
    #[arg(long)]
    show_labels: bool,

    #[arg(
            // value_delimiter = ',',
            required = true
        )]
    resources: Vec<String>,
}

impl Get {
    pub async fn exec(&self, kubectl: &Kubectl, output: OutputArg) -> kube::Result<()> {
        let resources = ResourceArg::from_strings(&self.resources)
            .map_err(|_err| kube::Error::LinesCodecMaxLineLengthExceeded)?;
        tracing::info!(args=?self.resources, ?resources);
        let namespace = kubectl.show_namespace();
        let show_kind = self.show_kind || resources.len() > 1;
        for resource in resources {
            let data = resource.get(kubectl).await?;
            println!("{}", data.output(namespace, show_kind, &output));
        }
        Ok(())
    }
}
