use super::*;

impl<K> Show for kube::core::ObjectList<K>
where
    K: Clone + Show,
{
    fn header(&self, output: &Output) -> Vec<String> {
        self.items.header(output)
    }

    fn data(&self, show_kind: bool, output: &Output) -> Vec<String> {
        self.items.data(show_kind, output)
    }

    fn normal(&self, show_kind: bool, output: &Output) -> Table {
        self.items.normal(show_kind, output)
    }

    fn wide(&self, show_kind: bool, output: &Output) -> Table {
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
