use std::ops::Deref;
use serde::Deserialize;
use crate::templates::{PostprocessTemplate, ProcessingTemplate};
use super::{FileTemplate, FileTemplateBase};
use super::file::FileInfoTemplate;
use super::file::inbound::InboundFileInfoTemplate;
use super::layout::inbound::InboundLayoutTemplate;


#[derive(Debug, Deserialize)]
pub(crate) struct FileImportTemplate {
    #[serde(flatten)]
    base: FileTemplateBase,

    file: InboundFileInfoTemplate,
    layout: InboundLayoutTemplate,
    processing: ProcessingTemplate,
    postprocess: PostprocessTemplate
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


impl FileImportTemplate {
    #[allow(dead_code)]
    fn processing(&self) -> &ProcessingTemplate {
        &self.processing
    }


    #[allow(dead_code)]
    fn postprocess(&self) -> &PostprocessTemplate {
        &self.postprocess
    }
}


impl Deref for FileImportTemplate {
    type Target = FileTemplateBase;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
