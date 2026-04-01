use serde::Deserialize;
use super::file::FileInfoTemplate;
use super::layout::RecordLayoutTemplate;
use super::traits::FileTemplate;


#[derive(Debug, Deserialize)]
pub(crate) struct FileImportTemplate {
    file: FileInfoTemplate,
    layout: RecordLayoutTemplate
}


impl FileTemplate for FileImportTemplate {
    const TEMPLATES_ROOT: &'static str = "import";


    fn file_type(&self) -> &str {
        self.file.file_type()
    }


    #[allow(dead_code)]
    fn layout(&self) -> &RecordLayoutTemplate {
        &self.layout
    }
}
