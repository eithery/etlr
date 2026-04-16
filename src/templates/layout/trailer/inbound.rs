use std::ops::Deref;
use serde::Deserialize;
use super::base::FileTrailerTemplateBase;
use super::record_count::RecordCountTemplate;


#[derive(Debug, Deserialize)]
pub(crate) struct InboundFileTrailerTemplate {
    #[serde(flatten)]
    base: FileTrailerTemplateBase,

    record_count: Option<RecordCountTemplate>
}


impl Deref for InboundFileTrailerTemplate {
    type Target = FileTrailerTemplateBase;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}


impl InboundFileTrailerTemplate {
    #[allow(dead_code)]
    fn record_count(&self) -> Option<&RecordCountTemplate> {
        self.record_count.as_ref()
    }
}
