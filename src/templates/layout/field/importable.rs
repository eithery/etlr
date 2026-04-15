use std::ops::Deref;
use serde::{Deserialize, Deserializer};
use crate::templates::defaults::default_false;
use super::{Field, Importable, ImportableField};
use super::base::DataElementTemplate;
use super::position::FieldPosition;


#[derive(Debug, Deserialize)]
pub(crate) struct ImportableFieldTemplate {
    #[serde(flatten)]
    base: DataElementTemplate,

    pos: FieldPosition,

    #[serde(rename = "type")]
    data_type: String,

    format: Option<String>,

    #[serde(default = "default_false")]
    key: bool,

    #[serde(default = "default_false")]
    exported: bool,

    #[serde(default = "default_false")]
    discriminator: bool,

    #[serde(default = "default_false")]
    preserve_invalid: bool
}


impl Deref for ImportableFieldTemplate {
    type Target = DataElementTemplate;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}


impl Field for ImportableFieldTemplate {
    fn pos(&self) -> FieldPosition {
        self.pos
    }


    fn deserialize_fields<'de, D>(_deserializer: D) -> Result<Vec<Self>, D::Error>
        where D: Deserializer<'de>
    {
        todo!();
    }
}


impl Importable for ImportableFieldTemplate {
    #[allow(dead_code)]
    fn data_type(&self) -> &str {
        &self.data_type
    }


    #[allow(dead_code)]
    fn key(&self) -> bool {
        self.key
    }
}


impl ImportableField for ImportableFieldTemplate{
    #[allow(dead_code)]
    fn format(&self) -> Option<&str> {
        self.format.as_deref()
    }


    #[allow(dead_code)]
    fn exported(&self) -> bool {
        self.exported
    }


    #[allow(dead_code)]
    fn discriminator(&self) -> bool {
        self.discriminator
    }


    #[allow(dead_code)]
    fn preserve_invalid(&self) -> bool {
        self.preserve_invalid
    }
}
