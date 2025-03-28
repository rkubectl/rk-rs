use super::*;

impl<K> Show for kube::core::ObjectList<K>
where
    K: Clone + Show + kube::ResourceExt + serde::Serialize,
{
    fn header(&self, output: &OutputFormat) -> Vec<String> {
        self.items.header(output)
    }

    fn data(&self, params: &ShowParams, output: &OutputFormat) -> Vec<String> {
        self.items.data(params, output)
    }

    fn normal(&self, params: &ShowParams, output: &OutputFormat) -> Table {
        self.items.normal(params, output)
    }

    fn wide(&self, params: &ShowParams, output: &OutputFormat) -> Table {
        self.items.wide(params, output)
    }

    fn yaml(&self, params: &ShowParams) -> String {
        let mut objects = self.clone();
        objects
            .iter_mut()
            .for_each(|k| k.strip_managed_fields(params));
        yaml::to_string(&objects).unwrap_or_default()
    }

    fn json(&self, params: &ShowParams) -> String {
        let mut objects = self.clone();
        objects
            .iter_mut()
            .for_each(|k| k.strip_managed_fields(params));
        json::to_string_pretty(&objects).unwrap_or_default()
    }

    fn name(&self) -> String {
        todo!()
    }
}
