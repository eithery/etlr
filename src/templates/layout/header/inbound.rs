use std::ops::Deref;
use serde::Deserialize;
use super::base::FileHeaderTemplateBase;
use super::date::HeaderDateTemplate;
use super::file_type::FileTypeTemplate;


#[derive(Debug, Deserialize)]
pub(crate) struct InboundFileHeaderTemplate {
    #[serde(flatten)]
    base: FileHeaderTemplateBase,

    file_type: Option<FileTypeTemplate>,

    #[serde(rename = "date")]
    header_date: Option<HeaderDateTemplate>,

    #[serde(default)]
    subheaders: Vec<String>
}


impl Deref for InboundFileHeaderTemplate {
    type Target = FileHeaderTemplateBase;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}


impl InboundFileHeaderTemplate {
    #[allow(dead_code)]
    pub(super) fn file_type(&self) -> Option<&FileTypeTemplate> {
        self.file_type.as_ref()
    }


    #[allow(dead_code)]
    pub(super) fn header_date(&self) -> Option<&HeaderDateTemplate> {
        self.header_date.as_ref()
    }


    #[allow(dead_code)]
    pub(super) fn subheaders(&self) -> impl Iterator<Item = &str> {
        self.subheaders.iter().map(String::as_str)
    }
}
