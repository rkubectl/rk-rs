use super::*;

use cani::CanI;
use whoami::WhoAmI;

mod cani;
mod whoami;

#[derive(Clone, Debug, Subcommand)]
pub enum Auth {
    CanI(CanI),
    Reconcile,
    Whoami(WhoAmI),
}

impl Auth {
    pub async fn exec(self, context: &Context) -> kube::Result<()> {
        match self {
            Self::CanI(can_i) => can_i.ask(context).await,
            Self::Reconcile => Ok(()),
            Self::Whoami(whoami) => whoami.ask(context).await,
        }
    }
}
