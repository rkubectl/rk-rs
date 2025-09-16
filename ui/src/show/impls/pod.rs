use super::*;

impl Show for corev1::Pod {
    fn header(&self, output: &OutputFormat) -> Vec<String> {
        let header = match output {
            OutputFormat::Normal => {
                ["NAMESPACE", "NAME", "READY", "STATUS", "RESTARTS", "AGE"].as_slice()
            }
            OutputFormat::Wide => {
                ["NAMESPACE", "NAME", "READY", "STATUS", "RESTARTS", "AGE"].as_slice()
            }
            _ => todo!("{output:?}"),
        };
        header.iter().map(ToString::to_string).collect()
    }

    fn data(&self, params: &ShowParams, output: &OutputFormat) -> Vec<String> {
        let namespace = self.namespace().unwrap_or_default();
        let name = name(self, params);
        let total = self.total_containers();
        let ready = self.ready_containers();
        let ready = format!("{ready}/{total}");
        let status = self.kubectl_status();
        let restarts = self.restarts().to_string();
        let age = self.creation_timestamp().map(age).unwrap_or_default();
        match output {
            OutputFormat::Normal => vec![namespace, name, ready, status, restarts, age],
            OutputFormat::Wide => vec![namespace, name, ready, status, restarts, age],
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
