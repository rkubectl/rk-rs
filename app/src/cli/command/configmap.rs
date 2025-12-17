use k8s::openapi::ByteString;

use super::*;

#[derive(Clone, Debug, Subcommand)]
pub enum ConfigMap {
    Show {
        configmap: String,
        data: Option<Vec<String>>,
    },
}

impl ConfigMap {
    pub async fn exec(&self, context: &Context) -> RkResult<()> {
        match self {
            Self::Show { configmap, data } => self.show(context, configmap, data.as_deref()).await,
        }
    }

    async fn show(
        &self,
        context: &Context,
        configmap: &str,
        data: Option<&[String]>,
    ) -> RkResult<()> {
        let params = default();
        let configmap = context.kubeapi().configmaps()?.get(configmap).await?;
        let items = configmap
            .binary_data
            .unwrap_or_default()
            .into_iter()
            .filter(|(key, _)| data.is_none_or(|items| items.contains(key)))
            .collect::<Vec<_>>();
        decode_binary_items(items);

        let items = configmap
            .data
            .unwrap_or_default()
            .into_iter()
            .filter(|(key, _)| data.is_none_or(|items| items.contains(key)))
            .collect::<Vec<_>>();
        if items.len() == 1 {
            // context.ui().show(items[0].clone(), &params);
            println!("{}", items[0].1);
        } else {
            context.ui().show(items, params);
            // decode_string_items(items);
        }

        Ok(())
    }
}

fn _decode_string_items(items: Vec<(String, String)>) {
    if items.len() == 1 {
        println!("{}", items[0].1);
        // context.ui().show(items[0], &params)
    } else {
        items
            .into_iter()
            .for_each(|(key, item)| println!("{key}: {item}"))
    }
}

fn decode_binary_items(items: Vec<(String, ByteString)>) {
    let single_item = items.len() == 1;
    items
        .into_iter()
        // .map(|(key, value)| (key, String::from_utf8_lossy(&value.0)))
        .for_each(|(key, value)| {
            let item = String::from_utf8_lossy(&value.0);
            if single_item {
                println!("{item}")
            } else {
                println!("{key}: {item}")
            }
        });
}
