pub(super) mod data_element;
pub(super) mod column;
pub(crate) mod field;
pub(crate) mod position;

#[cfg(test)]
mod tests;

use serde::{Deserialize, Deserializer, de};
use serde_yaml::Value;
use field::FieldTemplate;


pub(crate) trait Fields {
    #[allow(dead_code)]
    fn fields(&self) -> impl Iterator<Item = &FieldTemplate>;


    #[allow(dead_code)]
    fn deserialize<'de, D>(deserializer: D) -> Result<Vec<FieldTemplate>, D::Error>
        where D: Deserializer<'de>
    {
        let payload = Value::deserialize(deserializer)?;
        match payload {
            Value::Sequence(fields) => {
                fields
                    .into_iter()
                    .map(|payload| FieldTemplate::try_from(&payload).map_err(de::Error::custom))
                    .collect()
            }
            _ => Err(de::Error::custom("`fields` element must contain a sequence."))
        }
    }
}
