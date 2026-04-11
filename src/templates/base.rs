use serde::Deserialize;
use super::category::TemplateCategory;


#[derive(Debug, Deserialize)]
pub(crate) struct FileTemplateBase {
    #[serde(rename = "etl_template")]
    kind: TemplateCategory,

    version: u8,
    description: Option<String>
}


impl FileTemplateBase {
    #[allow(dead_code)]
    pub(crate) fn kind(&self) -> &TemplateCategory {
        &self.kind
    }


    #[allow(dead_code)]
    pub(crate) fn version(&self) -> u8 {
        self.version
    }


    #[allow(dead_code)]
    pub(crate) fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}
