use super::*;

impl Show for corev1::ComponentStatus {
    fn header(&self, output: &OutputFormat) -> Vec<String> {
        let header = match output {
            OutputFormat::Normal | OutputFormat::Wide => {
                ["NAME", "STATUS", "MESSAGE", "ERROR"].as_slice()
            }
            _ => todo!("{output:?}"),
        };
        header.iter().map(ToString::to_string).collect()
    }

    fn data(&self, params: &ShowParams, output: &OutputFormat) -> Vec<String> {
        let name = name(self, params);

        let (status, message, error) = if let Some(healthy) = self.healthy() {
            (
                if healthy.is_true() {
                    "Healthy"
                } else {
                    "Unhealthy"
                },
                healthy.message().unwrap_or_default(),
                healthy.error().unwrap_or_default(),
            )
        } else {
            ("Unknown", "", "")
        };

        match output {
            OutputFormat::Normal | OutputFormat::Wide => {
                vec![name, status.into(), message.into(), error.into()]
            }
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
