use serde::{Deserialize, Deserializer, de};
use serde_yaml::Mapping;
use super::columns::ColumnSelectionTemplate;
use super::join::DatasetJoinTemplate;


#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub(super) struct DatasetTemplate {
    id: String,
    source: String,
    columns: ColumnSelectionTemplate,

    #[serde(default)]
    join: Vec<DatasetJoinTemplate>
}


impl DatasetTemplate {
    pub(super) fn from_yaml<'de, D>(payload: &Mapping) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        let value = payload
            .get("dataset")
            .ok_or_else(|| de::Error::custom("Missing or invalid `dataset` metadata element."))?;

        Ok(serde_yaml::from_value(value.clone()).map_err(de::Error::custom)?)
    }
}
