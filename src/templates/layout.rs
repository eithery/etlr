mod base;
mod columns;
pub(crate) mod dataset;
mod field;
pub(crate) mod file;
pub(crate) mod join;
mod header;
pub(super) mod inbound;
mod multitenant;
pub(super) mod outbound;
mod record;
mod record_id;
mod section;
mod trailer;

use header::FileHeaderTemplate;
use trailer::FileTrailerTemplate;


pub(crate) trait LayoutTemplate {
    type Header: FileHeaderTemplate;
    type Trailer: FileTrailerTemplate;


    fn header(&self) -> &Self::Header;


    #[allow(dead_code)]
    fn has_header(&self) -> bool {
        self.header().enabled()
    }


    fn trailer(&self) -> &Self::Trailer;


    #[allow(dead_code)]
    fn has_trailer(&self) -> bool {
        self.trailer().enabled()
    }


    #[allow(dead_code)]
    fn include_column_names(&self) -> bool;

    fn has_multiple_files(&self) -> bool;


    fn has_single_file(&self) -> bool {
        !self.has_multiple_files()
    }


    fn included_file_types(&self) -> impl Iterator<Item = &str>;
}
