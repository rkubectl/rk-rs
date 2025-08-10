use super::*;

/// Check whether an action is allowed.
#[derive(Clone, Debug, Args)]
pub struct CanI {
    verb: String,

    object: String,

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
    pub async fn ask(self, kubectl: &Kubectl) -> kube::Result<()> {
        Object::from_text(self.object, kubectl)
            .map_err(|_err| kube::Error::LinesCodecMaxLineLengthExceeded)?
            .ask(kubectl, &self.verb)
            .await
    }
}

impl Object {
    fn from_text(text: String, kubectl: &Kubectl) -> Result<Self, resource::InvalidResourceSpec> {
        if text.starts_with("/") {
            Ok(Self::NonResourceUrl(text))
        } else {
            ResourceArg::from_strings(&[text], kubectl)?
                .pop()
                .map(Self::Resource)
                .ok_or(resource::InvalidResourceSpec)
        }
    }

    async fn ask(&self, kubectl: &Kubectl, verb: &str) -> kube::Result<()> {
        let ssar = authorizationv1::SelfSubjectAccessReview {
            spec: self.spec(kubectl, verb).await?,
            ..default()
        };
        let pp = kubectl.post_params();
        let ssar = kubectl
            .selfsubjectaccessreviews()?
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
            let api::ApiResource {
                group,
                version,
                // kind,
                plural,
                ..
                // api_version,
            } = resource_arg.resource().api_resource()
;
            Some(authorizationv1::ResourceAttributes {
                verb: Some(verb.to_string()),
                group: Some(group),
                name: Some(String::new()),
                namespace: kubectl.namespace().namespace(),
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
