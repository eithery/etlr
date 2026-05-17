mod date;
mod file_type;
pub(super) mod inbound;
pub(super) mod outbound;

use std::ops::Deref;
use serde::Deserialize;
use super::control_record::{ControlRecord, ControlRecordTemplate};


pub(crate) trait FileHeaderTemplate: ControlRecord {
}


#[derive(Debug, Deserialize)]
pub(crate) struct FileHeaderTemplateBase {
    #[serde(flatten)]
    control_record: ControlRecordTemplate
}


impl<T> FileHeaderTemplate for T
    where T: Deref<Target = FileHeaderTemplateBase> {
}


impl Deref for FileHeaderTemplateBase {
    type Target = ControlRecordTemplate;
    
    fn deref(&self) -> &Self::Target {
        &self.control_record
    }
}
