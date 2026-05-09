use either::Either;
use serde::{Deserialize, Deserializer, de};
use serde_yaml::Value;
use crate::fs::yaml;


#[derive(Debug, Deserialize)]
pub(super) struct DataColumnsTemplate {
    include: Columns,

    #[serde(default)]
    exclude: Vec<String>
}


impl DataColumnsTemplate {
    pub(super) fn column_names(&self) -> impl Iterator<Item = &str> {
        match &self.include {
            Columns::All => Either::Left(std::iter::empty()),
            Columns::Included(cols) => Either::Right(
                cols.iter()
                    .filter_map(move |(name, _)| {
                        if self.exclude.contains(name) { None } else { Some(name.as_str()) }
                    })
            )
        }
    }


    pub(super) fn excluded_columns(&self) -> impl Iterator<Item = &str> {
        self.exclude.iter().map(|s| s.as_str())
    }


    pub(super) fn labeled_columns(&self) -> impl Iterator<Item = (&str, &str)> {
        self.include.labeled_columns()
    }
}


#[derive(Debug)]
enum Columns {
    All,
    Included(Vec<(String, Option<String>)>)
}


impl<'de> Deserialize<'de> for Columns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        let value = Value::deserialize(deserializer)?;
        match value {
            Value::String(s) if s == ":all" => Ok(Columns::All),
            Value::Sequence(seq) => yaml::deserialize_columns::<D>(seq).map(Columns::Included),
            _ => Err(de::Error::custom("`include` must be `:all` or a sequence of columns."))
        }
    }
}


impl Columns {
    fn labeled_columns<'a>(&'a self) -> impl Iterator<Item = (&'a str, &'a str)> {
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
