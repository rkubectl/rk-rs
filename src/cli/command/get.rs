use super::*;

/// Display one or many resources
#[derive(Clone, Debug, Args)]

pub struct Get {
    #[command(flatten)]
    params: ShowParams,

    /// Raw URI to request from the server.  Uses the transport specified by the kubeconfig file.
    #[arg(long, conflicts_with = "resources")]
    raw: Option<String>,

    #[arg(
            // value_delimiter = ',',
            required = true
        )]
    resources: Option<Vec<String>>,
}

impl Get {
    pub async fn exec(&self, kubectl: &Kubectl) -> kube::Result<()> {
        if let Some(raw) = self.raw.as_deref() {
            let name = raw.strip_prefix("/").unwrap_or(raw);
            let text = kubectl.raw(name).await?;
            println!("{text}");
        } else {
            let resources = self.resources.as_deref().unwrap_or_default();
            let resources = ResourceArg::from_strings(resources)
                .map_err(|_err| kube::Error::LinesCodecMaxLineLengthExceeded)?;
            let mut params = self.params;
            params.show_kind |= resources.len() > 1;
            tracing::info!(args=?self.resources, ?resources);
            let namespace = kubectl.show_namespace();
            for resource in resources {
                let data = resource.get(kubectl).await?;
                println!("{}", data.output(namespace, &params, kubectl.output()));
            }
        }
        Ok(())
    }
}
