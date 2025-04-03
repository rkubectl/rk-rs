use super::*;

impl Show for authorizationv1::SelfSubjectAccessReview {
    fn header(&self, _output: &OutputFormat) -> Vec<String> {
        vec![]
    }

    fn data(&self, _params: &ShowParams, _output: &OutputFormat) -> Vec<String> {
        unreachable!()
    }

    fn json(&self, params: &ShowParams) -> String {
        let data = self.strip_managed_fields(params);
        json::to_string_pretty(&data).unwrap_or_default()
    }

    fn yaml(&self, params: &ShowParams) -> String {
        let data = self.strip_managed_fields(params);
        yaml::to_string(&data).unwrap_or_default()
    }

    fn name(&self) -> String {
        unreachable!()
    }

    fn output(&self, _namespace: bool, params: &ShowParams, output: &OutputFormat) -> String {
        match output {
            OutputFormat::Json => self.json(params),
            OutputFormat::Yaml => self.yaml(params),
            _ => {
                let authorizationv1::SubjectAccessReviewStatus {
                    allowed,
                    denied,
                    evaluation_error,
                    reason,
                } = self.status.clone().unwrap_or_default();

                let reason = reason
                    .map(|reason| format!(" - {reason}"))
                    .unwrap_or_default();
                if allowed {
                    format!("yes{}", if output.is_wide() { &reason } else { "" })
                } else {
                    let denied = if denied.unwrap_or_default() {
                        " (denied)"
                    } else {
                        ""
                    };
                    let evaluation_error = evaluation_error
                        .map(|error| format!(" - {error}"))
                        .unwrap_or_default();
                    format!("no{denied}{reason}{evaluation_error}")
                }
            }
        }
    }
}
