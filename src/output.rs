use tabled::Table;
use tabled::settings::Remove;
use tabled::settings::Style;
use tabled::settings::location::ByColumnName;

// use tabled::Tabled;

use super::*;

mod impls;

pub trait Output {
    fn header(&self, output: &OutputArg) -> Vec<String>;
    fn data(&self, full_name: bool, output: &OutputArg) -> Vec<String>;
    fn yaml(&self) -> String;
    fn json(&self) -> String;
    fn name(&self) -> String;

    fn normal(&self, full_name: bool, output: &OutputArg) -> Table {
        tabled::builder::Builder::from_iter([self.header(output), self.data(full_name, output)])
            .build()
    }
    fn wide(&self, full_name: bool, output: &OutputArg) -> Table {
        tabled::builder::Builder::from_iter([self.header(output), self.data(full_name, output)])
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

    fn output(&self, namespace: bool, full_name: bool, output: &OutputArg) -> String {
        match output {
            OutputArg::Normal => {
                let mut table = self.normal(full_name, output);
                if namespace {
                    table.with(Style::blank());
                } else {
                    table
                        .with(Style::blank())
                        .with(Remove::column(ByColumnName::new("NAMESPACE")));
                }
                table.to_string()
            }
            OutputArg::Json => self.json(),
            OutputArg::Yaml => self.yaml(),
            OutputArg::Name => self.name().to_string(),
            OutputArg::GoTemplate => self.go_template(),
            OutputArg::GoTemplateFile => self.go_template_file(),
            OutputArg::Template => self.template(),
            OutputArg::TemplateFile => self.template_file(),
            OutputArg::JsonPath => self.json_path(),
            OutputArg::JsonPathAsJson => self.json_path_as_json(),
            OutputArg::JsonPathFile => self.json_path_file(),
            OutputArg::CustomColumns => self.custom_columns(),
            OutputArg::CustomColumnsFile => self.custom_columns_file(),
            OutputArg::Wide => {
                let mut table = self.wide(full_name, output);
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
pub enum OutputArg {
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

// impl Output {
//     pub fn as_str(&self) -> &'static str {
//         match self {
//             Self::Normal => "",
//             Self::Json => "json",
//             Self::Yaml => "yaml",
//             Self::Name => ,
//             Self::GoTemplate => todo!(),
//             Self::GoTemplateFile => todo!(),
//             Self::Template => todo!(),
//             Self::TemplateFile => todo!(),
//             Self::JsonPath => todo!(),
//             Self::JsonPathAsJson => todo!(),
//             Self::JsonPathFile => todo!(),
//             Self::CustomColumns => todo!(),
//             Self::CustomColumnsFile => todo!(),
//             Self::Wide => todo!(),
//         }
//     }
// }

impl OutputArg {
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

impl<T> Output for Vec<T>
where
    T: Output,
{
    fn header(&self, output: &OutputArg) -> Vec<String> {
        todo!("Not supported on Vec<T> for {output:?}")
    }

    fn data(&self, _full_name: bool, output: &OutputArg) -> Vec<String> {
        todo!("Not supported on Vec<T> for {output:?}")
    }

    fn normal(&self, full_name: bool, output: &OutputArg) -> tabled::Table {
        let header = self.first().map(|t| t.header(output));
        let data = self.iter().map(|t| t.data(full_name, output));
        let builder = header
            .into_iter()
            .chain(data)
            .collect::<tabled::builder::Builder>();
        builder.build()
    }

    fn wide(&self, full_name: bool, output: &OutputArg) -> Table {
        let header = self.first().map(|t| t.header(output));
        let data = self.iter().map(|t| t.data(full_name, output));
        let builder = header
            .into_iter()
            .chain(data)
            .collect::<tabled::builder::Builder>();
        builder.build()
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

fn name<K>(object: &K, full_name: bool) -> String
where
    K: kube::Resource,
    K::DynamicType: Default,
{
    if full_name {
        format!("{}/{}", K::kind(&default()), object.name_any())
    } else {
        object.name_any()
    }
}
