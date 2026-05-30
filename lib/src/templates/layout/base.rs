use std::ops::Deref;
use serde::Deserialize;
use crate::templates::FileEntry;
use crate::templates::defaults::default_true;
use super::LayoutTemplate;
use super::header::FileHeaderTemplate;
use super::trailer::FileTrailerTemplate;


#[derive(Debug, Deserialize)]
#[serde(bound(deserialize = "H: Deserialize<'de>, T: Deserialize<'de>, F: Deserialize<'de>"))]
pub(crate) struct LayoutTemplateBase<H, T, F: FileEntry> {
    header: Option<H>,

    #[serde(default)]
    headers: Vec<H>,

    trailer: Option<T>,

    #[serde(default)]
    trailers: Vec<T>,

    record_size: Option<usize>,

    #[serde(default)]
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


    fn header(&self) -> Option<&Self::Header> {
        self.header.as_ref()
    }


    fn headers(&self) -> impl Iterator<Item = &H> {
        self.header()
            .into_iter()
            .chain(self.headers.iter())
    }


    fn trailer(&self) -> Option<&Self::Trailer> {
        self.trailer.as_ref()
    }


    fn trailers(&self) -> impl Iterator<Item = &T> {
        self.trailer()
            .into_iter()
            .chain(self.trailers.iter())
    }


    fn record_size(&self) -> Option<usize> {
        self.record_size
    }


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
