use serde::{Deserialize, Deserializer, de};
use serde_yaml::Value;


#[derive(Debug, Deserialize, Clone, Copy)]
pub(crate) struct FieldPosition {
    start: usize,
    end: usize
}


impl FieldPosition {
    pub(crate) fn start(&self) -> usize {
        self.start
    }


    #[allow(dead_code)]
    pub(super) fn end(&self) -> usize {
        self.end
    }


    pub(super) fn len(&self) -> usize {
        self.end - self.start + 1
    }


    pub(crate) fn from_yaml<'de, D>(payload: &Value) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        match payload {
            Value::String(value) => {
                if let Some((s, e)) = value.split_once("..") {
                    let start = s.trim().parse::<usize>().map_err(de::Error::custom)?;
                    let end = e.trim().parse::<usize>().map_err(de::Error::custom)?;
                    Ok(Self { start, end })
                } else {
                    let val = value.trim().parse::<usize>().map_err(de::Error::custom)?;
                    Ok(Self { start: val, end: val })
                }
            }
            Value::Number(n) => {
                let val = n.as_u64()
                    .ok_or_else(|| de::Error::custom("Invalid `pos` element value."))? as usize;
                Ok(FieldPosition { start: val, end: val })
            }
            _ => Err(de::Error::custom("Field `pos` element must be a number or string."))
        }
    }
}
