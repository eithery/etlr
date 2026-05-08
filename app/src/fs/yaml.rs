use std::fs::File;
use std::path::Path;
use serde::{Deserializer, de};
use serde::de::DeserializeOwned;
use serde_yaml::{Value, Mapping};
use crate::errors as err;
use crate::errors::{EtlError, ErrorKind};
use crate::std::result::EtlResult;


pub(crate) trait YamlNameValueMap {
    fn to_name_value_map(&self) -> EtlResult<(String, &Value)>;
}


impl YamlNameValueMap for &Value {
    fn to_name_value_map(&self) -> EtlResult<(String, &Value)> {
        match self {
            Value::Mapping(map) if map.len() == 1 => {
                let (key, val) = map.iter().next().unwrap();
                let name = key
                    .as_str()
                    .map(str::to_string)
                    .ok_or_else(|| invalid_name_value_map("The key element must be a string."))?;

                Ok((name, val))
            }
            _ => Err(invalid_name_value_map("YAML entries must be single-entry maps."))
        }
    }
}


pub(crate) trait YamlReader {
    fn get_opt_str(&self, tag_name: &str) -> EtlResult<Option<String>>;

    #[allow(dead_code)]
    fn get_opt_usize(&self, tag_name: &str) -> EtlResult<Option<usize>>;

    fn get_bool(&self, tag_name: &str, default: bool) -> EtlResult<bool>;

    fn get_value<'a, T>(&'a self, tag_name: &str) -> EtlResult<T>
        where T: TryFrom<&'a Value, Error = EtlError>;

    fn get_opt_value<'a, T>(&'a self, tag_name: &str) -> EtlResult<Option<T>>
        where T: TryFrom<&'a Value, Error = EtlError>;

    fn get_value_or_default<'a, T>(&'a self, tag_name: &str) -> EtlResult<T>
        where T: TryFrom<&'a Value, Error = EtlError> + Default;

    fn get_vec<'a, T>(&'a self, tag_name: &str) -> EtlResult<Vec<T>>
        where T: TryFrom<&'a Value, Error = EtlError>;
}


impl YamlReader for &Mapping {
    fn get_opt_str(&self, tag_name: &str) -> EtlResult<Option<String>> {
        self.get(tag_name)
            .map(|val| {
                val.as_str()
                    .map(String::from)
                    .ok_or_else(|| invalid_yaml_value(tag_name, "Expected string"))
            })
            .transpose()
    }


    fn get_opt_usize(&self, tag_name: &str) -> EtlResult<Option<usize>> {
        self.get(tag_name)
            .map_or(Ok(None), |val| {
                val.as_u64()
                    .map(|n| Some(n as usize))
                    .ok_or_else(|| invalid_yaml_value(tag_name, "Expected number"))
            })
    }


    fn get_bool(&self, tag_name: &str, default: bool) -> EtlResult<bool> {
        self.get(tag_name)
            .map_or(Ok(default), |val| {
                val.as_bool()
                    .ok_or_else(|| invalid_yaml_value(tag_name, "Expected boolean"))
            })
    }


    fn get_value<'a, T>(&'a self, tag_name: &str) -> EtlResult<T>
        where T: TryFrom<&'a Value, Error = EtlError>
    {
        self.get(tag_name)
            .ok_or_else(|| missing_required_yaml_value("pos"))?
            .try_into()
    }


    fn get_opt_value<'a, T>(&'a self, tag_name: &str) -> EtlResult<Option<T>>
        where T: TryFrom<&'a Value, Error = EtlError>
    {
        self.get(tag_name)
            .map(TryInto::try_into)
            .transpose()
    }


    fn get_value_or_default<'a, T>(&'a self, tag_name: &str) -> EtlResult<T>
        where T: TryFrom<&'a Value, Error = EtlError> + Default
    {
        self.get_opt_value(tag_name)
            .map(Option::unwrap_or_default)
    }


    fn get_vec<'a, T>(&'a self, tag_name: &str) -> EtlResult<Vec<T>>
        where T: TryFrom<&'a Value, Error = EtlError>
    {
        let value = self.get(tag_name)
            .ok_or_else(|| missing_required_yaml_value(tag_name))?;
        match value {
            Value::Sequence(items) => {
                items.iter()
                    .map(TryInto::try_into)
                    .collect()
            }
            _ => Err(invalid_yaml_format(tag_name, "Expected a sequence"))
        }
    }
}


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


pub(crate) fn invalid_yaml_value(tag_name: &str, details: &str) -> EtlError {
    invalid_yaml_error("Invalid value", tag_name, details)
}


pub(crate) fn invalid_yaml_format(tag_name: &str, details: &str) -> EtlError {
    invalid_yaml_error("Invalid format", tag_name, details)
}


fn missing_required_yaml_value(tag_name: &str) -> EtlError {
    let error_msg = format!("Missing required `{tag_name}` value.");
    EtlError::new(error_msg, ErrorKind::YamlFormatError)
}


fn invalid_name_value_map(message: &str) -> EtlError {
    let error_msg = format!("Invalid name/value map format. {message}");
    EtlError::new(error_msg, ErrorKind::YamlFormatError)
}


fn invalid_yaml_error(kind: &str, tag_name: &str, details: &str) -> EtlError {
    let error_msg = format!("{kind} for `{tag_name}`. {details}.");
    EtlError::new(error_msg, ErrorKind::YamlFormatError)
}
