use std::collections::BTreeMap;
use std::collections::BTreeSet;

use super::*;

#[derive(Debug)]
pub(super) struct ImageInfo {
    host: String,
    also: Vec<String>,
    path: String,
    hash: BTreeMap<String, String>,
    tags: Vec<String>,
    size: i64,
}

impl ImageInfo {
    pub(super) fn from_container_image(image: corev1::ContainerImage) -> Option<Self> {
        let names = image.names?;
        let size = image.size_bytes?;
        let mut also = vec![];
        let mut paths = BTreeSet::<String>::new();
        let mut hash = BTreeMap::new();
        let mut tags = vec![];
        for name in names {
            let image = Image::from_name(name);
            also.push(image.server);
            paths.insert(image.path);
            match image.id {
                Id::None => {}
                Id::Hash(algo, value) => {
                    hash.insert(algo, value);
                }
                Id::Tag(tag) => tags.push(tag),
            }
        }

        let path = paths.pop_first()?;
        let common = find_common(&also);
        let host = also.remove(common);
        also.retain_mut(|item| item != &host);

        Some(Self {
            host,
            also,
            path,
            hash,
            tags,
            size,
        })
    }

    pub(super) fn print(&self) {
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

#[derive(Debug)]
struct Image {
    server: String,
    path: String,
    id: Id,
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

#[derive(Debug, PartialEq)]
enum Id {
    None,
    Hash(String, String),
    Tag(String),
}

impl Id {
    fn hash(algo: &str, value: &str) -> Self {
        Self::Hash(algo.to_string(), value.to_string())
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
