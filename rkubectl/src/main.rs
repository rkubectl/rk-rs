use miette::IntoDiagnostic;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

use rkubectl_app as app;

#[tokio::main]
async fn main() -> miette::Result<()> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    app::Cli::new().exec().await.into_diagnostic()?;

    Ok(())
}
