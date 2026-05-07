pub(crate) mod size;
pub(crate) mod validation;

use std::ops::Deref;
use etl_macros::DeserializeYaml;
use serde::Deserialize;
use serde_yaml::Value;
use crate::errors::EtlError;
use crate::fs::yaml::{YamlReader, YamlNameValueMap, invalid_yaml_format};
use super::data_element::DataElementTemplate;
use size::ColumnSize;
use validation::ColumnValidationTemplate;


#[derive(Debug, DeserializeYaml)]
pub(crate) struct ColumnTemplate {
    name: String,
    base: DataElementTemplate,
    size: ColumnSize,
    validate: Option<ColumnValidationTemplate>
}


impl ColumnTemplate {
    #[allow(dead_code)]
    pub(crate) fn name(&self) -> &str {
        &self.name
    }


    #[allow(dead_code)]
    pub(crate) fn size(&self) -> ColumnSize {
        self.size
    }


    #[allow(dead_code)]
    pub(crate) fn validate(&self) -> Option<&ColumnValidationTemplate> {
        self.validate.as_ref()
    }


    fn new(column_name: String) -> Self {
        Self {
            name: column_name,
            base: DataElementTemplate::default(),
            size: ColumnSize::Default,
            validate: None
        }
    }
}


impl Deref for ColumnTemplate {
    type Target = DataElementTemplate;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}


impl TryFrom<&Value> for ColumnTemplate {
    type Error = EtlError;

    fn try_from(payload: &Value) -> Result<Self, Self::Error> {
        let (column_name, value) = payload.to_name_value_map()?;
        match value {
            Value::Mapping(m) => {
                Ok(Self {
                    name: column_name,
                    base: DataElementTemplate::deserialize(value)?,
                    size: m.get_value_or_default("size")?,
                    validate: m.get_opt_value("validate")?
                })
            }
            Value::Null => Ok(Self::new(column_name)),
            _ => Err(invalid_column_format())
        }
    }
}


fn invalid_column_format() -> EtlError {
    invalid_yaml_format("column", "Expected a mapping or empty value")
}
