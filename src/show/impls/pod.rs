use super::*;

impl Show for corev1::Pod {
    fn header(&self, output: &Output) -> Vec<String> {
        let header = match output {
            Output::Normal => ["NAMESPACE", "NAME"].as_slice(),
            Output::Wide => ["NAMESPACE", "NAME", "AGE"].as_slice(),
            _ => todo!("{output:?}"),
        };
        header.iter().map(ToString::to_string).collect()
    }

    fn data(&self, show_kind: bool, output: &Output) -> Vec<String> {
        let namespace = self.namespace().unwrap_or_default();
        let name = name(self, show_kind);
        let age = self
            .creation_timestamp()
            .map(|t| t.0)
            .unwrap_or_default()
            .to_string();
        match output {
            Output::Normal => vec![namespace, name],
            Output::Wide => vec![namespace, name, age],
            _ => todo!("{output:?}"),
        }
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
