use std::ops::Deref;
use serde::Deserialize;
use crate::templates::defaults::default_false;
use super::base::FieldTemplateBase;


#[derive(Debug, Deserialize)]
pub(crate) struct ImportableFieldTemplate {
    #[serde(flatten)]
    base: FieldTemplateBase,

    #[serde(rename = "type")]
    field_type: String,

    format: Option<String>,

    #[serde(default = "default_false")]
    key: bool,

    #[serde(default = "default_false")]
    exported: bool,

    #[serde(default = "default_false")]
    discriminator: bool
}


impl Deref for ImportableFieldTemplate {
    type Target = FieldTemplateBase;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}


impl ImportableFieldTemplate {
    #[allow(dead_code)]
    pub(super) fn field_type(&self) -> &str {
        &self.field_type
    }


    #[allow(dead_code)]
    pub(super) fn format(&self) -> Option<&str> {
        self.format.as_deref()
    }


    #[allow(dead_code)]
    pub(super) fn key(&self) -> bool {
        self.key
    }


    #[allow(dead_code)]
    pub(super) fn exported(&self) -> bool {
        self.exported
    }


    #[allow(dead_code)]
    pub(super) fn discriminator(&self) -> bool {
        self.discriminator
    }
}
