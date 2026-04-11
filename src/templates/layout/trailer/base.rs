use std::ops::Deref;
use serde::Deserialize;
use crate::templates::defaults::{default_true, default_date_format};
use super::FileTrailerTemplate;


#[derive(Debug, Deserialize)]
pub(crate) struct FileTrailerTemplateBase {
    #[serde(default = "default_true")]
    enabled: bool,

    tag: Option<String>,

    #[serde(rename = "date", default = "default_date_format")]
    date_format: String
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
}
