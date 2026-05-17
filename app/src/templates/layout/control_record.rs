use std::ops::Deref;
use serde::Deserialize;
use crate::templates::defaults::{default_false, default_true};


pub(crate) trait ControlRecord {
    fn enabled(&self) -> bool;

    fn tag(&self) -> Option<&str>;

    #[allow(dead_code)]
    fn include_to_row_count(&self) -> bool;
}


#[derive(Debug, Deserialize)]
pub(crate) struct ControlRecordTemplate {
    #[serde(default = "default_true")]
    enabled: bool,

    tag: Option<String>,

    #[serde(default = "default_false")]
    include_to_row_count: bool
}


impl ControlRecord for ControlRecordTemplate {
    fn enabled(&self) -> bool {
        self.enabled
    }


    fn tag(&self) -> Option<&str> {
        self.tag.as_deref()
    }


    #[allow(dead_code)]
    fn include_to_row_count(&self) -> bool {
        self.enabled && self.include_to_row_count
    }
}


impl<T> ControlRecord for T
    where T: Deref, T::Target: ControlRecord
{
    fn enabled(&self) -> bool {
        self.deref().enabled()
    }


    fn tag(&self) -> Option<&str> {
        self.deref().tag()
    }


    fn include_to_row_count(&self) -> bool {
        self.deref().include_to_row_count()
    }
}
