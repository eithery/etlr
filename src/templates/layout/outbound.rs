use std::collections::HashMap;
use std::ops::Deref;
use serde::Deserialize;
use crate::std::result::Result;
use crate::templates::defaults::default_false;
use crate::templates::errors as err;
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


    fn sections(&self) -> impl Iterator<Item = &FileSectionTemplate> {
        self.sections.iter()
    }


    fn section_selector(&self) -> Option<&str> {
        self.section_selector.as_deref()
    }


    fn record_size(&self) -> Option<usize> {
        self.record_size
    }


    #[allow(dead_code)]
    fn build_fixed_length_rows(&self, field_values: &HashMap<&str, Option<&str>>) -> Result<Vec<String>> {
        let selector = self.section_selector()
            .ok_or_else(err::missing_section_selector)?;
        let record_size = self.record_size()
            .ok_or_else(err::record_size_not_defined)?;
        let section_id = field_values
            .get(selector)
            .ok_or_else(|| err::missing_discriminator_field(selector))?
            .as_ref()
            .ok_or_else(|| err::blank_discriminator_field(selector))?;
        self.sections()
            .find(|s| s.id() == *section_id)
            .map(|s| s.build_fixed_length_rows(field_values, record_size))
            .ok_or_else(|| err::missing_file_section_template(&section_id))
    }
}
