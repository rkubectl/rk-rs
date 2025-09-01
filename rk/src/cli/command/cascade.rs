#[derive(Clone, Copy, Debug, Default, clap::ValueEnum)]
pub enum Cascade {
    #[default]
    Background,
    Foreground,
    Orphan,
}
