use std::str::FromStr;
use etl_macros::DeserializeYaml;
use serde::Deserialize;
use serde_yaml::{Value, Number};
use crate::errors::EtlError;
use crate::fs::yaml::invalid_yaml_value;


const DELIMITER: &str = "..";


#[derive(Debug, Clone, Copy, PartialEq, Eq, DeserializeYaml)]
pub(crate) struct FieldPosition {
    start: usize,
    end: usize
}


impl FieldPosition {
    pub(crate) fn start(&self) -> usize {
        self.start
    }


    pub(crate) fn end(&self) -> usize {
        self.end
    }


    pub(crate) fn len(&self) -> usize {
        self.end() - self.start() + 1
    }


    pub(crate) fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }


    pub(crate) fn single(pos: usize) -> Self {
        Self { start: pos, end: pos }
    }
}


impl FromStr for FieldPosition {
    type Err = EtlError;

    fn from_str(str_value: &str) -> Result<Self, Self::Err> {
        if let Some((s, e)) = str_value.split_once(DELIMITER) {
            let start = s.trim().parse::<usize>()?;
            let end = e.trim().parse::<usize>()?;
            Ok(Self::new(start, end))
        } else {
            let pos = str_value.trim().parse::<usize>()?;
            Ok(Self::single(pos))
        }
    }
}


impl From<u64> for FieldPosition {
    fn from(pos: u64) -> Self {
        (pos as usize).into()
    }
}


impl From<usize> for FieldPosition {
    fn from(pos: usize) -> Self {
        Self::single(pos)
    }
}


impl TryFrom<&str> for FieldPosition {
    type Error = EtlError;

    fn try_from(str_value: &str) -> Result<Self, Self::Error> {
        str_value.parse()
    }
}


impl TryFrom<&Number> for FieldPosition {
    type Error = EtlError;

    fn try_from(num: &Number) -> Result<Self, Self::Error> {
        num.as_u64()
            .ok_or_else(invalid_field_position_value)
            .map(Into::into)
    }
}


impl TryFrom<&Value> for FieldPosition {
    type Error = EtlError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::String(str_value) => str_value.parse(),
            Value::Number(n) => n.try_into(),
            _ => Err(invalid_field_position_value())
        }
    }
}


fn invalid_field_position_value() -> EtlError {
    invalid_yaml_value("field.pos", "Expected a number or a string")
}
