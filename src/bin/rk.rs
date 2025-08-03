use tracing_subscriber::{EnvFilter, fmt, prelude::*};

#[tokio::main]
async fn main() -> Result<(), ()> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    rk::Cli::new()
        .exec()
        .await
        .map_err(|error| println!("{}", report_kube_error(&error)))?;

    Ok(())
}

fn report_kube_error(error: &kube::Error) -> String {
    match error {
        kube::Error::Api(kube::error::ErrorResponse {
            message, reason, ..
        }) => {
            format!("Error from server ({reason}): {message}")
        }
        kube::Error::HyperError(error) => format!("hyper error: {error}"),
        kube::Error::Service(error) => format!("Service error: {error}"),
        kube::Error::ProxyProtocolUnsupported { proxy_url } => {
            format!("Unsupported proxy protocol: {proxy_url}")
        }
        kube::Error::ProxyProtocolDisabled {
            proxy_url,
            protocol_feature,
        } => format!("Proxy protocol disabled: {proxy_url} {protocol_feature}"),
        error @ kube::Error::FromUtf8(_) => format!("{error}"),
        kube::Error::LinesCodecMaxLineLengthExceeded => {
            "LinesCodecMaxLineLengthExceeded".to_string()
        }
        kube::Error::ReadEvents(error) => format!("{error}"),
        kube::Error::HttpError(error) => format!("{error}"),
        kube::Error::SerdeError(error) => format!("{error}"),
        kube::Error::BuildRequest(error) => format!("{error}"),
        kube::Error::InferConfig(infer_config_error) => format!("{infer_config_error}"),
        kube::Error::Discovery(discovery_error) => format!("{discovery_error}"),
        kube::Error::RustlsTls(error) => format!("{error}"),
        kube::Error::TlsRequired => "TLS required".to_string(),
        kube::Error::Auth(error) => format!("{error}"),
    }
}
