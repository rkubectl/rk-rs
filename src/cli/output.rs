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
