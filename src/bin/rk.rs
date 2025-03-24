use tracing_subscriber::{EnvFilter, fmt, prelude::*};

#[tokio::main]
async fn main() -> kube::Result<()> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let cli = rk::Cli::new();
    tracing::debug!(?cli, "rk");
    let kubectl = rk::Kubectl::new(cli.debug).await?;
    cli.exec(kubectl).await
}
