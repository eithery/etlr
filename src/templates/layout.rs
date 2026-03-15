mod columns;
mod dataset;
mod file;
mod header;
mod trailer;

use std::collections::HashMap;
use serde::Deserialize;
use super::defaults::default_true;
use file::OutboundFileTemplate;
use header::FileHeaderTemplate;
use trailer::FileTrailerTemplate;


#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub(crate) struct RecordLayoutTemplate {
    header: FileHeaderTemplate,
    trailer: FileTrailerTemplate,

    #[serde(default = "default_true")]
    include_column_names: bool,

    files: Vec<HashMap<String, OutboundFileTemplate>>
}
