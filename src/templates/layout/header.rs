mod date;
mod file_type;
pub(super) mod inbound;
pub(super) mod outbound;

use std::ops::Deref;
use serde::Deserialize;
use crate::templates::defaults::default_true;


pub(crate) trait FileHeaderTemplate {
    fn enabled(&self) -> bool;

    fn tag(&self) -> Option<&str>;
}


#[derive(Debug, Deserialize)]
pub(crate) struct FileHeaderTemplateBase {
    #[serde(default = "default_true")]
    enabled: bool,

    tag: Option<String>
}


impl<T> FileHeaderTemplate for T
    where T: Deref<Target = FileHeaderTemplateBase>
{
    fn enabled(&self) -> bool {
        self.enabled
    }


    fn tag(&self) -> Option<&str> {
        self.tag.as_deref()
    }
}
