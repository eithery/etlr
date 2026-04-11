use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub(super) struct FileNameTemplate {
    file_mask: String,
    date_format: String
}


impl FileNameTemplate {
    #[allow(dead_code)]
    pub(super) fn file_mask(&self) -> &str {
        &self.file_mask
    }


    #[allow(dead_code)]
    pub(super) fn date_format(&self) -> &str {
        &self.date_format
    }
}
