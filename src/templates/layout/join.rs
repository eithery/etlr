use serde::{Deserialize, Deserializer, de};
use serde_yaml::Value;
use crate::fs::yaml;


#[derive(Debug, Deserialize)]
pub(super) struct DatasetJoinTemplate {
    source: String,
    key: String,
    foreign_key: Option<String>,
    columns: JoinColumnsTemplate
}


impl DatasetJoinTemplate {
    #[allow(dead_code)]
    pub(super) fn key(&self) -> &str {
        &self.key
    }


    pub(super) fn foreign_key(&self) -> Option<&str> {
        self.foreign_key.as_deref()
    }


    #[allow(dead_code)]
    pub(super) fn source(&self) -> &str {
        &self.source
    }


    #[allow(dead_code)]
    pub(super) fn column_names(&self) -> impl Iterator<Item = &str> {
        self.columns.column_names()
    }
}


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


impl JoinColumnsTemplate {
    #[allow(dead_code)]
    fn column_names(&self) -> impl Iterator<Item = &str> {
        self.0
            .iter()
            .map(|(name, _)| name.as_str())
    }
}
