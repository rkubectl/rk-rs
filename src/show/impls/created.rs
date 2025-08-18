use crate::cli::Created;

use super::*;

impl<K> Show for Created<K>
where
    K: ResourceExt,
    K::DynamicType: Default,
{
    fn header(&self, _output: &OutputFormat) -> Vec<String> {
        todo!()
    }

    fn data(&self, _params: &ShowParams, _output: &OutputFormat) -> Vec<String> {
        todo!()
    }

    fn json(&self, _params: &ShowParams) -> String {
        todo!()
    }

    fn yaml(&self, _params: &ShowParams) -> String {
        todo!()
    }

    fn name(&self) -> String {
        todo!()
    }

    fn output(&self, _namespace: bool, _params: &ShowParams, _output: &OutputFormat) -> String {
        let kind = K::kind(&default()).to_lowercase();
        let name = self.k.name_any();
        format!("{kind}/{name} created")
    }
}
