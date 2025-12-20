use serde::Deserialize;
use super::category::TemplateCategory;
use super::traits::FileTemplate;


#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub(crate) struct FileExportTemplate {
    #[serde(rename = "etl_template")]
    kind: TemplateCategory,

    version: u8,

    // file: String,
    description: String
    // layout: String
}


impl FileTemplate for FileExportTemplate {
    const TEMPLATES_ROOT: &'static str = "export";
}
