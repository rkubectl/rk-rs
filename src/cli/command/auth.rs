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
    pub async fn exec(&self, kubectl: &Kubectl, output: OutputFormat) -> kube::Result<()> {
        println!("{kubectl:?} {output:?}");
        match self {
            Self::CanI(can_i) => can_i.ask(kubectl).await,
            Self::Reconcile => Ok(()),
            Self::Whoami => Ok(()),
        }
    }
}
