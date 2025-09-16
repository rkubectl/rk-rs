use super::*;

impl Show for corev1::Namespace {
    fn header(&self, output: &OutputFormat) -> Vec<String> {
        let header = match output {
            OutputFormat::Normal | OutputFormat::Wide => ["NAME", "STATUS", "AGE"].as_slice(),
            _ => todo!("{output:?}"),
        };
        header.iter().map(ToString::to_string).collect()
    }

    fn data(&self, params: &ShowParams, output: &OutputFormat) -> Vec<String> {
        let name = name(self, params);
        let status = self.get_status().to_string();
        let age = self.creation_timestamp().map(age).unwrap_or_default();
        match output {
            OutputFormat::Normal | OutputFormat::Wide => vec![name, status, age],
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
