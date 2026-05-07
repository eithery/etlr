use std::ops::Deref;
use serde::Deserialize;
use super::base::FileTemplateBase;
use super::file::FileInfoTemplate;
use super::file::inbound::InboundFileInfoTemplate;
use super::layout::inbound::InboundLayoutTemplate;
use super::traits::FileTemplate;


#[derive(Debug, Deserialize)]
pub(crate) struct FileImportTemplate {
    #[serde(flatten)]
    base: FileTemplateBase,

    file: InboundFileInfoTemplate,
    layout: InboundLayoutTemplate
}


impl Deref for FileImportTemplate {
    type Target = FileTemplateBase;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}


impl FileTemplate for FileImportTemplate {
    const TEMPLATES_ROOT: &'static str = "import";

    type Layout = InboundLayoutTemplate;


    fn file_type(&self) -> &str {
        self.file.file_type()
    }


    fn layout(&self) -> &InboundLayoutTemplate {
        &self.layout
    }
}
