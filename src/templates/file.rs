use serde::Deserialize;
use super::format::FileFormat;


#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub(super) struct FileInfoTemplate {
    #[serde(rename = "type")]
    file_type: String,

    format: FileFormat
}
