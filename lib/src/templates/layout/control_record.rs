use std::ops::Deref;
use serde::Deserialize;
use crate::templates::defaults::{default_false, default_true};
use crate::templates::prelude::FieldTemplate;


pub(crate) trait ControlRecord {
    fn enabled(&self) -> bool;

    fn tag(&self) -> Option<&str>;

    fn include_to_row_count(&self) -> bool;

    fn fields(&self) -> impl Iterator<Item = &FieldTemplate>;
}


#[derive(Debug, Deserialize)]
pub(crate) struct ControlRecordTemplate {
    #[serde(default = "default_true")]
    enabled: bool,

    tag: Option<String>,

    #[serde(default = "default_false")]
    include_to_row_count: bool,

    #[serde(default)]
    fields: Vec<FieldTemplate>
}


impl ControlRecord for ControlRecordTemplate {
    fn enabled(&self) -> bool {
        self.enabled
    }


    fn tag(&self) -> Option<&str> {
        self.tag.as_deref()
    }


    fn include_to_row_count(&self) -> bool {
        self.enabled && self.include_to_row_count
    }


    fn fields(&self) -> impl Iterator<Item = &FieldTemplate> {
        self.fields.iter()
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


    fn fields(&self) -> impl Iterator<Item = &FieldTemplate> {
        self.deref().fields()
    }
}
