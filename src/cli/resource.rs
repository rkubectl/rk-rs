use super::*;

#[derive(Clone, Debug, Subcommand)]
pub enum Resource {
    #[command(aliases = ["po", "pod"])]
    Pods,

    #[command(aliases = ["no", "node"])]
    Nodes,
    // Other(String),
}
