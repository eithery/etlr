pub(super) mod inbound;
pub(super) mod outbound;
mod row_count;

use std::ops::Deref;
use serde::Deserialize;
use crate::templates::defaults::default_date_format;
use crate::templates::layout::control_record::ControlRecordTemplate;
use super::ControlRecord;
use row_count::RowCountTemplate;


pub(crate) trait FileTrailerTemplate: ControlRecord {
    fn date_format(&self) -> &str;

    #[allow(dead_code)]
    fn row_count(&self) -> Option<&RowCountTemplate>;
}


#[derive(Debug, Deserialize)]
pub(crate) struct FileTrailerTemplateBase {
    #[serde(flatten)]
    control_record: ControlRecordTemplate,

    #[serde(rename = "date", default = "default_date_format")]
    date_format: String,

    #[serde(rename = "record_count")]
    row_count: Option<RowCountTemplate>
}


impl<T> FileTrailerTemplate for T
    where T: Deref<Target = FileTrailerTemplateBase>
{
    fn date_format(&self) -> &str {
        &self.date_format
    }


    #[allow(dead_code)]
    fn row_count(&self) -> Option<&RowCountTemplate> {
        self.row_count.as_ref()
    }
}


impl Deref for FileTrailerTemplateBase {
    type Target = ControlRecordTemplate;

    fn deref(&self) -> &Self::Target {
        &self.control_record
    }
}
