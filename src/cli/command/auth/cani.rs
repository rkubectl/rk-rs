use super::*;

/// Check whether an action is allowed.
#[derive(Clone, Debug, Args)]
pub struct CanI {
    verb: String,

    object: Object,

    /// If true, prints all allowed actions.
    #[arg(long)]
    list: bool,

    /// SubResource such as pod/log or deployment/scale
    #[arg(long)]
    subresource: Option<String>,
}

#[derive(Clone, Debug)]
enum Object {
    Resource(ResourceArg),
    NonResourceUrl(String),
}

impl CanI {
    pub async fn ask(&self, kubectl: &Kubectl) -> kube::Result<()> {
        self.object.ask(kubectl, &self.verb).await
    }
}

impl Object {
    fn non_resource_url(text: &str) -> Self {
        Self::NonResourceUrl(text.to_string())
    }

    async fn ask(&self, kubectl: &Kubectl, verb: &str) -> kube::Result<()> {
        let ssar: authorizationv1::SelfSubjectAccessReview =
            authorizationv1::SelfSubjectAccessReview {
                spec: self.spec(kubectl, verb).await?,
                ..default()
            };
        let pp = kubectl.post_params();
        let ssar = kubectl
            .selfsubjectaccessreviews()
            .create(&pp, &ssar)
            .await
            .inspect(|k| kubectl.inspect(k))
            .inspect_err(|err| kubectl.inspect_err(err))?;

        let show_params = default();
        println!("{}", ssar.output(false, &show_params, kubectl.output()));
        Ok(())
    }

    async fn spec(
        &self,
        kubectl: &Kubectl,
        verb: &str,
    ) -> kube::Result<authorizationv1::SelfSubjectAccessReviewSpec> {
        Ok(authorizationv1::SelfSubjectAccessReviewSpec {
            resource_attributes: self.resource_attributes(kubectl, verb).await?,
            non_resource_attributes: self.non_resource_attributes(verb),
        })
    }

    async fn resource_attributes(
        &self,
        kubectl: &Kubectl,
        verb: &str,
    ) -> kube::Result<Option<authorizationv1::ResourceAttributes>> {
        let resource_attributes = if let Self::Resource(resource_arg) = self {
            let Some(api::ApiResource {
                group,
                version,
                // kind,
                plural,
                ..
                // api_version,
            }) = resource_arg.resource().api_resource(kubectl).await?
            else {
                panic!("No api resource found")
            };
            // let name = resource_arg.name().map(|name| name.to_string());
            let namespace = kubectl.default_namespace().to_string();
            Some(authorizationv1::ResourceAttributes {
                verb: Some(verb.to_string()),
                group: Some(group),
                name: Some(String::new()),
                namespace: Some(namespace),
                resource: Some(plural),
                subresource: Some(String::new()),
                version: Some(version),
                // field_selector: todo!(),
                // label_selector: todo!(),
                ..default()
            })
        } else {
            None
        };
        Ok(resource_attributes)
    }

    fn non_resource_attributes(
        &self,
        verb: &str,
    ) -> Option<authorizationv1::NonResourceAttributes> {
        if let Self::NonResourceUrl(path) = self {
            Some(authorizationv1::NonResourceAttributes {
                path: Some(path.to_string()),
                verb: Some(verb.to_string()),
            })
        } else {
            None
        }
    }
}

impl std::str::FromStr for Object {
    type Err = resource::InvalidResourceSpec;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        if text.starts_with("/") {
            Ok(Self::non_resource_url(text))
        } else {
            text.parse().map(Self::Resource)
        }
    }
}
