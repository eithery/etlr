use std::ops::Deref;
use serde::{Deserialize, Deserializer, de};
use serde_yaml::{Value, Mapping};
use crate::templates::defaults::default_false;
use super::base::FieldTemplateBase;
use super::position::FieldPosition;


#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub(crate) struct ExportableFieldTemplate {
    #[serde(flatten)]
    base: FieldTemplateBase,

    source: Option<String>,

    #[serde(default = "default_false")]
    preserve_invalid: bool
}


impl Deref for ExportableFieldTemplate {
    type Target = FieldTemplateBase;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}


impl ExportableFieldTemplate {
    pub(crate) fn pos(&self) -> &FieldPosition {
        &self.pos
    }


    pub(crate) fn len(&self) -> usize {
        self.pos().len()
    }


    pub(crate) fn source(&self) -> Option<&str> {
        self.source.as_deref()
    }


    #[allow(dead_code)]
    pub(super) fn preserve_invalid(&self) -> bool {
        self.preserve_invalid
    }


    #[allow(dead_code)]
    pub(crate) fn deserialize_fields<'de, D>(deserializer: D) -> Result<Vec<Self>, D::Error>
        where D: Deserializer<'de>
    {
        let payload = Value::deserialize(deserializer)?;
        match payload {
            Value::Sequence(fields) => {
                fields
                    .into_iter()
                    .map(|payload| Self::from_yaml::<D>(payload))
                    .collect()
            }
            _ => Err(de::Error::custom("`fields` element must contain a sequence."))
        }
    }


    pub(super) fn from_yaml<'de, D>(payload: Value) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        match payload {
            Value::Mapping(map) if map.len() == 1 => {
                let (key, val) = map.iter().next().unwrap();
                let field_name = key
                    .as_str()
                    .map(str::to_string)
                    .ok_or_else(|| de::Error::custom("Field name key element must be a string."))?;

                let mapping = val
                    .as_mapping()
                    .ok_or_else(|| de::Error::custom("`fields` metadata elements must be a map."))?;

                let pos = mapping
                    .get("pos")
                    .map(|pos| FieldPosition::from_yaml::<D>(pos))
                    .transpose()?
                    .ok_or_else(|| de::Error::custom("Missing or invalid `pos` metadata element."))?;

                let value = get_str::<D>(mapping, "value")?;
                let source = match (&value, get_str::<D>(mapping, "source")?) {
                    (None, None) => Some(field_name.clone()),
                    (Some(_), None) => None,
                    (_, Some(s)) => Some(s)
                };
                let required = get_bool::<D>(mapping, "required")?
                    .unwrap_or(false);
                let preserve_invalid = get_bool::<D>(mapping, "preserve_invalid")?
                    .unwrap_or(false);

                Ok(Self {
                    base: FieldTemplateBase {
                        name: field_name,
                        pos,
                        value,
                        required,
                        pii: None
                    },
                    source,
                    preserve_invalid
                })
            }
            _ => Err(de::Error::custom("`fields` entries must be single-entry maps."))
        }
    }
}


fn get_str<'de, D>(mapping: &Mapping, element_name: &str) -> Result<Option<String>, D::Error>
    where D: Deserializer<'de>
{ 
    mapping
        .get(element_name)
        .map(|val| {
            val.as_str()
                .ok_or_else(|| de::Error::custom(format!("Field `{element_name}` element must be a string.")))
                .map(str::to_string)
        })
        .transpose()
}


fn get_bool<'de, D>(mapping: &Mapping, element_name: &str) -> Result<Option<bool>, D::Error>
    where D: Deserializer<'de>
{ 
    mapping
        .get(element_name)
        .map(|val| {
            val.as_bool()
                .ok_or_else(|| de::Error::custom(format!("Field `{element_name}` element must be a boolean.")))
        })
        .transpose()
}
