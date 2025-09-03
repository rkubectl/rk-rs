#[derive(Clone, Debug, Default)]
pub enum Namespace {
    All,
    #[default]
    Default,
    Namespace(String),
}

impl Namespace {
    pub fn new(all_namespaces: bool, namespace: Option<&str>) -> Self {
        let namespace = namespace.map(ToString::to_string);
        match (all_namespaces, namespace) {
            (true, _) => Self::All,
            (false, None) => Self::Default,
            (false, Some(namespace)) => Self::Namespace(namespace),
        }
    }

    pub fn namespace(&self) -> Option<String> {
        match self {
            Self::All => None,
            Self::Default => None,
            Self::Namespace(namespace) => Some(namespace.clone()),
        }
    }
}
