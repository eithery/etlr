use std::ops::Deref;
use chrono::NaiveDateTime;
use serde::Deserialize;
use crate::std::datetime::DateTime;
use crate::std::result::Result;
use crate::templates::defaults::{default_true, default_false};
use super::FileTrailerTemplate;
use super::base::FileTrailerTemplateBase;


const DEFAULT_TAG: &str = "T";


#[derive(Debug, Deserialize)]
pub(crate) struct OutboundFileTrailerTemplate {
    #[serde(flatten)]
    base: FileTrailerTemplateBase,

    #[serde(default = "default_true")]
    include_file_type: bool,

    #[serde(default = "default_false")]
    include_file_name: bool,

    #[serde(default = "default_true")]
    include_trailer_date: bool,

    #[serde(default = "default_true")]
    include_record_count: bool
}


impl Deref for OutboundFileTrailerTemplate {
    type Target = FileTrailerTemplateBase;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}


impl OutboundFileTrailerTemplate {
    #[allow(dead_code)]
    fn include_file_type(&self) -> bool {
        self.include_file_type
    }


    #[allow(dead_code)]
    fn include_file_name(&self) -> bool {
        self.include_file_name
    }


    #[allow(dead_code)]
    fn include_trailer_date(&self) -> bool {
        self.include_trailer_date
    }


    #[allow(dead_code)]
    fn include_record_count(&self) -> bool {
        self.include_record_count
    }


    #[allow(dead_code)]
    fn build(&self, file_type: &str, file_name: &str, row_count: usize) -> Result<impl Iterator<Item = String>> {
        Ok([
            Some(self.tag().unwrap_or(DEFAULT_TAG).to_string()),
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
