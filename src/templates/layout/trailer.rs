use chrono::NaiveDateTime;
use serde::Deserialize;
use crate::std::datetime::DateTime;
use crate::std::result::Result;
use crate::templates::defaults::{default_true, default_false, default_date_format};


const DEFAULT_TAG: &str = "T";


#[derive(Debug, Deserialize)]
pub(crate) struct FileTrailerTemplate {
    tag: Option<String>,

    #[serde(rename = "date", default = "default_date_format")]
    date_format: String,

    #[serde(default = "default_true")]
    include_file_type: bool,

    #[serde(default = "default_false")]
    include_file_name: bool,

    #[serde(default = "default_true")]
    include_trailer_date: bool,

    #[serde(default = "default_true")]
    include_record_count: bool,

    #[serde(default = "default_true")]
    enabled: bool
}


#[allow(dead_code)]
impl FileTrailerTemplate {
    pub(super) fn enabled(&self) -> bool {
        self.enabled
    }


    pub(super) fn tag(&self) -> &str {
        self.tag.as_deref().unwrap_or(DEFAULT_TAG)
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


    pub(super) fn build(&self, file_type: &str, file_name: &str, row_count: u32) -> Result<impl Iterator<Item = String>> {
        Ok([
            Some(self.tag().to_string()),
            self.include_file_type.then(|| file_type.to_string()),
            self.include_file_name.then(|| file_name.to_string()),
            self.include_trailer_date
                .then(|| NaiveDateTime::format_timestamp_with(self.date_format(), false))
                .transpose()?,
            self.include_record_count.then(|| row_count.to_string())
        ]
        .into_iter()
        .flatten())
    }
}
