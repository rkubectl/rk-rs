use tabled::builder::Builder;

use super::*;

impl Resource {
    pub fn output(
        &self,
        objects: &[api::DynamicObject],
        namespace: bool,
        output: Output,
    ) -> tabled::Table {
        match self {
            Self::Pods => {
                if namespace {
                    let mut builder = objects
                        .iter()
                        .map(|obj| [obj.namespace().unwrap_or_default(), obj.name_any()])
                        .collect::<Builder>();
                    builder.insert_record(0, ["NAMESPACE", "NAME"]);
                    builder.build()
                } else {
                    let mut builder = objects
                        .iter()
                        .map(|obj| [obj.name_any()])
                        .collect::<Builder>();
                    builder.insert_record(0, ["NAME"]);
                    builder.build()
                }
            }
            Self::Nodes => todo!(),
            Self::ConfigMaps => todo!(),
            Self::Other(_) => todo!(),
        }
    }
}
