use super::*;

impl<K> Output for kube::core::ObjectList<K>
where
    K: Clone + Output,
{
    fn header(&self, output: &OutputArg) -> Vec<String> {
        self.items.header(output)
    }

    fn data(&self, show_kind: bool, output: &OutputArg) -> Vec<String> {
        self.items.data(show_kind, output)
    }

    fn normal(&self, show_kind: bool, output: &OutputArg) -> Table {
        self.items.normal(show_kind, output)
    }

    fn wide(&self, show_kind: bool, output: &OutputArg) -> Table {
        self.items.wide(show_kind, output)
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
