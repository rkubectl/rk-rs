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
        println!("{kubectl:?}");
        match self {
            Self::Resource(arg) => println!("{verb} {arg:?}"),
            Self::NonResourceUrl(url) => println!("{verb} {url}"),
        }
        Ok(())
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
