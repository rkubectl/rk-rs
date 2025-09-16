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
        let path = path.as_ref();
        // println!("Loading cache from {}", path.display());
        let start = time::Instant::now();
        let groups = load_json::<metav1::APIGroupList>(path.join("servergroups.json")).ok();
        let resources = if let Some(groups) = &groups {
            groups
                .groups
                .iter()
                .flat_map(|group| group.versions.iter().map(|version| (&group.name, version)))
                .map(|(name, version)| {
                    (
                        &version.group_version,
                        path.join(name)
                            .join(&version.version)
                            .join("serverresources.json"),
                    )
                })
                // .inspect(|(group_version, path)| {
                //     println!("Loading {group_version} from {}", path.display())
                // })
                .filter_map(|(group_version, path)| {
                    load_json(path).ok().map(|aa| (group_version.clone(), aa))
                })
                .collect()
        } else {
            default()
        };
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

fn load_json<T>(path: impl AsRef<Path>) -> io::Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let text = fs::read_to_string(path)?;
    let data = json::from_str(&text)?;
    Ok(data)
}
