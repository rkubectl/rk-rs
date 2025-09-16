use super::*;

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum RkError {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Kube(#[from] kube::Error),
    #[error("Not implemented yet")]
    #[diagnostic(help("This functionality is not implemented yet"))]
    NotImplemented,
}

impl RkError {
    pub fn todo() -> Self {
        Self::NotImplemented
    }
}
