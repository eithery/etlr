use serde::{Deserialize, Deserializer, de};
use serde_yaml::Value;
use crate::fs::yaml;


#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub(super) struct ColumnSelectionTemplate {
    include: ColumnsTemplate,

    #[serde(default)]
    exclude: Vec<String>
}


#[derive(Debug)]
#[allow(dead_code)]
enum ColumnsTemplate {
    All,
    Columns(Vec<(String, Option<String>)>)
}


impl<'de> Deserialize<'de> for ColumnsTemplate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        let value = Value::deserialize(deserializer)?;
        match value {
            Value::String(s) if s == ":all" => Ok(ColumnsTemplate::All),
            Value::Sequence(seq) => yaml::deserialize_columns::<D>(seq).map(ColumnsTemplate::Columns),
            _ => Err(de::Error::custom("`include` must be `:all` or a sequence of columns."))
        }
    }
}
