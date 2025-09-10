use std::fmt;

use k8s_openapi_ext as k8s;
use kube::Resource;
use kube::ResourceExt;
use kube::api;
use serde_json as json;
use serde_yaml as yaml;

use rk_ext::NamespaceGetExt2;
use rk_ext::PodGetExt2;

use k8s::ComponentConditionGetExt;
use k8s::ComponentStatusGetExt;

use k8s::authenticationv1;
use k8s::authorizationv1;
use k8s::corev1;
use k8s::metav1;

pub use show::Show;
pub use show::ShowParams;

mod show;

#[derive(Debug)]
pub struct Ui {
    namespace: bool,
    output: OutputFormat,
}

impl Ui {
    pub fn new(output: OutputFormat) -> Self {
        Self {
            namespace: true,
            output,
        }
    }

    pub fn show<T>(&self, item: T, params: &ShowParams)
    where
        T: Show,
    {
        item.output(self.namespace, params, &self.output);
    }

    pub fn print(&self, text: impl fmt::Display) {
        println!("{text}");
    }

    pub fn output_deprecated(&self) -> &OutputFormat {
        &self.output
    }

    pub fn not_implemented(&self, item: impl fmt::Debug) {
        println!("{item:?} not implemented yet");
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

    pub fn is_wide(&self) -> bool {
        matches!(self, Self::Wide)
    }
}

#[derive(Debug)]
pub struct Created<K> {
    // pub resource: CreateResource,
    pub k: K,
}

fn default<T: Default>() -> T {
    T::default()
}
