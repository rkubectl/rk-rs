use super::*;

impl Show for authenticationv1::SelfSubjectReview {
    fn header(&self, _output: OutputFormat) -> Vec<String> {
        ["ATTRIBUTE", "VALUE"]
            .iter()
            .map(ToString::to_string)
            .collect()
    }

    fn data(&self, _params: ShowParams, _output: OutputFormat) -> Vec<String> {
        unreachable!()
    }

    fn json(&self, params: ShowParams) -> String {
        let data = self.maybe_strip_managed_fields(params);
        json::to_string_pretty(&data).unwrap_or_default()
    }

    fn yaml(&self, params: ShowParams) -> String {
        let data = self.maybe_strip_managed_fields(params);
        yaml::to_string(&data).unwrap_or_default()
    }

    fn name(&self) -> String {
        unreachable!()
    }

    fn normal(&self, _params: ShowParams, output: OutputFormat) -> Table {
        let iter = [
            Some(self.header(output)),
            username(self),
            uid(self),
            groups(self),
        ]
        .into_iter()
        .flatten()
        .chain(extra(self));
        tabled::builder::Builder::from_iter(iter).build()
    }
}

fn userinfo(ssr: &authenticationv1::SelfSubjectReview) -> Option<&authenticationv1::UserInfo> {
    ssr.status.as_ref()?.user_info.as_ref()
}

fn username(ssr: &authenticationv1::SelfSubjectReview) -> Option<Vec<String>> {
    userinfo(ssr)?
        .username
        .as_ref()
        .map(|name| vec!["Username".to_string(), name.to_string()])
}

fn uid(ssr: &authenticationv1::SelfSubjectReview) -> Option<Vec<String>> {
    userinfo(ssr)?
        .uid
        .as_ref()
        .map(|uid| vec!["Uid".to_string(), uid.to_string()])
}

fn groups(ssr: &authenticationv1::SelfSubjectReview) -> Option<Vec<String>> {
    userinfo(ssr)?
        .groups
        .as_ref()
        .map(|groups| vec!["Groups".to_string(), format!("[{}]", groups.join(","))])
}

fn extra(ssr: &authenticationv1::SelfSubjectReview) -> Vec<Vec<String>> {
    userinfo(ssr)
        .and_then(|info| info.extra.as_ref())
        .iter()
        .flat_map(|extra| extra.iter())
        .map(|(key, values)| vec![format!("Extra: {key}"), format!("[{}]", values.join(","))])
        .collect()
}
