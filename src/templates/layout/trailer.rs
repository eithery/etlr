pub(super) mod inbound;
pub(super) mod outbound;
mod row_count;

use std::ops::Deref;
use serde::Deserialize;
use crate::templates::defaults::{default_true, default_date_format};
use super::trailer::row_count::RowCountTemplate;


pub(crate) trait FileTrailerTemplate {
    fn enabled(&self) -> bool;

    fn tag(&self) -> Option<&str>;

    fn date_format(&self) -> &str;

    fn row_count(&self) -> Option<&RowCountTemplate>;


    #[allow(dead_code)]
    fn row_count_includes_header(&self) -> bool {
        self.row_count()
            .map_or(false, |rc| rc.include_header())
    }


    #[allow(dead_code)]
    fn row_count_includes_trailer(&self) -> bool {
        self.row_count()
            .map_or(false, |rc| rc.include_trailer())
    }
}


#[derive(Debug, Deserialize)]
pub(crate) struct FileTrailerTemplateBase {
    #[serde(default = "default_true")]
    enabled: bool,

    tag: Option<String>,

    #[serde(rename = "date", default = "default_date_format")]
    date_format: String,

    #[serde(rename = "record_count")]
    row_count: Option<RowCountTemplate>
}


impl<T> FileTrailerTemplate for T
    where T: Deref<Target = FileTrailerTemplateBase>
{
    #[allow(dead_code)]
    fn enabled(&self) -> bool {
        self.enabled
    }


    #[allow(dead_code)]
    fn tag(&self) -> Option<&str> {
        self.tag.as_deref()
    }


    #[allow(dead_code)]
    fn date_format(&self) -> &str {
        &self.date_format
    }


    #[allow(dead_code)]
    fn row_count(&self) -> Option<&RowCountTemplate> {
        self.row_count.as_ref()
    }
}
