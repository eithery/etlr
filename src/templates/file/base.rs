use serde::Deserialize;
use super::FileInfoTemplate;
use super::format::FileFormat;


#[derive(Debug, Deserialize)]
pub(crate) struct FileInfoTemplateBase {
    #[serde(rename = "type")]
    file_type: String,

    format: FileFormat
}


impl FileInfoTemplate for FileInfoTemplateBase {
    fn file_type(&self) -> &str {
        self.file_type.as_str()
    }


    fn format(&self) -> FileFormat {
        self.format
    }
}
