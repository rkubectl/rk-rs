use indexmap::IndexMap;

use super::*;

#[derive(Clone, Debug, Default)]
pub struct Cache {
    groups: Option<metav1::APIGroupList>,
    resources: IndexMap<String, metav1::APIResourceList>,
    took: time::Duration,
    _pad: (),
}

impl Cache {
    pub(super) fn try_load(self, path: impl AsRef<Path>) -> Self {
        let start = time::Instant::now();
        let cached_resources = CachedResources::new(path);
        let groups = cached_resources.load_server_groups().ok();
        let resources = groups
            .as_ref()
            .map(|groups| cached_resources.load_groups_resources(groups))
            .unwrap_or_default();
        let took = start.elapsed();
        Self {
            groups,
            resources,
            took,
            ..self
        }
    }

    pub(super) fn api_groups(&self) -> Option<metav1::APIGroupList> {
        self.groups.clone()
    }

    pub(super) fn api_resources(&self) -> Option<Vec<metav1::APIResourceList>> {
        let resources = self.resources.values().cloned().collect::<Vec<_>>();
        if resources.is_empty() {
            None
        } else {
            Some(resources)
        }
    }

    pub fn took(&self) -> time::Duration {
        self.took
    }
}

struct CachedResources {
    path: PathBuf,
}

impl CachedResources {
    const SERVER_GROUPS: &'static str = "servergroups.json";
    const SERVER_RESOURCES: &'static str = "serverresources.json";

    fn new(path: impl AsRef<Path>) -> Self {
        let path = path.as_ref().to_path_buf();
        trace!(from = %path.display(), "Loading cached resources");
        Self { path }
    }

    fn load_server_groups(&self) -> io::Result<metav1::APIGroupList> {
        let path = self.path.join(Self::SERVER_GROUPS);
        load_json(path)
    }

    fn load_groups_resources(
        &self,
        groups: &metav1::APIGroupList,
    ) -> IndexMap<String, metav1::APIResourceList> {
        groups
            .groups
            .iter()
            .flat_map(|group| group.versions.iter())
            .filter_map(|version| self.load_server_group_version_resources(version).ok())
            .map(|arl| (arl.group_version.clone(), arl))
            .collect()
    }

    fn load_server_group_version_resources(
        &self,
        version: &metav1::GroupVersionForDiscovery,
    ) -> io::Result<metav1::APIResourceList> {
        let path = self
            .path
            .join(&version.group_version)
            .join(Self::SERVER_RESOURCES);
        load_json(path)
    }
}

#[tracing::instrument(level = "trace", err)]
fn load_json<T>(path: PathBuf) -> io::Result<T>
where
    T: serde::de::DeserializeOwned,
{
    trace!("Loading cached data");
    let text = fs::read_to_string(path)?;
    let data = json::from_str(&text)?;
    Ok(data)
}
