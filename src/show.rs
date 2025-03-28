use serde_json as json;
use serde_yaml as yaml;
use tabled::Table;
use tabled::settings::Remove;
use tabled::settings::Style;
use tabled::settings::location::ByColumnName;

use super::*;

pub use params::ShowParams;

mod impls;
mod params;

pub trait Show {
    fn header(&self, output: &OutputFormat) -> Vec<String>;
    fn data(&self, params: &ShowParams, output: &OutputFormat) -> Vec<String>;
    fn yaml(&self, params: &ShowParams) -> String;
    fn json(&self, params: &ShowParams) -> String;
    fn name(&self) -> String;

    fn normal(&self, params: &ShowParams, output: &OutputFormat) -> Table {
        tabled::builder::Builder::from_iter([self.header(output), self.data(params, output)])
            .build()
    }
    fn wide(&self, params: &ShowParams, output: &OutputFormat) -> Table {
        tabled::builder::Builder::from_iter([self.header(output), self.data(params, output)])
            .build()
    }

    fn go_template(&self) -> String {
        todo!("This method is not supported yet")
    }

    fn go_template_file(&self) -> String {
        todo!("This method is not supported yet")
    }

    fn template(&self) -> String {
        todo!("This method is not supported yet")
    }

    fn template_file(&self) -> String {
        todo!("This method is not supported yet")
    }

    fn json_path(&self) -> String {
        todo!("This method is not supported yet")
    }

    fn json_path_as_json(&self) -> String {
        todo!("This method is not supported yet")
    }

    fn json_path_file(&self) -> String {
        todo!("This method is not supported yet")
    }

    fn custom_columns(&self) -> String {
        todo!("This method is not supported yet")
    }

    fn custom_columns_file(&self) -> String {
        todo!("This method is not supported yet")
    }

    fn output(&self, namespace: bool, params: &ShowParams, output: &OutputFormat) -> String {
        match output {
            OutputFormat::Normal => {
                let mut table = self.normal(params, output);
                if namespace {
                    table.with(Style::blank());
                } else {
                    table
                        .with(Style::blank())
                        .with(Remove::column(ByColumnName::new("NAMESPACE")));
                }
                table.to_string()
            }
            OutputFormat::Json => self.json(params),
            OutputFormat::Yaml => self.yaml(params),
            OutputFormat::Name => self.name().to_string(),
            OutputFormat::GoTemplate => self.go_template(),
            OutputFormat::GoTemplateFile => self.go_template_file(),
            OutputFormat::Template => self.template(),
            OutputFormat::TemplateFile => self.template_file(),
            OutputFormat::JsonPath => self.json_path(),
            OutputFormat::JsonPathAsJson => self.json_path_as_json(),
            OutputFormat::JsonPathFile => self.json_path_file(),
            OutputFormat::CustomColumns => self.custom_columns(),
            OutputFormat::CustomColumnsFile => self.custom_columns_file(),
            OutputFormat::Wide => {
                let mut table = self.wide(params, output);
                if namespace {
                    table.with(Style::blank());
                } else {
                    table
                        .with(Style::blank())
                        .with(Remove::column(ByColumnName::new("NAMESPACE")));
                }
                table.to_string()
            }
        }
    }
}

// [(-o|--output=)json|yaml|name|go-template|go-template-file|template|templatefile|jsonpath|jsonpath-as-json|jsonpath-file|custom-columns|custom-columns-file|wide]
#[derive(Clone, Copy, Debug, Default, PartialEq, clap::ValueEnum)]
pub enum OutputFormat {
    #[default]
    #[value(skip)]
    Normal,
    Json,
    Yaml,
    Name,
    GoTemplate,
    GoTemplateFile,
    Template,
    TemplateFile,
    JsonPath,
    JsonPathAsJson,
    JsonPathFile,
    CustomColumns,
    CustomColumnsFile,
    Wide,
}

impl OutputFormat {
    pub fn objects(&self, objects: &[api::DynamicObject]) {
        objects.iter().for_each(|object| self.object(object));
    }

    pub fn object(&self, object: &api::DynamicObject) {
        let name = object.name_any();
        match self {
            Self::Normal => {
                let types = object.types.as_ref();
                println!("{types:?}/{name}");
            }
            Self::Json => todo!(),
            Self::Yaml => todo!(),
            Self::Name => todo!(),
            Self::GoTemplate => todo!(),
            Self::GoTemplateFile => todo!(),
            Self::Template => todo!(),
            Self::TemplateFile => todo!(),
            Self::JsonPath => todo!(),
            Self::JsonPathAsJson => todo!(),
            Self::JsonPathFile => todo!(),
            Self::CustomColumns => todo!(),
            Self::CustomColumnsFile => todo!(),
            Self::Wide => {
                let kind = object
                    .types
                    .as_ref()
                    .map(|types| types.kind.as_str())
                    .unwrap_or_default();
                let data = object
                    .data
                    .as_array()
                    .map(|arr| arr.len())
                    .unwrap_or_default();
                let age = object
                    .meta()
                    .creation_timestamp
                    .clone()
                    .map(|time| time.0)
                    .unwrap_or_default();
                println!("{kind}/{name} {data} {age}");
            }
        }
    }
}

impl<T> Show for Vec<T>
where
    T: Show,
{
    fn header(&self, output: &OutputFormat) -> Vec<String> {
        todo!("Not supported on Vec<T> for {output:?}")
    }

    fn data(&self, _params: &ShowParams, output: &OutputFormat) -> Vec<String> {
        todo!("Not supported on Vec<T> for {output:?}")
    }

    fn normal(&self, params: &ShowParams, output: &OutputFormat) -> tabled::Table {
        let header = self.first().map(|t| t.header(output));
        let data = self.iter().map(|t| t.data(params, output));
        let builder = header
            .into_iter()
            .chain(data)
            .collect::<tabled::builder::Builder>();
        builder.build()
    }

    fn wide(&self, params: &ShowParams, output: &OutputFormat) -> Table {
        let header = self.first().map(|t| t.header(output));
        let data = self.iter().map(|t| t.data(params, output));
        let builder = header
            .into_iter()
            .chain(data)
            .collect::<tabled::builder::Builder>();
        builder.build()
    }

    fn yaml(&self, params: &ShowParams) -> String {
        self.iter()
            .map(|item| item.yaml(params))
            .collect::<Vec<_>>()
            .join("")
    }

    fn json(&self, params: &ShowParams) -> String {
        self.iter()
            .map(|item| item.json(params))
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn name(&self) -> String {
        todo!()
    }
}

fn name<K>(object: &K, params: &ShowParams) -> String
where
    K: kube::Resource,
    K::DynamicType: Default,
{
    let name = object.name_any();
    if params.show_kind {
        let kind = K::kind(&default()).to_lowercase();
        format!("{kind}/{name}")
    } else {
        name
    }
}

fn age(time: metav1::Time) -> String {
    let delta = k8s_openapi::chrono::Utc::now().signed_duration_since(time.0);
    human_time(delta)
}

/// Mimics k8s humantime printer
fn human_time(delta: k8s_openapi::chrono::TimeDelta) -> String {
    let days = delta.num_days();
    let hours = delta.num_hours();
    let seconds = delta.num_seconds();
    let minutes = delta.num_minutes();

    if seconds < -1 {
        "<invalid>".to_string()
    } else if seconds == 0 {
        "0s".to_string()
    } else if seconds < 60 * 2 {
        // 2 minutes
        format!("{seconds}s")
    } else if minutes < 10 {
        let seconds = seconds % 60; // seconds in last minute
        if seconds == 0 {
            format!("{minutes}m")
        } else {
            format!("{minutes}m{seconds}s")
        }
    } else if hours < 8 {
        let minutes = minutes % 60; // minutes in last hour
        if minutes == 0 {
            format!("{hours}h")
        } else {
            format!("{hours}h{minutes}m")
        }
    } else if hours < 48 {
        format!("{hours}h")
    } else if days < 8 {
        let hours = hours % 24;
        if hours == 0 {
            format!("{days}d")
        } else {
            format!("{days}d{hours}h")
        }
    } else if days < 2 * 365 {
        format!("{days}d")
    } else if days < 8 * 365 {
        let years = days / 365;
        let days = days % 365;
        if days == 0 {
            format!("{years}y")
        } else {
            format!("{years}y{days}d")
        }
    } else {
        let years = days / 365;
        format!("{years}y")
    }
}
