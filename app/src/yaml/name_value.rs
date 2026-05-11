use serde_yaml::Value;
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


fn invalid_name_value_map(message: &str) -> EtlError {
    let error_msg = format!("Invalid name/value map format. {message}");
    EtlError::new(error_msg, ErrorKind::YamlFormatError)
}
