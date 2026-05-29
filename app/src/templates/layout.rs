mod base;
pub(super) mod control_record;
pub(crate) mod fields;
pub(crate) mod files;
mod header;
pub(super) mod inbound;
mod multitenant;
pub(super) mod outbound;
mod record;
mod record_id;
mod section;
mod trailer;

#[cfg(test)]
mod tests;

pub(crate) use control_record::ControlRecord;
use crate::templates::FileEntry;
use header::FileHeaderTemplate;
use trailer::FileTrailerTemplate;


pub(crate) trait LayoutTemplate {
    type Header: FileHeaderTemplate;
    type Trailer: FileTrailerTemplate;
    type File: FileEntry;


    fn header(&self) -> Option<&Self::Header>;

    fn headers(&self) -> impl Iterator<Item = &Self::Header>;

    #[allow(dead_code)]
    fn has_header(&self) -> bool {
        self.headers().any(|h| h.enabled())
    }


    fn trailer(&self) -> Option<&Self::Trailer>;

    fn trailers(&self) -> impl Iterator<Item = &Self::Trailer>;

    #[allow(dead_code)]
    fn has_trailer(&self) -> bool {
        self.trailers().any(|t| t.enabled())
    }


    fn record_size(&self) -> Option<usize>;

    fn files(&self) -> impl Iterator<Item = &Self::File>;

    fn has_multiple_files(&self) -> bool;

    fn has_single_file(&self) -> bool {
        !self.has_multiple_files()
    }

    fn included_file_types(&self) -> impl Iterator<Item = &str>;

    #[allow(dead_code)]
    fn include_column_names(&self) -> bool;
}
