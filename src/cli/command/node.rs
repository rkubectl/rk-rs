use std::collections::BTreeMap;

use super::*;

#[derive(Clone, Debug, Subcommand)]

pub enum Node {
    ListImages,
}

impl Node {
    pub async fn exec(&self, kubectl: &Kubectl) -> kube::Result<()> {
        let lp = kubectl.list_params();
        let nodes = kubectl.nodes()?.list(&lp).await?;
        for node in nodes {
            println!("\n{}\n", node.name_any());
            node.status
                .unwrap_or_default()
                .images
                .unwrap_or_default()
                .into_iter()
                .filter_map(ImageInfo::from_container_image)
                .for_each(|info| info.print());
        }

        Ok(())
    }
}

fn _print_image(image: corev1::ContainerImage) {
    let names = image.names.unwrap_or_default();
    let size = image.size_bytes.unwrap_or_default();
    if let Some((name, aliases)) = names.split_first() {
        println!("  {name}  :  {size}");
        if !aliases.is_empty() {
            aliases.iter().for_each(|alias| println!("    {alias}"));
        }
    }
}

#[derive(Debug)]
struct ImageInfo {
    host: String,
    also: Vec<String>,
    path: String,
    hash: BTreeMap<String, String>,
    tags: Vec<String>,
    size: i64,
}

#[derive(Debug)]
struct Image {
    server: String,
    path: String,
    id: Id,
}

#[derive(Debug, PartialEq)]
enum Id {
    None,
    Hash { algo: String, value: String },
    Tag(String),
}

impl Id {
    fn hash(algo: &str, value: &str) -> Self {
        Self::Hash {
            algo: algo.to_string(),
            value: value.to_string(),
        }
    }
}

impl Image {
    fn from_name(name: impl AsRef<str>) -> Self {
        let name = name.as_ref();
        if let Some((server, path)) = name.split_once("/") {
            let server = server.to_string();
            if let Some((path, tag)) = path.rsplit_once(":") {
                if let Some((path, algo)) = path.rsplit_once("@") {
                    let path = format!("/{path}");
                    let id = Id::hash(algo, tag);
                    Self { server, path, id }
                } else {
                    let path = format!("/{path}");
                    let id = Id::Tag(tag.to_string());
                    Self { server, path, id }
                }
            } else {
                let path = format!("/{path}");
                let id = Id::None;
                Self { server, path, id }
            }
        } else {
            Self {
                server: default(),
                path: name.to_string(),
                id: Id::None,
            }
        }
    }
}

impl ImageInfo {
    fn from_container_image(image: corev1::ContainerImage) -> Option<Self> {
        let names = image.names?;
        let size = image.size_bytes?;
        let mut also = vec![];
        let mut paths = vec![];
        let mut hash = BTreeMap::new();
        let mut tags = vec![];
        for name in names {
            let image = Image::from_name(name);
            also.push(image.server);
            paths.push(image.path);
            match image.id {
                Id::None => {}
                Id::Hash { algo, value } => {
                    hash.insert(algo, value);
                }
                Id::Tag(tag) => tags.push(tag),
            }
        }

        let common = find_common(&also);
        let host = also.remove(common);
        also.retain_mut(|item| item != &host);

        paths.dedup();
        let path = paths.remove(0);

        Some(Self {
            host,
            also,
            path,
            hash,
            tags,
            size,
        })
    }

    fn print(&self) {
        println!(
            "Image: {}{}     ({})",
            self.host,
            self.path,
            size::Size::from_bytes(self.size)
        );

        if !self.also.is_empty() {
            println!(" Also in: {}", self.also.join(", "));
        }
        if !self.tags.is_empty() {
            println!(" Tags: {}", self.tags.join(", "));
        }
        if !self.hash.is_empty() {
            println!(
                " Hash: {}",
                self.hash
                    .iter()
                    .map(|(k, v)| format!("{k}={v}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
        println!()
    }
}

fn find_common(hosts: &[String]) -> usize {
    // Find the shortest item
    let (idx, _min) = hosts.iter().map(|item| item.len()).enumerate().fold(
        (0, usize::MAX),
        |(needle, min), (idx, len)| {
            if len < min { (idx, len) } else { (needle, min) }
        },
    );
    let item = &hosts[idx];
    if hosts.iter().all(|host| host.ends_with(item)) {
        idx
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tag() {
        let image = Image::from_name("cr.fluentbit.io/fluent/fluent-bit:3.2.8");
        assert_eq!(image.server, "cr.fluentbit.io");
        assert_eq!(image.path, "/fluent/fluent-bit");
        assert_eq!(image.id, Id::Tag("3.2.8".to_string()));
    }
}
