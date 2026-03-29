use serde::Deserialize;
use super::category::TemplateCategory;
use super::file::FileInfoTemplate;
use super::format::FileFormat;
use super::layout::RecordLayoutTemplate;
use super::traits::FileTemplate;


#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub(crate) struct FileExportTemplate {
    #[serde(rename = "etl_template")]
    kind: TemplateCategory,

    version: u8,
    file: FileInfoTemplate,
    description: Option<String>,
    layout: RecordLayoutTemplate
}


impl FileTemplate for FileExportTemplate {
    const TEMPLATES_ROOT: &'static str = "export";
}


impl FileExportTemplate {
    #[allow(dead_code)]
    pub(crate) fn version(&self) -> u8 {
        self.version
    }


    #[allow(dead_code)]
    pub(crate) fn file_type(&self) -> &str {
        self.file.file_type()
    }


    #[allow(dead_code)]
    pub(crate) fn file_category(&self) -> &str {
        self.file.category()
    }


    #[allow(dead_code)]
    pub(crate) fn outbound_file_name(&self) -> &str {
        self.file.file_name()
    }


    #[allow(dead_code)]
    pub(crate) fn file_format(&self) -> FileFormat {
        self.file.format()
    }
}
