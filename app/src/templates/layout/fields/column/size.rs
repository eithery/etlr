use serde_yaml::Value;
use crate::errors::EtlError;
use crate::fs::yaml::invalid_yaml_value;


const ANY_SIZE_TAG: &str = ":any";


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ColumnSize {
    Sized(usize),
    Default,
    Any
}


impl TryFrom<&Value> for ColumnSize {
    type Error = EtlError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Number(number) => {
                number.as_u64()
                    .map(|n| Self::Sized(n as usize))
                    .ok_or_else(invalid_column_size)
            }
            Value::String(s) if s == ANY_SIZE_TAG => Ok(Self::Any),
            _ => Err(invalid_column_size())
        }
    }
}


impl Default for ColumnSize {
    fn default() -> Self {
        Self::Default
    }
}


fn invalid_column_size() -> EtlError {
    invalid_yaml_value("column.size", "Expected a number or `:any` value")
}
