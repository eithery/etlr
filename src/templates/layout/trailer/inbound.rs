use std::ops::Deref;
use serde::Deserialize;
use super::base::FileTrailerTemplateBase;


#[derive(Debug, Deserialize)]
pub(crate) struct InboundFileTrailerTemplate {
    #[serde(flatten)]
    base: FileTrailerTemplateBase,
}


impl Deref for InboundFileTrailerTemplate {
    type Target = FileTrailerTemplateBase;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
