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
    pub async fn ask(self, context: &Context) -> kube::Result<()> {
        let kubectl = context.kubectl();
        let ssar = Object::from_text(self.object, kubectl)
            .map_err(|_err| kube::Error::LinesCodecMaxLineLengthExceeded)?
            .ask(kubectl, &self.verb)
            .await?;

        let show_params = default();
        let output = context.output_deprecated();
        println!("{}", ssar.output(false, &show_params, output));
        Ok(())
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

    async fn ask(
        &self,
        kubectl: &Kubectl,
        verb: &str,
    ) -> kube::Result<authorizationv1::SelfSubjectAccessReview> {
        let ssar = authorizationv1::SelfSubjectAccessReview {
            spec: self.spec(kubectl, verb),
            ..default()
        };
        let pp = kubectl.post_params();
        kubectl
            .selfsubjectaccessreviews()?
            .create(&pp, &ssar)
            .await
            .inspect(|k| kubectl.inspect(k))
            .inspect_err(|err| kubectl.inspect_err(err))
    }

    fn spec(&self, kubectl: &Kubectl, verb: &str) -> authorizationv1::SelfSubjectAccessReviewSpec {
        match self {
            Self::Resource(resource_arg) => authorizationv1::SelfSubjectAccessReviewSpec {
                resource_attributes: Some(self.resource_attributes(kubectl, resource_arg, verb)),
                ..default()
            },
            Self::NonResourceUrl(path) => authorizationv1::SelfSubjectAccessReviewSpec {
                non_resource_attributes: Some(self.non_resource_attributes(path, verb)),
                ..default()
            },
        }
    }

    fn resource_attributes(
        &self,
        kubectl: &Kubectl,
        resource_arg: &ResourceArg,
        verb: &str,
    ) -> authorizationv1::ResourceAttributes {
        let (scope, resource) = resource_arg.resource().api_resource();
        let api::ApiResource {
                group,
                version,
                plural,
                ..
                // kind,
                // api_version,
            } = resource;

        let namespace = match scope {
            discovery::Scope::Cluster => None,
            discovery::Scope::Namespaced => kubectl.namespace().namespace(),
        };

        authorizationv1::ResourceAttributes {
            verb: Some(verb.to_string()),
            group: Some(group),
            name: Some(String::new()),
            namespace,
            resource: Some(plural),
            subresource: Some(String::new()),
            version: Some(version),
            // field_selector: todo!(),
            // label_selector: todo!(),
            ..default()
        }
    }

    fn non_resource_attributes(
        &self,
        path: &str,
        verb: &str,
    ) -> authorizationv1::NonResourceAttributes {
        authorizationv1::NonResourceAttributes {
            path: Some(path.to_string()),
            verb: Some(verb.to_string()),
        }
    }
}
