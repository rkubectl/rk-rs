use super::*;

impl Show for corev1::ConfigMap {
    fn header(&self, output: OutputFormat) -> Vec<String> {
        let header = match output {
            OutputFormat::Normal => ["NAMESPACE", "NAME", "DATA", "AGE"].as_slice(),
            OutputFormat::Wide => ["NAMESPACE", "NAME", "DATA", "AGE"].as_slice(),
            _ => todo!("{output:?}"),
        };
        header.iter().map(ToString::to_string).collect()
    }

    fn data(&self, params: ShowParams, output: OutputFormat) -> Vec<String> {
        let namespace = self.namespace().unwrap_or_default();
        let name = name(self, params);
        let data = self
            .data
            .as_ref()
            .map(|data| data.len())
            .unwrap_or_default();
        let binary_data = self
            .binary_data
            .as_ref()
            .map(|data| data.len())
            .unwrap_or_default();
        let data = format!("{}", data + binary_data);
        let age = self.creation_timestamp().map(age).unwrap_or_default();
        match output {
            OutputFormat::Normal => vec![namespace, name, data, age],
            OutputFormat::Wide => vec![namespace, name, data, age],
            _ => todo!("{output:?}"),
        }
    }

    fn yaml(&self, params: ShowParams) -> String {
        let data = self.maybe_strip_managed_fields(params);
        yaml::to_string(&data).unwrap_or_default()
    }

    fn json(&self, params: ShowParams) -> String {
        let data = self.maybe_strip_managed_fields(params);
        json::to_string_pretty(&data).unwrap_or_default()
    }

    fn name(&self) -> String {
        todo!()
    }
}
