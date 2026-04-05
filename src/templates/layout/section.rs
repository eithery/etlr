use serde::Deserialize;
use super::record::FileRecordTemplate;


#[derive(Debug, Deserialize)]
pub(super) struct FileSectionTemplate {
    #[allow(dead_code)]
    id: String,

    #[allow(dead_code)]
    description: Option<String>,

    #[allow(dead_code)]
    record_size: usize,

    #[allow(dead_code)]
    #[serde(default)]
    records: Vec<FileRecordTemplate>
}
