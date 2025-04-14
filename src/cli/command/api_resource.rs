use super::*;

#[derive(Clone, Debug, Args)]
pub struct ApiResources {
    /// If false, only non-namespaced resources are shown
    #[arg(long)]
    namespaced: Option<bool>,

    /// Show also subresources
    #[arg(long)]
    subresources: bool,
}

impl ApiResources {
    pub async fn exec(self, kubectl: &Kubectl) -> kube::Result<()> {
        let core = kubectl.get_core_api_resources().await?;
        let apis = kubectl.get_api_resources().await?;
        let ar = core
            .into_iter()
            .chain(apis)
            .flat_map(|group| self.collect(group))
            .collect::<Vec<_>>();

        let mut table = tabled::Table::new(ar);
        table.with(Style::blank());
        if matches!(kubectl.output(), cli::OutputFormat::Normal) {
            table
                .with(Remove::column(ByColumnName::new("VERBS")))
                .with(Remove::column(ByColumnName::new("CATEGORIES")));
        }
        println!("{table}");

        Ok(())
    }

    fn collect(&self, list: metav1::APIResourceList) -> Vec<ApiResource> {
        let metav1::APIResourceList {
            group_version,
            resources,
        } = list;

        resources
            .into_iter()
            .filter(|resource| self.subresources || !resource.name.contains("/"))
            .filter(|resource| {
                self.namespaced.is_none() || self.namespaced == Some(resource.namespaced)
            })
            .map(|resource| ApiResource::new(&group_version, resource))
            .collect()
    }
}

#[derive(Debug, tabled::Tabled)]
#[tabled(rename_all = "UPPERCASE")]
pub struct ApiResource {
    name: String,
    shortnames: String,
    apiversion: String,
    namespaced: bool,
    kind: String,
    verbs: String,
    categories: String,
}

impl ApiResource {
    pub fn new(apiversion: &str, resource: metav1::APIResource) -> Self {
        let metav1::APIResource {
            categories,
            // group,
            kind,
            name,
            namespaced,
            short_names,
            // singular_name,
            // storage_version_hash,
            verbs,
            // version,
            ..
        } = resource;
        let apiversion = apiversion.to_string();
        let shortnames = option_vec(short_names);
        let verbs = verbs.join(",");
        let categories = option_vec(categories);
        Self {
            name,
            shortnames,
            apiversion,
            namespaced,
            kind,
            verbs,
            categories,
        }
    }
}

fn option_vec(value: Option<Vec<String>>) -> String {
    value
        .as_deref()
        .map(|values| values.join(","))
        .unwrap_or_default()
}
