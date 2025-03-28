use super::*;

impl<K> Show for kube::core::ObjectList<K>
where
    K: Clone + Show,
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

    fn yaml(&self) -> String {
        todo!()
    }

    fn json(&self) -> String {
        todo!()
    }

    fn name(&self) -> String {
        todo!()
    }
}
