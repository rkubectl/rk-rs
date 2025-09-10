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
    pub async fn exec(&self, context: &Context) -> RkResult<()> {
        match self {
            Self::Show { secret, data } => self.show(context, secret, data.as_deref()).await,
        }
    }

    async fn show(&self, context: &Context, secret: &str, data: Option<&[String]>) -> RkResult<()> {
        let _output = context.output_deprecated();
        let secret = context.kubeapi().secrets()?.get(secret).await?;
        secret
            .data
            .unwrap_or_default()
            .into_iter()
            .filter(|(key, _)| data.is_none_or(|items| items.contains(key)))
            .map(ShowSecret::new)
            .for_each(|item| item.show());

        Ok(())
    }
}

struct ShowSecret {
    name: String,
    value: String,
}

impl ShowSecret {
    fn new((name, value): (String, ByteString)) -> Self {
        let value = String::from_utf8_lossy(&value.0).to_string();
        Self { name, value }
    }

    fn show(&self) {
        println!("{}: {}", self.name, self.value);
    }
}
