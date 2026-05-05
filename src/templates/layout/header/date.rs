use serde::{Deserialize, Deserializer, de};
use serde_yaml::Value;
use crate::templates::FieldPosition;


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
                    .map(TryInto::try_into)
                    .transpose()
                    .map_err(de::Error::custom)?
                    .ok_or_else(|| de::Error::custom("Missing or invalid header date `pos` metadata element."))?;

                let format = mapping
                    .get("format")
                    .map(|val| {
                        val.as_str()
                        .ok_or_else(|| de::Error::custom("Header date `format` element must be a string."))
                        .map(str::to_string)
                    })
                    .transpose()?
                    .ok_or_else(|| de::Error::custom("Missing or invalid header date `format` element."))?;

                Ok(Self { pos: Some(pos), format })
            }
            _ => Err(de::Error::custom("Header `date` element must be a string or an object."))
        }
    }
}


impl HeaderDateTemplate {
    #[allow(dead_code)]
    fn pos(&self) -> Option<FieldPosition> {
        self.pos
    }


    #[allow(dead_code)]
    fn format(&self) -> &str {
        &self.format
    }
}
