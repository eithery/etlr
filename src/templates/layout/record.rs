use serde::Deserialize;
use crate::templates::defaults::default_false;
use super::field::FieldTemplate;


#[derive(Debug, Deserialize)]
pub(super) struct FileRecordTemplate {
    #[allow(dead_code)]
    id: String,

    #[allow(dead_code)]
    #[serde(default = "default_false")]
    required: bool,

    #[allow(dead_code)]
    #[serde(default, deserialize_with = "FieldTemplate::deserialize_fields")]
    fields: Vec<FieldTemplate>
}
