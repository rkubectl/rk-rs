use super::*;

impl<K> Show for Created<K>
where
    K: Clone + ResourceExt + serde::Serialize,
    K::DynamicType: Default,
{
    fn header(&self, _output: OutputFormat) -> Vec<String> {
        vec![]
    }

    fn data(&self, _params: ShowParams, _output: OutputFormat) -> Vec<String> {
        let kind = K::kind(&default()).to_lowercase();
        let name = self.k.name_any();
        vec![format!("{kind}/{name} created")]
    }

    fn json(&self, params: ShowParams) -> String {
        let data = self.k.maybe_strip_managed_fields(params);
        json::to_string_pretty(&data).unwrap_or_default()
    }

    fn yaml(&self, params: ShowParams) -> String {
        let data = self.k.maybe_strip_managed_fields(params);
        yaml::to_string(&data).unwrap_or_default()
    }

    fn name(&self) -> String {
        todo!()
    }
}
