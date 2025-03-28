use super::*;

/// Display one or many resources
#[derive(Clone, Debug, Args)]

pub struct Get {
    #[command(flatten)]
    params: ShowParams,

    #[arg(
            // value_delimiter = ',',
            required = true
        )]
    resources: Vec<String>,
}

impl Get {
    pub async fn exec(&self, kubectl: &Kubectl, output: OutputFormat) -> kube::Result<()> {
        let resources = ResourceArg::from_strings(&self.resources)
            .map_err(|_err| kube::Error::LinesCodecMaxLineLengthExceeded)?;
        let mut params = self.params.clone();
        params.show_kind |= resources.len() > 1;
        tracing::info!(args=?self.resources, ?resources);
        let namespace = kubectl.show_namespace();
        for resource in resources {
            let data = resource.get(kubectl).await?;
            println!("{}", data.output(namespace, &params, &output));
        }
        Ok(())
    }
}
