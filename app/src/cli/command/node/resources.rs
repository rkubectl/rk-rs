use k8s::resource::Quantity;

use super::*;

#[derive(Debug, Default)]
pub(super) struct Resources {
    resources: Vec<String>,
    values: Vec<String>,
}

impl Resources {
    pub(super) fn from_nodes(nodes: Vec<corev1::Node>, capacity: bool) -> Vec<Self> {
        let resources = nodes.iter().filter_map(|node| {
            if capacity {
                Self::capacity(node)
            } else {
                Self::allocatable(node)
            }
        });
        let resources = resource_names(resources);
        nodes
            .into_iter()
            .map(|node| Self::from_node(node, capacity, &resources))
            .collect()
    }

    fn from_node(node: corev1::Node, capacity: bool, resources: &[String]) -> Self {
        let name = node.name_any();
        let values = Self::node_resources(node, capacity, resources);
        let resources = iter::once("NODE".to_string())
            .chain(resources.iter().cloned())
            .collect();
        let values = iter::once(name).chain(values).collect();
        Self { resources, values }
    }

    fn node_resources(node: corev1::Node, capacity: bool, names: &[String]) -> Vec<String> {
        let status = node.status.unwrap_or_default();
        let mut resources = if capacity {
            status.capacity.unwrap_or_default()
        } else {
            status.allocatable.unwrap_or_default()
        };

        names
            .iter()
            .map(|name| {
                resources
                    .remove(name)
                    .map_or_else(|| String::from("-"), |qx| qx.0)
            })
            .collect()
    }

    fn capacity(node: &corev1::Node) -> Option<&BTreeMap<String, Quantity>> {
        node.status.as_ref()?.capacity.as_ref()
    }

    fn allocatable(node: &corev1::Node) -> Option<&BTreeMap<String, Quantity>> {
        node.status.as_ref()?.allocatable.as_ref()
    }
}

fn resource_names<'a>(
    resources: impl Iterator<Item = &'a BTreeMap<String, Quantity>>,
) -> Vec<String> {
    let mut names = resources
        .flat_map(|resource| resource.keys())
        .cloned()
        .collect::<BTreeSet<_>>();
    // Standard resources first, in a specific order.
    let std_resources = [
        "cpu",
        "memory",
        "ephemeral-storage",
        "pods",
        "hugepages-1Gi",
        "hugepages-2Mi",
    ];
    let mut ordered = std_resources
        .into_iter()
        .fold(Vec::new(), |mut ordered, resource| {
            ordered.extend(names.take(resource));
            ordered
        });

    // Take the rest in alphabetical order.
    ordered.extend(names);
    ordered
}

impl Show for Resources {
    fn header(&self, _output: OutputFormat) -> Vec<String> {
        self.resources.clone()
    }

    fn data(&self, _params: ShowParams, _output: OutputFormat) -> Vec<String> {
        self.values.clone()
    }

    fn normal(&self, _params: ShowParams, _output: OutputFormat) -> tabled::Table {
        todo!("Not supported on Resources for normal")
    }

    fn wide(&self, _params: ShowParams, _output: OutputFormat) -> tabled::Table {
        todo!("Not supported on Resources for wide")
    }

    fn yaml(&self, _params: ShowParams) -> String {
        todo!("Not supported on Resources for yaml")
    }

    fn json(&self, _params: ShowParams) -> String {
        todo!("Not supported on Resources for json")
    }

    fn name(&self) -> String {
        String::from("resources")
    }
}
