use serde::{Deserialize, Deserializer, de};
use serde_yaml::Mapping;
use super::columns::ColumnSelectionTemplate;
use super::join::DatasetJoinTemplate;


#[derive(Debug, Deserialize)]
pub(crate) struct DatasetTemplate {
    id: String,
    source: String,
    columns: ColumnSelectionTemplate,

    #[serde(default)]
    join: Vec<DatasetJoinTemplate>
}


impl DatasetTemplate {
    #[allow(dead_code)]
    fn id(&self) -> &str {
        &self.id
    }


    #[allow(dead_code)]
    fn source(&self) -> &str {
        &self.source
    }


    #[allow(dead_code)]
    fn columns(&self) -> &ColumnSelectionTemplate {
        &self.columns
    }


    #[allow(dead_code)]
    fn column_names(&self) -> impl Iterator<Item = &str> {
        self.columns.column_names()
    }


    fn joins(&self) -> impl Iterator<Item = &DatasetJoinTemplate> {
        self.join.iter()
    }


    #[allow(dead_code)]
    fn ignored_columns(&self) -> impl Iterator<Item = &str> {
        self.joins()
            .filter_map(|join| join.foreign_key())
            .chain(self.columns.excluded_columns())
    }


    #[allow(dead_code)]
    fn labeled_columns(&self) -> impl Iterator<Item = (&str, &str)> {
        self.columns
            .labeled_columns()
    }


    pub(super) fn from_yaml<'de, D>(payload: &Mapping) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        let value = payload
            .get("dataset")
            .ok_or_else(|| de::Error::custom("Missing or invalid `dataset` metadata element."))?;

        Ok(serde_yaml::from_value(value.clone()).map_err(de::Error::custom)?)
    }
}
