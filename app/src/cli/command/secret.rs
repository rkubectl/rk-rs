use ext::Base64Decode;
use flate2::read::GzDecoder;
use k8s::SecretExt;
use k8s::openapi::ByteString;

use super::*;

#[derive(Clone, Debug, Subcommand)]
pub enum Secret {
    Show {
        secret: String,
        data: Option<Vec<String>>,
    },
}

impl Secret {
    const SECRET_TYPE_HELM_SH_RELEASE_V1: &'static str = "helm.sh/release.v1";

    pub async fn exec(&self, context: &Context) -> RkResult<()> {
        match self {
            Self::Show { secret, data } => self.show(context, secret, data.as_deref()).await,
        }
    }

    async fn show(&self, context: &Context, secret: &str, data: Option<&[String]>) -> RkResult<()> {
        let _output = context.output_deprecated();
        let secret = context.kubeapi().secrets()?.get(secret).await?;
        let r#type = secret.type_.as_deref().unwrap_or("Opaque");
        let items = secret
            .data
            .unwrap_or_default()
            .into_iter()
            .filter(|(key, _)| data.is_none_or(|items| items.contains(key)))
            .collect::<Vec<_>>();
        decode_items(r#type, items);

        Ok(())
    }
}

fn decode_items(r#type: &str, items: Vec<(String, ByteString)>) {
    let single_item = items.len() == 1;
    items
        .into_iter()
        .map(|(key, value)| (key, decode(r#type, value)))
        .for_each(|(key, item)| {
            if single_item {
                println!("{item}")
            } else {
                println!("{key}: {item}")
            }
        });
}

fn decode(r#type: &str, value: ByteString) -> String {
    match r#type {
        Secret::SECRET_TYPE_HELM_SH_RELEASE_V1 => decode_helm_sh_release_v1(value),
        corev1::Secret::SECRET_TYPE_DOCKER_CONFIG_JSON => decode_docker_config_json(value),
        corev1::Secret::SECRET_TYPE_SERVICE_ACCOUNT_TOKEN => decode_service_account_token(value),
        corev1::Secret::SECRET_TYPE_OPAQUE => decode_generic(value),
        _other => decode_generic(value),
    }
}

fn decode_generic(item: ByteString) -> String {
    String::from_utf8_lossy(&item.0).to_string()
}

fn decode_docker_config_json(item: ByteString) -> String {
    if let Ok(value) = json::from_slice::<json::Value>(&item.0) {
        json::to_string_pretty(&value).unwrap_or_default()
    } else {
        String::from_utf8_lossy(&item.0).to_string()
    }
}

fn decode_service_account_token(item: ByteString) -> String {
    String::from_utf8_lossy(&item.0).to_string()
}

fn decode_helm_sh_release_v1(item: ByteString) -> String {
    use io::Read;
    let bytes: Vec<u8> = item.decode().unwrap_or_default();
    let mut decoder = GzDecoder::new(bytes.as_slice());
    let mut text = String::with_capacity(bytes.len() * 3);
    decoder.read_to_string(&mut text).unwrap_or(0);
    text
}
