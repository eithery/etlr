use std::collections::HashMap;
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
    pub(super) fn id(&self) -> &str {
        &self.id
    }


    #[allow(dead_code)]
    fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }


    fn records(&self) -> impl Iterator<Item = &FileRecordTemplate> {
        self.records.iter()
    }


    pub(super) fn build_fixed_length_rows(&self, field_values: &HashMap<&str, Option<&str>>, record_size: usize) -> Vec<String> {
        self.records()
            .filter_map(|rec| rec.build_fixed_length_row(field_values, record_size))
            .collect()
    }
}
