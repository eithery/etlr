use std::collections::HashMap;
use std::ops::Deref;
use serde::Deserialize;
use crate::std::result::Result;
use crate::templates::defaults::default_false;
use crate::templates::errors as err;
use super::base::LayoutTemplateBase;
use super::dataset::DatasetTemplate;
use super::header::outbound::OutboundFileHeaderTemplate;
use super::file::OutboundFileTemplate;
use super::section::FileSectionTemplate;
use super::trailer::outbound::OutboundFileTrailerTemplate;


#[derive(Debug, Deserialize)]
pub(crate) struct OutboundLayoutTemplate {
    #[serde(flatten)]
    base: LayoutTemplateBase<OutboundFileHeaderTemplate, OutboundFileTrailerTemplate>,

    #[serde(default = "default_false")]
    extra_trailing_delimiters: bool,

    dataset: Option<DatasetTemplate>,

    #[serde(default, deserialize_with = "OutboundFileTemplate::deserialize_files")]
    files: Vec<OutboundFileTemplate>,

    #[serde(default)]
    sections: Vec<FileSectionTemplate>,
    section_selector: Option<String>
}


impl Deref for OutboundLayoutTemplate {
    type Target = LayoutTemplateBase<OutboundFileHeaderTemplate, OutboundFileTrailerTemplate>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}


impl OutboundLayoutTemplate {
    #[allow(dead_code)]
    pub(crate) fn has_multiple_files(&self) -> bool {
        self.files.len() > 1
    }


    #[allow(dead_code)]
    pub(crate) fn included_file_types(&self) -> impl Iterator<Item = &str> {
        self.files().map(|f| f.file_type())
    }


    #[allow(dead_code)]
    pub(crate) fn extra_trailing_delimiters(&self) -> bool {
        self.extra_trailing_delimiters
    }


    #[allow(dead_code)]
    pub(crate) fn dataset(&self) -> Option<&DatasetTemplate> {
        self.dataset.as_ref()
    }


    pub(crate) fn files(&self) -> impl Iterator<Item = &OutboundFileTemplate> {
        self.files.iter()
    }


    #[allow(dead_code)]
    fn sections(&self) -> impl Iterator<Item = &FileSectionTemplate> {
        self.sections.iter()
    }


    #[allow(dead_code)]
    pub(crate) fn build_fixed_length_rows(&self, fields: &HashMap<String, String>) -> Result<Vec<String>> {
        let selector = self.section_selector.as_ref().ok_or_else(err::missing_section_selector)?;
        let section_id = fields.get(selector).ok_or_else(|| err::missing_discriminator_field(selector))?;
        self.sections()
            .find(|s| s.id() == section_id)
            .map(|s| s.build_fixed_length_rows(fields))
            .ok_or_else(|| err::missing_file_section_template(section_id))
    }
}
