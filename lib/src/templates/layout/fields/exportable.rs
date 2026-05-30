use std::borrow::Cow;
use std::collections::HashMap;
use crate::std::result::Result;
use crate::std::string::PadLeft;
use super::field::FieldTemplate;


pub(crate) trait ExportableFields {
    fn exportable_fields(&self) -> impl Iterator<Item = &FieldTemplate>;


    fn build_fixed_length(&self, field_values: &HashMap<&str, Option<&str>>, record_size: usize) -> Result<String> {
        let mut row = vec![b' '; record_size + 1];
        row[record_size] = b'\n';
        for field in self.exportable_fields() {
            if let Some(val) = field.value()
                .or_else(|| {
                    field.source()
                        .and_then(|name| field_values.get(name))
                        .and_then(|opt| *opt)
                })
            {
                let value = if field.has_leading_zeros() {
                    Cow::Owned(val.leading_zeros(field.len()))
                } else {
                    Cow::Borrowed(val)
                };
                let val_bytes = value.as_bytes();
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
