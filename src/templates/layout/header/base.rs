use std::ops::Deref;
use serde::Deserialize;
use crate::templates::defaults::default_true;
use super::FileHeaderTemplate;


#[derive(Debug, Deserialize)]
pub(crate) struct FileHeaderTemplateBase {
    #[serde(default = "default_true")]
    enabled: bool,

    tag: Option<String>
}


impl<T> FileHeaderTemplate for T
    where T: Deref<Target = FileHeaderTemplateBase>
{
    #[allow(dead_code)]
    fn enabled(&self) -> bool {
        self.enabled
    }


    #[allow(dead_code)]
    fn tag(&self) -> Option<&str> {
        self.tag.as_deref()
    }
}
