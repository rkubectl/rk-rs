use heck::ToTitleCase;

use super::*;

pub(crate) fn serialize_to_title_case_table(
    value: impl serde::Serialize,
) -> json::Result<tabled::Table> {
    let value = json::to_value(value)?;
    if let Some(value) = value.as_object() {
        let items = value
            .iter()
            .map(|(key, value)| (key.to_title_case(), value.as_str().unwrap_or_default()))
            .collect::<BTreeMap<_, _>>();

        Ok(tabled::builder::Builder::from(items).build())
    } else {
        Ok(tabled::builder::Builder::default().build())
    }
}

pub(crate) fn _serialize_to_table(value: impl serde::Serialize) -> json::Result<tabled::Table> {
    let value = json::to_value(value)?;
    if let Some(value) = value.as_object() {
        let iter = value.iter().map(|(key, value)| (key, value.to_string()));
        Ok(tabled::Table::new(iter))
    } else {
        Ok(tabled::Table::default())
    }
}
