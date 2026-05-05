use std::collections::HashMap;
use serde::Deserialize;
use super::record::FileRecordTemplate;


#[derive(Debug, Deserialize)]
pub(super) struct FileSectionTemplate {
    id: String,
    description: Option<String>,
    record_size: usize,

    #[serde(default)]
    records: Vec<FileRecordTemplate>
}


impl FileSectionTemplate {
    pub(super) fn id(&self) -> &str {
        &self.id
    }


    #[allow(dead_code)]
    fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }


    #[allow(dead_code)]
    fn record_size(&self) -> usize {
        self.record_size
    }


    #[allow(dead_code)]
    fn records(&self) -> impl Iterator<Item = &FileRecordTemplate> {
        self.records.iter()
    }


    pub(super) fn build_fixed_length_rows(&self, fields: &HashMap<String, String>) -> Vec<String> {
        self.records
            .iter()
            .filter_map(|rec| rec.build_fixed_length_row(self.record_size, fields))
            .collect()
    }
}
