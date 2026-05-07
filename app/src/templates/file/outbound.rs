use std::ops::Deref;
use serde::Deserialize;
use super::FileInfoTemplate;
use super::base::FileInfoTemplateBase;


#[derive(Debug, Deserialize)]
pub(crate) struct OutboundFileInfoTemplate {
    #[serde(flatten)]
    base: FileInfoTemplateBase,

    #[serde(rename = "name")]
    file_name: Option<String>
}


impl Deref for OutboundFileInfoTemplate {
    type Target = FileInfoTemplateBase;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}


impl OutboundFileInfoTemplate {
    pub(crate) fn file_name(&self) -> &str {
        self.file_name.as_deref().unwrap_or_else(|| {
            self.file_type()
                .split_once('.')
                .map_or(self.file_type(), |(_, name)| name)
        })
    }
}
