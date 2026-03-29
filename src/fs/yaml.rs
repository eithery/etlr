use std::fs::File;
use std::path::Path;
use serde::{Deserializer, de};
use serde::de::DeserializeOwned;
use serde_yaml::Value;
use crate::errors as err;


type EtlResult<T> = crate::std::result::Result<T>;


pub(crate) fn load_from_file<T: DeserializeOwned>(file_path: &Path) -> EtlResult<T> {
    if !file_path.is_file() {
        return Err(err::file_does_not_exist(file_path));
    }

    let file = File::open(file_path)?;
    Ok(serde_yaml::from_reader(file)?)
}


pub(crate) fn load_from_str<T: DeserializeOwned>(yaml_str: &str) -> EtlResult<T> {
    Ok(serde_yaml::from_str(yaml_str)?)
}


pub(crate) fn deserialize_columns<'de, D>(seq: Vec<Value>) -> Result<Vec<(String, Option<String>)>, D::Error>
    where D: Deserializer<'de>
{
    fn as_str<'a, E: de::Error>(value: &'a Value, error_message: &str) -> Result<&'a str, E> {
        value.as_str().ok_or_else(|| de::Error::custom(error_message))
    }

    seq
        .into_iter()
        .map(|item| {
            match item {
                Value::String(name) => Ok((name, None)),
                Value::Mapping(map) if map.len() == 1 => {
                    let (name, label) = map.into_iter().next().unwrap();
                    Ok((
                        as_str(&name, "Column name must be a string.")?.to_string(),
                        Some(as_str(&label, "Column label must be a string.")?.to_string())
                    ))
                },
                _ => Err(de::Error::custom("column entries must be strings or single-entry maps."))
            }
        })
        .collect::<Result<Vec<_>, _>>()
}
