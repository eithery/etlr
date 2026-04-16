use serde::{Deserialize, Deserializer, de::Error};
use serde_yaml::Value;
use crate::templates::layout::fields::position::FieldPosition;


#[derive(Debug)]
pub(super) struct HeaderDateTemplate {
    pos: Option<FieldPosition>,
    format: String
}


impl<'de> Deserialize<'de> for HeaderDateTemplate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        let value = Value::deserialize(deserializer)?;
        match value {
            Value::String(format) => Ok(Self { pos: None, format }),
            Value::Mapping(mapping) => {
                let pos = mapping
                    .get("pos")
                    .map(|pos| FieldPosition::from_yaml::<D>(pos))
                    .transpose()?
                    .ok_or_else(|| D::Error::custom("Missing or invalid header date `pos` metadata element."))?;

                let format = mapping
                    .get("format")
                    .map(|val| {
                        val.as_str()
                        .ok_or_else(|| D::Error::custom("Header date `format` element must be a string."))
                        .map(str::to_string)
                    })
                    .transpose()?
                    .ok_or_else(|| D::Error::custom("Missing or invalid header date `format` element."))?;

                Ok(Self { pos: Some(pos), format })
            }
            _ => Err(D::Error::custom("Header `date` element must be a string or an object."))
        }
    }
}


impl HeaderDateTemplate {
    #[allow(dead_code)]
    pub(super) fn pos(&self) -> Option<FieldPosition> {
        self.pos
    }


    #[allow(dead_code)]
    pub(super) fn format(&self) -> &str {
        &self.format
    }
}
