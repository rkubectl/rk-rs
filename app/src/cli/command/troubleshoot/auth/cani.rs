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
    pub async fn ask(self, context: &Context) -> RkResult<()> {
        let kubeapi = context.kubeapi();
        let ssar = Object::from_text(self.object, kubeapi)
            .map_err(|_err| kube::Error::LinesCodecMaxLineLengthExceeded)?
            .ask(kubeapi, &self.verb)
            .await?;

        let show_params = default();
        let output = context.output_deprecated();
        println!("{}", ssar.output(false, &show_params, output));
        Ok(())
    }
}

impl Object {
    fn from_text(text: String, kubeapi: &Kubeapi) -> Result<Self, InvalidResourceSpec> {
        if text.starts_with("/") {
            Ok(Self::NonResourceUrl(text))
        } else {
            ResourceArg::from_strings(&[text], kubeapi)?
                .pop()
                .map(Self::Resource)
                .ok_or(InvalidResourceSpec)
        }
    }

    async fn ask(
        &self,
        kubeapi: &Kubeapi,
        verb: &str,
    ) -> kube::Result<authorizationv1::SelfSubjectAccessReview> {
        let ssar = authorizationv1::SelfSubjectAccessReview {
            spec: self.spec(kubeapi, verb),
            ..default()
        };
        let pp = kubeapi.post_params();
        kubeapi
            .selfsubjectaccessreviews()?
            .create(&pp, &ssar)
            .await
            .inspect(|k| kubeapi.inspect(k))
            .inspect_err(|err| kubeapi.inspect_err(err))
    }

    fn spec(&self, kubeapi: &Kubeapi, verb: &str) -> authorizationv1::SelfSubjectAccessReviewSpec {
        match self {
            Self::Resource(resource_arg) => authorizationv1::SelfSubjectAccessReviewSpec {
                resource_attributes: Some(self.resource_attributes(kubeapi, resource_arg, verb)),
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
        kubeapi: &Kubeapi,
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
            discovery::Scope::Namespaced => kubeapi.namespace().namespace(),
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
