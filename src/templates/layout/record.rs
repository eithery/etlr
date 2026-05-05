use std::collections::HashMap;
use serde::{Deserialize, Deserializer, de};
use serde_yaml::Value;
use crate::templates::FieldTemplate;
use crate::templates::layout::fields::Fields;
use crate::templates::defaults::default_false;


#[derive(Debug, Deserialize)]
pub(super) struct FileRecordTemplate
{
    id: String,

    name: Option<String>,

    #[serde(default = "default_false")]
    required: bool,

    #[serde(default = "default_false")]
    multiple: bool,

    #[serde(default)]
    fields: Vec<FieldTemplate>
}


impl FileRecordTemplate
{
    #[allow(dead_code)]
    pub(crate) fn id(&self) -> &str {
        &self.id
    }


    #[allow(dead_code)]
    fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }


    #[allow(dead_code)]
    fn required(&self) -> bool {
        self.required
    }


    #[allow(dead_code)]
    fn multiple(&self) -> bool {
        self.multiple
    }
}


impl FileRecordTemplate {
    pub(super) fn build_fixed_length_row(&self, record_size: usize, field_values: &HashMap<String, String>) -> Option<String> {
        if !self.fields().any(|f| field_values.contains_key(f.name())) {
            return None;
        }

        let mut row = vec![b' '; record_size + 1];
        row[record_size] = b'\n';

        for field in &self.fields {
            if let Some(val_bytes) = field.value()
                .or_else(|| {
                    field.source()
                        .and_then(|name| field_values.get(name))
                        .map(|s| s.as_str())
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
        String::from_utf8(row).ok()
    }
}


impl Fields for FileRecordTemplate
{
    fn fields(&self) -> impl Iterator<Item = &FieldTemplate> {
        self.fields.iter()
    }


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
