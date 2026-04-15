use std::ops::Deref;
use serde::Deserialize;
use crate::templates::defaults::default_true;
use super::LayoutTemplate;
use super::header::FileHeaderTemplate;
use super::trailer::FileTrailerTemplate;
use super::file::FileEntry;


#[derive(Debug, Deserialize)]
#[serde(bound(deserialize = "H: Deserialize<'de>, T: Deserialize<'de>, F: Deserialize<'de>"))]
pub(crate) struct LayoutTemplateBase<H, T, F>
    where F: FileEntry
{
    header: H,
    trailer: T,

    #[serde(default, deserialize_with = "FileEntry::deserialize_files")]
    files: Vec<F>,

    #[serde(default = "default_true")]
    include_column_names: bool
}


impl<L, H, T, F> LayoutTemplate for L
    where L: Deref<Target = LayoutTemplateBase<H, T, F>>,
        H: FileHeaderTemplate + 'static,
        T: FileTrailerTemplate + 'static,
        F: FileEntry + 'static
{
    type Header = H;
    type Trailer = T;
    type File = F;


    fn header(&self) -> &Self::Header {
        &self.header
    }


    fn trailer(&self) -> &Self::Trailer {
        &self.trailer
    }


    #[allow(dead_code)]
    fn files(&self) -> impl Iterator<Item = &Self::File> {
        self.files.iter()
    }


    #[allow(dead_code)]
    fn include_column_names(&self) -> bool {
        self.include_column_names
    }


    fn has_multiple_files(&self) -> bool {
        self.files.len() > 1
    }


    fn included_file_types(&self) -> impl Iterator<Item = &str> {
        self.files().map(|f| f.file_type())
    }
}
