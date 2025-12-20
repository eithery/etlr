use serde::{Deserialize, Deserializer, de};
use serde_yaml::Value;


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
        fn as_str<'a, E: de::Error>(value: &'a Value, error_message: &str) -> Result<&'a str, E> {
            value.as_str().ok_or_else(|| de::Error::custom(error_message))
        }

        let value = Value::deserialize(deserializer)?;
        match value {
            Value::String(s) if s == ":all" => Ok(ColumnsTemplate::All),
            Value::Sequence(seq) => {
                let columns = seq.into_iter().map(|item| {
                    match item {
                        Value::String(name) => Ok((name, None)),
                        Value::Mapping(map) if map.len() == 1 => {
                            let (k, v) = map.into_iter().next().unwrap();
                            Ok((
                                as_str(&k, "Column name must be a string.")?.to_string(),
                                Some(as_str(&v, "Column label must be a string.")?.to_string())
                            ))
                        }
                        _ => Err(de::Error::custom("`include` entries must be strings or single-entry maps."))
                    }
                }).collect::<Result<Vec<_>, _>>()?;
                Ok(ColumnsTemplate::Columns(columns))
            }
            _ => Err(de::Error::custom("`include` must be `:all` or a sequence of columns."))
        }
    }
}
