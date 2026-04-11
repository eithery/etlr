use std::ops::Deref;
use serde::Deserialize;
use super::base::FileTemplateBase;
use super::file::FileInfoTemplate;
use super::file::format::FileFormat;
use super::file::outbound::OutboundFileInfoTemplate;
use super::layout::outbound::OutboundLayoutTemplate;
use super::traits::FileTemplate;


#[derive(Debug, Deserialize)]
pub(crate) struct FileExportTemplate {
    #[serde(flatten)]
    base: FileTemplateBase,

    file: OutboundFileInfoTemplate,
    layout: OutboundLayoutTemplate
}


impl Deref for FileExportTemplate {
    type Target = FileTemplateBase;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}


impl FileTemplate for FileExportTemplate {
    const TEMPLATES_ROOT: &'static str = "export";

    type Layout = OutboundLayoutTemplate;


    fn file_type(&self) -> &str {
        self.file.file_type()
    }


    fn layout(&self) -> &OutboundLayoutTemplate {
        &self.layout
    }
}


impl FileExportTemplate {
    #[allow(dead_code)]
    pub(crate) fn file_type(&self) -> &str {
        self.file.file_type()
    }


    #[allow(dead_code)]
    pub(crate) fn outbound_file_name(&self) -> &str {
        self.file.file_name()
    }


    #[allow(dead_code)]
    pub(crate) fn file_format(&self) -> FileFormat {
        self.file.format()
    }
}
