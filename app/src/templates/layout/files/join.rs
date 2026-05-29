use etl_macros::DeserializeYaml;
use serde::Deserialize;
use serde_yaml::Value;
use crate::errors::EtlError;
use crate::yaml::{LabeledColumns, errors as err};


#[derive(Debug, Deserialize)]
pub(super) struct DatasetJoinTemplate {
    source: String,
    key: String,
    foreign_key: Option<String>,
    columns: JoinColumnsTemplate
}


impl DatasetJoinTemplate {
    #[allow(dead_code)]
    fn key(&self) -> &str {
        &self.key
    }


    pub(super) fn foreign_key(&self) -> Option<&str> {
        self.foreign_key.as_deref()
    }


    #[allow(dead_code)]
    fn source(&self) -> &str {
        &self.source
    }


    #[allow(dead_code)]
    fn column_names(&self) -> impl Iterator<Item = &str> {
        let key_column = self.foreign_key()
            .unwrap_or(self.key());
        std::iter::once(key_column)
            .chain(self.columns.column_names())
    }
}


#[derive(Debug, DeserializeYaml)]
struct JoinColumnsTemplate(Vec<(String, Option<String>)>);


impl JoinColumnsTemplate {
    fn column_names(&self) -> impl Iterator<Item = &str> {
        self.0
            .iter()
            .map(|(name, _)| name.as_str())
    }
}


impl TryFrom<&Value> for JoinColumnsTemplate {
    type Error = EtlError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Sequence(columns) => columns.to_labeled_columns().map(Self),
            _ => Err(invalid_columns_format())
        }
    }
}


fn invalid_columns_format() -> EtlError {
    err::invalid_yaml_format("columns", "Expected a sequence of strings as column names")
}
