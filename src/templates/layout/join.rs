use serde::{Deserialize, Deserializer, de};
use serde_yaml::Value;
use crate::fs::yaml;


#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub(super) struct DatasetJoinTemplate {
    source: String,
    key: String,
    foreign_key: Option<String>,
    columns: JoinColumnsTemplate
}


#[allow(dead_code)]
#[derive(Debug)]
struct JoinColumnsTemplate(Vec<(String, Option<String>)>);


impl<'de> Deserialize<'de> for JoinColumnsTemplate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        let value = Value::deserialize(deserializer)?;
        match value {
            Value::Sequence(seq) => yaml::deserialize_columns::<D>(seq).map(JoinColumnsTemplate),
            _ => Err(de::Error::custom("`columns` must be a sequence of columns."))
        }
    }
}
