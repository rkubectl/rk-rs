// use super::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, clap::ValueEnum)]
pub enum DryRun {
    /// Default value, just show the object
    #[default]
    None,
    /// If server strategy, submit server-side request without persisting the resource.
    Server,
    /// If client strategy, only print the object that would be sent, without sending it.
    Client,
}
