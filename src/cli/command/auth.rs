use super::*;

use cani::CanI;

mod cani;

#[derive(Clone, Debug, Subcommand)]
pub enum Auth {
    CanI(CanI),
    Reconcile,
    Whoami,
}

impl Auth {
    pub async fn exec(&self, kubectl: &Kubectl) -> kube::Result<()> {
        match self {
            Self::CanI(can_i) => can_i.ask(kubectl).await,
            Self::Reconcile => Ok(()),
            Self::Whoami => Ok(()),
        }
    }
}
