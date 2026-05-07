use std::ops::Deref;
use serde::Deserialize;
use super::base::FileInfoTemplateBase;
use super::name::FileNameTemplate;


#[derive(Debug, Deserialize)]
pub(crate) struct InboundFileInfoTemplate {
    #[serde(flatten)]
    base: FileInfoTemplateBase,

    #[serde(rename = "name")]
    file_name: Option<FileNameTemplate>
}


impl Deref for InboundFileInfoTemplate {
    type Target = FileInfoTemplateBase;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}


impl InboundFileInfoTemplate {
    #[allow(dead_code)]
    fn file_name(&self) -> Option<&FileNameTemplate> {
        self.file_name.as_ref()
    }
}
