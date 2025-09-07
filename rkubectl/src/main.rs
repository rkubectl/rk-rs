use miette::IntoDiagnostic;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

#[tokio::main]
async fn main() -> miette::Result<()> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    rk::Cli::new().exec().await.into_diagnostic()?;

    Ok(())
}
