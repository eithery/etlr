use serde::Deserialize;
use super::category::TemplateCategory;
use super::file::FileInfoTemplate;
use super::layout::RecordLayoutTemplate;
use super::traits::FileTemplate;


#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub(crate) struct FileExportTemplate {
    #[serde(rename = "etl_template")]
    kind: TemplateCategory,

    version: u8,

    file: FileInfoTemplate,

    #[serde(default)]
    description: String,

    layout: RecordLayoutTemplate
}


impl FileTemplate for FileExportTemplate {
    const TEMPLATES_ROOT: &'static str = "export";
}
