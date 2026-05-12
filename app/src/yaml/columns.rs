use serde_yaml::Value;
use crate::errors::{EtlError, ErrorKind};
use crate::std::result::Result;


pub(crate) trait LabeledColumns {
    fn to_labeled_columns(&self) -> Result<Vec<(String, Option<String>)>>;
}


impl LabeledColumns for Vec<Value> {
    fn to_labeled_columns(&self) -> Result<Vec<(String, Option<String>)>> {
        self
            .into_iter()
            .map(to_labeled_column)
            .collect::<Result<Vec<_>>>()
    }
}


fn to_labeled_column(value: &Value) -> Result<(String, Option<String>)> {
    match value {
        Value::String(name) => Ok((name.clone(), None)),
        Value::Mapping(m) if m.len() == 1 => {
            let (key, val) = m.iter().next().unwrap();
            let name = value_to_string(key)?;
            let label = value_to_string(val)?;
            Ok((name, Some(label)))
        }
        _ => Err(invalid_labeled_column())
    }
}


fn value_to_string(value: &Value) -> Result<String> {
    value.as_str()
        .map(str::to_string)
        .ok_or_else(invalid_labeled_column)
}


fn invalid_labeled_column() -> EtlError {
    EtlError::new(
        "Invalid labeled column format. Expected to be a string or `name: label` single entry map.",
        ErrorKind::YamlFormatError
    )
}
