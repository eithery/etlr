use serde::Deserialize;
use crate::templates::defaults::{default_true, default_date_format};


#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub(super) struct FileTrailerTemplate {
    tag: String,

    #[serde(rename = "date", default = "default_date_format")]
    date_format: String,

    #[serde(default = "default_true")]
    include_file_type: bool,

    include_file_name: bool,

    include_trailer_date: bool,

    #[serde(default = "default_true")]
    include_record_count: bool
}


#[allow(dead_code)]
impl FileTrailerTemplate {
    pub(super) fn tag(&self) -> &str {
        self.tag.as_str()
    }


    pub(super) fn date_format(&self) -> &str {
        self.date_format.as_str()
    }


    pub(super) fn include_file_type(&self) -> bool {
        self.include_file_type
    }


    pub(super) fn include_file_name(&self) -> bool {
        self.include_file_name
    }


    pub(super) fn include_trailer_date(&self) -> bool {
        self.include_trailer_date
    }


    pub(super) fn include_record_count(&self) -> bool {
        self.include_record_count
    }


    pub(super) fn build(&self, file_type: &str, file_name: &str, row_count: u32) -> impl Iterator<Item = String> {
        [
            Some(self.tag.clone()),
            self.include_file_type.then(|| file_type.to_string()),
            self.include_file_name.then(|| file_name.to_string()),
            // self.include_trailer_date.then(|| )
            self.include_record_count.then(|| row_count.to_string())
        ]
        .into_iter()
        .flatten()
    }
}
