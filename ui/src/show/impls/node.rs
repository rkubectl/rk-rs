use super::*;

impl Show for corev1::Node {
    fn header(&self, output: &OutputFormat) -> Vec<String> {
        let header = match output {
            OutputFormat::Normal => ["NAMESPACE", "NAME"].as_slice(),
            OutputFormat::Wide => ["NAMESPACE", "NAME", "AGE"].as_slice(),
            _ => todo!("{output:?}"),
        };
        header.iter().map(ToString::to_string).collect()
    }

    fn data(&self, params: &ShowParams, output: &OutputFormat) -> Vec<String> {
        let namespace = self.namespace().unwrap_or_default();
        let name = name(self, params);
        let age = self.creation_timestamp().map(age).unwrap_or_default();
        match output {
            OutputFormat::Normal => vec![namespace, name],
            OutputFormat::Wide => vec![namespace, name, age],
            _ => todo!("{output:?}"),
        }
    }

    fn yaml(&self, params: &ShowParams) -> String {
        let data = self.maybe_strip_managed_fields(params);
        yaml::to_string(&data).unwrap_or_default()
    }

    fn json(&self, params: &ShowParams) -> String {
        let data = self.maybe_strip_managed_fields(params);
        json::to_string_pretty(&data).unwrap_or_default()
    }

    fn name(&self) -> String {
        todo!()
    }
}

impl Show for corev1::NodeSystemInfo {
    fn header(&self, _output: &OutputFormat) -> Vec<String> {
        todo!()
    }

    fn data(&self, _params: &ShowParams, _output: &OutputFormat) -> Vec<String> {
        todo!()
    }

    fn json(&self, _params: &ShowParams) -> String {
        json::to_string(self).unwrap_or_default()
    }

    fn yaml(&self, _params: &ShowParams) -> String {
        yaml::to_string(self).unwrap_or_default()
    }

    fn name(&self) -> String {
        todo!()
    }

    fn normal(&self, _params: &ShowParams, _output: &OutputFormat) -> tabled::Table {
        convert::serialize_to_title_case_table(self).unwrap_or_default()
    }
}
