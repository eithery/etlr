use serde::Deserialize;
use super::record::FileRecordTemplate;


#[derive(Debug, Deserialize)]
pub(super) struct FileSectionTemplate {
    id: String,

    description: Option<String>,

    #[serde(default)]
    records: Vec<FileRecordTemplate>
}


impl FileSectionTemplate {
    #[allow(dead_code)]
    fn id(&self) -> &str {
        &self.id
    }


    #[allow(dead_code)]
    fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }


    #[allow(dead_code)]
    fn records(&self) -> impl Iterator<Item = &FileRecordTemplate> {
        self.records.iter()
    }
}
