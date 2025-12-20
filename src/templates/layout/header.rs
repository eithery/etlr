use serde::Deserialize;
use crate::templates::defaults::{default_true, default_date_format};


#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub(super) struct FileHeaderTemplate {
    tag: String,

    #[serde(rename = "date", default = "default_date_format")]
    date_format: String,

    #[serde(default = "default_true")]
    include_file_version: bool
}
