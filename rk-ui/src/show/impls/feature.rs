use super::*;

impl Show for rk_features::Feature {
    fn header(&self, _output: &OutputFormat) -> Vec<String> {
        Self::headers().into_iter().map(Into::into).collect()
    }

    fn data(&self, _params: &ShowParams, _output: &OutputFormat) -> Vec<String> {
        self.fields().into_iter().map(Into::into).collect()
    }

    fn json(&self, _params: &ShowParams) -> String {
        json::to_string_pretty(self).unwrap_or_default()
    }

    fn yaml(&self, _params: &ShowParams) -> String {
        yaml::to_string(self).unwrap_or_default()
    }

    fn name(&self) -> String {
        unreachable!()
    }
}
