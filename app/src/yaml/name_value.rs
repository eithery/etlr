use serde_yaml::Value;
use crate::errors::{EtlError, ErrorKind};
use crate::std::result::Result;


pub(crate) trait YamlNameValue {
    fn to_name_value(&self) -> Result<(&str, &Value)>;
}


impl YamlNameValue for &Value {
    fn to_name_value(&self) -> Result<(&str, &Value)> {
        match self {
            Value::Mapping(map) if map.len() == 1 => {
                let (key, val) = map.iter().next().unwrap();
                let name = key
                    .as_str()
                    .ok_or_else(|| invalid_name_value("The key element must be a string."))?;
                Ok((name, val))
            }
            _ => Err(invalid_name_value("YAML entries must be single-entry maps."))
        }
    }
}


fn invalid_name_value(message: &str) -> EtlError {
    let error_msg = format!("Invalid name/value map format. {message}");
    EtlError::new(error_msg, ErrorKind::YamlFormatError)
}
