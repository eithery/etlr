use std::ops::Deref;
use etl_macros::DeserializeYaml;
use serde::Deserialize;
use serde_yaml::Value;
use crate::errors::EtlError;
use crate::yaml::{YamlNameValueMap, YamlReader, errors as err};
use crate::templates::FieldPosition;
use super::data_element::DataElementTemplate;


#[derive(Debug, DeserializeYaml)]
pub(crate) struct FieldTemplate {
    name: String,
    base: DataElementTemplate,
    pos: FieldPosition,
    source: Option<String>,
    exported: bool,
    discriminator: bool,
    preserve_invalid: bool
}


impl FieldTemplate {
    pub(crate) fn name(&self) -> &str {
        &self.name
    }


    pub(crate) fn pos(&self) -> FieldPosition {
        self.pos
    }


    pub(crate) fn len(&self) -> usize {
        self.pos().len()
    }


    pub(crate) fn source(&self) -> Option<&str> {
        self.source.as_deref()
    }


    #[allow(dead_code)]
    pub(crate) fn exported(&self) -> bool {
        self.exported
    }


    #[allow(dead_code)]
    pub(crate) fn discriminator(&self) -> bool {
        self.discriminator
    }


    #[allow(dead_code)]
    pub(crate) fn preserve_invalid(&self) -> bool {
        self.preserve_invalid
    }


    fn new(field_name: String, pos: FieldPosition) -> Self {
        Self {
            name: field_name,
            base: DataElementTemplate::default(),
            pos,
            source: None,
            exported: false,
            discriminator: false,
            preserve_invalid: false
        }
    }
}


impl Deref for FieldTemplate {
    type Target = DataElementTemplate;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}


impl TryFrom<&Value> for FieldTemplate {
    type Error = EtlError;

    fn try_from(payload: &Value) -> Result<Self, Self::Error> {
        let (field_name, value) = payload.to_name_value_map()?;
        match value {
            Value::Mapping(m) => {
                Ok(Self {
                    name: field_name,
                    base: DataElementTemplate::deserialize(value)?,
                    pos: m.get_value("pos")?,
                    source: m.get_opt_str("source")?,
                    exported: m.get_bool("exported", false)?,
                    discriminator: m.get_bool("discriminator", false)?,
                    preserve_invalid: m.get_bool("preserve_invalid", false)?
                })
            }
            Value::String(_) | Value::Number(_) => Ok(Self::new(field_name, value.try_into()?)),
            _ => Err(invalid_field_format())
        }
    }
}


fn invalid_field_format() -> EtlError {
    err::invalid_yaml_format("field", "Expected a mapping, string, or number")
}
