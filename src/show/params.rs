#[derive(Clone, Debug, Default, clap::Args)]
pub struct ShowParams {
    /// If present, list the resource type for the requested object(s).
    #[arg(long)]
    pub show_kind: bool,

    /// When printing, show all labels as the last column (default hide labels column)
    #[arg(long)]
    pub show_labels: bool,

    /// If true, keep the managedFields when printing objects in JSON or YAML format.
    #[arg(long)]
    pub show_managed_fields: bool,
}
