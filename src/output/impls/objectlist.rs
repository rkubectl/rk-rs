use super::*;

impl<K> Output for kube::core::ObjectList<K>
where
    K: Clone + Output,
{
    fn header(&self, output: &OutputArg) -> Vec<String> {
        self.items.header(output)
    }

    fn data(&self, full_name: bool, output: &OutputArg) -> Vec<String> {
        self.items.data(full_name, output)
    }

    fn normal(&self, full_name: bool, output: &OutputArg) -> Table {
        self.items.normal(full_name, output)
    }

    fn wide(&self, full_name: bool, output: &OutputArg) -> Table {
        self.items.wide(full_name, output)
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
