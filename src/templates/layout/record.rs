use std::collections::HashMap;
use std::ops::Deref;
use serde::Deserialize;
use crate::templates::defaults::default_false;
use super::field::base::FieldTemplateBase;
use super::field::FieldTemplate;
use super::field::exportable::ExportableFieldTemplate;


#[derive(Debug, Deserialize)]
#[serde(bound(deserialize = "F: Deserialize<'de>"))]
pub(super) struct FileRecordTemplate<F> {
    id: String,
    name: Option<String>,

    #[serde(default = "default_false")]
    required: bool,

    #[serde(default = "default_false")]
    multiple: bool,

    #[serde(default)]
    fields: Vec<F>
}


impl<F> FileRecordTemplate<F>
    where F: FieldTemplate + Deref<Target = FieldTemplateBase>
{
    #[allow(dead_code)]
    pub(super) fn id(&self) -> &str {
        &self.id
    }


    #[allow(dead_code)]
    pub(super) fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }


    #[allow(dead_code)]
    pub(super) fn required(&self) -> bool {
        self.required
    }


    #[allow(dead_code)]
    pub(super) fn multiple(&self) -> bool {
        self.multiple
    }


    fn fields(&self) -> impl Iterator<Item = &F> {
        self.fields.iter()
    }
}


impl FileRecordTemplate<ExportableFieldTemplate> {
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
