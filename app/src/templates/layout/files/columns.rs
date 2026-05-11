use either::Either;
use etl_macros::DeserializeYaml;
use serde::Deserialize;
use serde_yaml::Value;
use crate::errors::EtlError;
use crate::fs::yaml::{LabeledColumns, invalid_yaml_format};


#[derive(Debug, DeserializeYaml)]
pub(super) enum Columns {
    All,
    Included(Vec<(String, Option<String>)>)
}


impl Columns {
    pub(super) fn labeled_columns<'a>(&'a self) -> impl Iterator<Item = (&'a str, &'a str)> {
        match self {
            Self::All => Either::Left(std::iter::empty()),
            Self::Included(columns) => Either::Right(
                columns.iter()
                .filter_map(|(name, label)| {
                    label.as_ref().map(|label| (name.as_str(), label.as_str()))
                })
            )
        }
    }
}


impl TryFrom<&Value> for Columns {
    type Error = EtlError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::String(s) if s == ":all" => Ok(Columns::All),
            Value::Sequence(columns) => {
                columns.to_labeled_columns()
                    .map(Columns::Included)
            }
            _ => Err(invalid_include_format())
        }
    }
}


fn invalid_include_format() -> EtlError {
    invalid_yaml_format("include", "Expected `:all` or a sequence of columns")
}
