mod columns;
mod dataset;
mod file;
mod join;
mod header;
mod trailer;

use serde::Deserialize;
use super::defaults::default_true;
use dataset::DatasetTemplate;
use file::OutboundFileTemplate;
use header::FileHeaderTemplate;
use trailer::FileTrailerTemplate;


#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub(super) struct RecordLayoutTemplate {
    header: FileHeaderTemplate,
    trailer: FileTrailerTemplate,

    #[serde(default = "default_true")]
    include_column_names: bool,

    dataset: Option<DatasetTemplate>,

    #[serde(default, deserialize_with = "OutboundFileTemplate::deserialize_files")]
    files: Vec<OutboundFileTemplate>
}


#[allow(dead_code)]
impl RecordLayoutTemplate {
    pub(super) fn has_header(&self) -> bool {
        self.header().enabled()
    }


    pub(super) fn header(&self) -> &FileHeaderTemplate {
        &self.header
    }


    pub(super) fn has_trailer(&self) -> bool {
        self.trailer().enabled()
    }


    pub(super) fn trailer(&self) -> &FileTrailerTemplate {
        &self.trailer
    }


    pub(super) fn include_column_names(&self) -> bool {
        self.include_column_names
    }
}
