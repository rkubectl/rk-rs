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

    fn data(&self, params: &ShowParams, output: &Output) -> Vec<String> {
        let namespace = self.namespace().unwrap_or_default();
        let name = name(self, params);
        let age = self.creation_timestamp().map(age).unwrap_or_default();
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
