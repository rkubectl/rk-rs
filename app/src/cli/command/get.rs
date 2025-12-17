use super::*;

/// Display one or many resources
///
///  Prints a table of the most important information about the specified resources. You can filter the list using a label
/// selector and the --selector flag. If the desired resource type is namespaced you will only see results in the current
/// namespace if you don't specify any namespace.
///
///  By specifying the output as 'template' and providing a Go template as the value of the --template flag, you can filter
/// the attributes of the fetched resources.
///
/// Use "kubectl api-resources" for a complete list of supported resources.
///
#[derive(Clone, Debug, Args)]

pub struct Get {
    #[command(flatten)]
    params: ShowParams,

    /// Raw URI to request from the server.  Uses the transport specified by the kubeconfig file.
    #[arg(long, conflicts_with = "regular")]
    raw: Option<String>,

    /// If specified, gets the subresource of the requested object.
    #[arg(group = "regular", long)]
    subresource: Option<String>,

    /// Resources to fetch
    #[arg(group = "regular", required = true)]
    resources: Option<Vec<String>>,
}

impl Get {
    pub async fn exec(&self, context: &Context) -> RkResult<()> {
        let kubeapi = context.kubeapi();
        if let Some(raw) = self.raw.as_deref() {
            let name = raw.strip_prefix("/").unwrap_or(raw);
            let text = kubeapi.raw_get(name).await?;
            println!("{text}");
        } else {
            let resources = self.resources(kubeapi)?;
            let mut params = self.params;
            params.show_kind |= resources.len() > 1;
            let namespace = kubeapi.show_namespace();
            for resource in resources {
                let data = resource.get(kubeapi).await?;
                let output = context.output_deprecated();
                println!("{}", data.output(namespace, params, output));
            }
        }
        Ok(())
    }

    fn resources(&self, kubeapi: &Kubeapi) -> kube::Result<Vec<ResourceArg>> {
        let resources = self.resources.as_deref().unwrap_or_default();
        ResourceArg::from_strings(resources, kubeapi)
            .inspect(|resources| info!(args=?self.resources, ?resources))
            .map_err(|_err| kube::Error::LinesCodecMaxLineLengthExceeded)
    }
}
