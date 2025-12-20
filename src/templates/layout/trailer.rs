use serde::Deserialize;
use crate::templates::defaults::{default_true, default_date_format};


#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub(crate) struct FileTrailerTemplate {
    tag: String,

    #[serde(rename = "date", default = "default_date_format")]
    date_format: String,

    #[serde(default = "default_true")]
    include_file_type: bool,

    #[serde(default = "default_true")]
    include_trailer_date: bool,

    #[serde(default = "default_true")]
    include_record_count: bool
}
