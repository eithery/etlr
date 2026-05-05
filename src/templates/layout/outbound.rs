use std::ops::Deref;
use serde::Deserialize;
use crate::templates::defaults::default_false;
use super::base::LayoutTemplateBase;
use super::files::dataset::DatasetTemplate;
use super::header::outbound::OutboundFileHeaderTemplate;
use super::files::outbound::OutboundFileTemplate;
use super::section::FileSectionTemplate;
use super::trailer::outbound::OutboundFileTrailerTemplate;


#[derive(Debug, Deserialize)]
pub(crate) struct OutboundLayoutTemplate {
    #[serde(flatten)]
    base: LayoutTemplateBase<OutboundFileHeaderTemplate, OutboundFileTrailerTemplate, OutboundFileTemplate>,

    #[serde(default = "default_false")]
    extra_trailing_delimiters: bool,

    #[serde(default)]
    sections: Vec<FileSectionTemplate>,

    section_selector: Option<String>,

    record_size: Option<usize>,

    dataset: Option<DatasetTemplate>
}


impl Deref for OutboundLayoutTemplate {
    type Target = LayoutTemplateBase<OutboundFileHeaderTemplate, OutboundFileTrailerTemplate, OutboundFileTemplate>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}


impl OutboundLayoutTemplate {
    #[allow(dead_code)]
    fn extra_trailing_delimiters(&self) -> bool {
        self.extra_trailing_delimiters
    }


    #[allow(dead_code)]
    fn dataset(&self) -> Option<&DatasetTemplate> {
        self.dataset.as_ref()
    }


    #[allow(dead_code)]
    fn sections(&self) -> impl Iterator<Item = &FileSectionTemplate> {
        self.sections.iter()
    }


    #[allow(dead_code)]
    fn section_selector(&self) -> Option<&str> {
        self.section_selector.as_deref()
    }


    #[allow(dead_code)]
    fn record_size(&self) -> Option<usize> {
        self.record_size
    }
}
