use serde::Deserialize;
use super::category::TemplateCategory;
use super::file::FileInfoTemplate;
use super::format::FileFormat;
use super::layout::RecordLayoutTemplate;
use super::traits::FileTemplate;


#[derive(Debug, Deserialize)]
pub(crate) struct FileExportTemplate {
    #[allow(dead_code)]
    #[serde(rename = "etl_template")]
    kind: TemplateCategory,

    version: u8,
    file: FileInfoTemplate,

    #[allow(dead_code)]
    description: Option<String>,

    layout: RecordLayoutTemplate
}


impl FileTemplate for FileExportTemplate {
    const TEMPLATES_ROOT: &'static str = "export";


    fn file_type(&self) -> &str {
        self.file.file_type()
    }


    fn layout(&self) -> &RecordLayoutTemplate {
        &self.layout
    }
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
