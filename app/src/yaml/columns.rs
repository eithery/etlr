use serde_yaml::Value;
use crate::errors::{EtlError, ErrorKind};
use crate::std::result::Result;
use super::YamlNameValueMap;


pub(crate) trait LabeledColumns {
    fn to_labeled_columns(&self) -> Result<Vec<(String, Option<String>)>>;
}


impl LabeledColumns for Vec<Value> {
    fn to_labeled_columns(&self) -> Result<Vec<(String, Option<String>)>> {
        self
            .into_iter()
            .map(|item| {
                let (name, value) = item.to_name_value_map()?;
                match value {
                    Value::String(label) => Ok((name, Some(label.clone()))),
                    Value::Null => Ok((name, None)),
                    _ => Err(invalid_labeled_column())
                }
            })
            .collect::<Result<Vec<_>>>()
    }
}


fn invalid_labeled_column() -> EtlError {
    EtlError::new(
        "Invalid labeled column format. Expected to be a string or `name: label` single entry map.",
        ErrorKind::YamlFormatError
    )
}
