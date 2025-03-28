use kube::ResourceExt;

use super::*;

// [(-o|--output=)json|yaml|name|go-template|go-template-file|template|templatefile|jsonpath|jsonpath-as-json|jsonpath-file|custom-columns|custom-columns-file|wide]
#[derive(Clone, Copy, Debug, Default, PartialEq, ValueEnum)]
pub enum Output {
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

impl Output {
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
