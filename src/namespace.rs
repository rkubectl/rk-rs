#[derive(Clone, Debug, Default)]
pub enum Namespace {
    All,
    #[default]
    Default,
    Namespace(String),
}

impl Namespace {
    pub fn new(all_namespaces: bool, namespace: Option<String>) -> Self {
        match (all_namespaces, namespace) {
            (true, _) => Self::All,
            (false, None) => Self::Default,
            (false, Some(namespace)) => Self::Namespace(namespace),
        }
    }
}
