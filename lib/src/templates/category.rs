use serde::Deserialize;


#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub(crate) enum TemplateCategory {
    #[serde(rename = ":import")]
    Import,

    #[serde(rename = ":export")]
    Export
}
