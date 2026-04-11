use std::ops::Deref;
use serde::Deserialize;
use crate::templates::defaults::default_true;
use super::LayoutTemplate;
use super::header::FileHeaderTemplate;
use super::trailer::FileTrailerTemplate;


#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub(crate) struct LayoutTemplateBase<H, T> {
    header: H,
    trailer: T,

    #[serde(default = "default_true")]
    include_column_names: bool
}


impl<L, H, T> LayoutTemplate for L
    where L: Deref<Target = LayoutTemplateBase<H, T>>,
        H: FileHeaderTemplate + 'static,
        T: FileTrailerTemplate + 'static
{
    type Header = H;
    type Trailer = T;


    fn header(&self) -> &Self::Header {
        &self.header
    }


    fn trailer(&self) -> &Self::Trailer {
        &self.trailer
    }


    fn include_column_names(&self) -> bool {
        self.include_column_names
    }


    fn has_multiple_files(&self) -> bool {
        false
    }


    fn included_file_types(&self) -> impl Iterator<Item = &str> {
        std::iter::empty::<&str>()
    }
}
