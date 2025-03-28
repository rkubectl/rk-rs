use super::*;

impl Output for corev1::Pod {
    fn header(&self, output: &OutputArg) -> Vec<String> {
        let header = match output {
            OutputArg::Normal => ["NAMESPACE", "NAME"].as_slice(),
            OutputArg::Wide => ["NAMESPACE", "NAME", "AGE"].as_slice(),
            _ => todo!("{output:?}"),
        };
        header.iter().map(ToString::to_string).collect()
    }

    fn data(&self, full_name: bool, output: &OutputArg) -> Vec<String> {
        let namespace = self.namespace().unwrap_or_default();
        let name = name(self, full_name);
        let age = self
            .creation_timestamp()
            .map(|t| t.0)
            .unwrap_or_default()
            .to_string();
        match output {
            OutputArg::Normal => vec![namespace, name],
            OutputArg::Wide => vec![namespace, name, age],
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
