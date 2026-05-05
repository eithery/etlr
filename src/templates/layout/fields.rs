pub(super) mod data_element;
pub(super) mod column;
pub(crate) mod field;
pub(crate) mod position;

#[cfg(test)]
mod tests;

use std::collections::HashMap;
use serde::{Deserialize, Deserializer, de};
use serde_yaml::Value;
use crate::std::result::EtlResult;
use field::FieldTemplate;


pub(crate) trait Fields {
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


    #[allow(dead_code)]
    fn build_fixed_length(&self, field_values: &HashMap<&str, Option<&str>>, record_size: usize) -> EtlResult<String> {
        let mut row = vec![b' '; record_size + 1];
        row[record_size] = b'\n';
        for field in self.fields() {
            if let Some(val_bytes) = field.value()
                .or_else(|| {
                    field.source()
                        .and_then(|name| field_values.get(name))
                        .and_then(|opt| *opt)
                })
                .map(|s| s.as_bytes())
            {
                let start = field.pos().start() - 1;
                let limit = val_bytes.len().min(field.len());
                if start < record_size {
                    let end = (start + limit).min(record_size);
                    row[start..end].copy_from_slice(&val_bytes[..limit]);
                }
            }
        }
        Ok(String::from_utf8(row).unwrap())
    }
}
