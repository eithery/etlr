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
pub(crate) struct RecordLayoutTemplate {
    header: FileHeaderTemplate,
    trailer: FileTrailerTemplate,

    #[serde(default = "default_true")]
    include_column_names: bool,

    dataset: Option<DatasetTemplate>,

    #[serde(default, deserialize_with = "OutboundFileTemplate::deserialize_files")]
    files: Vec<OutboundFileTemplate>
}


impl RecordLayoutTemplate {
    pub(super) fn has_multiple_files(&self) -> bool {
        self.files.len() > 1
    }


    #[allow(dead_code)]
    pub(super) fn has_single_file(&self) -> bool {
        !self.has_multiple_files()
    }


    pub(super) fn files(&self) -> impl Iterator<Item = &OutboundFileTemplate> {
        self.files.iter()
    }


    pub(super) fn included_file_types(&self) -> impl Iterator<Item = &str> {
        self.files().map(|f| f.file_type())
    }


    #[allow(dead_code)]
    pub(super) fn has_header(&self) -> bool {
        self.header().enabled()
    }


    #[allow(dead_code)]
    pub(super) fn header(&self) -> &FileHeaderTemplate {
        &self.header
    }


    #[allow(dead_code)]
    pub(super) fn has_trailer(&self) -> bool {
        self.trailer().enabled()
    }


    #[allow(dead_code)]
    pub(super) fn trailer(&self) -> &FileTrailerTemplate {
        &self.trailer
    }


    #[allow(dead_code)]
    pub(super) fn include_column_names(&self) -> bool {
        self.include_column_names
    }
}
