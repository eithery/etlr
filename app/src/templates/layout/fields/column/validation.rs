use etl_macros::DeserializeYaml;
use serde::Deserialize;
use serde_yaml::Value;
use crate::errors::EtlError;
use crate::yaml::{YamlReader, errors as err};


#[derive(Debug, DeserializeYaml)]
pub(crate) struct ColumnValidationTemplate {
    validate: bool,
    rule: Option<String>,
    reject_invalid: Option<InvalidValueHandling>
}


impl ColumnValidationTemplate {
    #[allow(dead_code)]
    pub(crate) fn validate(&self) -> bool {
        self.validate
    }


    #[allow(dead_code)]
    pub(crate) fn rule(&self) -> Option<&str> {
        self.rule.as_deref()
    }


    #[allow(dead_code)]
    pub(crate) fn reject_invalid(&self) -> Option<InvalidValueHandling> {
        self.reject_invalid
    }


    fn new(validate: bool) -> Self {
        Self { validate, rule: None, reject_invalid: None }
    }


    fn with_rule(rule: String) -> Self {
        Self { validate: true, rule: Some(rule), reject_invalid: None }
    }
}


impl TryFrom<&Value> for ColumnValidationTemplate {
    type Error = EtlError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Bool(validate) => Ok(Self::new(*validate)),
            Value::String(rule) => Ok(Self::with_rule(rule.to_owned())),
            Value::Mapping(m) => {
                Ok(Self{
                    validate: true,
                    rule: m.get_opt_string("rule")?,
                    reject_invalid: m.get_opt_value("reject_invalid")?
                })
            }
            _ => Err(invalid_validate_value())
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum InvalidValueHandling {
    Preserve,
    Reject,
    RejectField,
    RejectRow,
    RejectFile
}


impl TryFrom<&Value> for InvalidValueHandling {
    type Error = EtlError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Bool(reject) => Ok(if *reject { Self::Reject } else { Self::Preserve }),
            Value::String(kind) => {
                match kind.as_str() {
                    ":field" => Ok(Self::RejectField),
                    ":row" => Ok(Self::RejectRow),
                    ":file" => Ok(Self::RejectFile),
                    _ => Err(invalid_reject_invalid_value())
                }
            }
            _ => Err(invalid_reject_invalid_value())
        }
    }
}


fn invalid_validate_value() -> EtlError {
    err::invalid_yaml_value("validate", "Expected a mapping, string, or boolean")
}


fn invalid_reject_invalid_value() -> EtlError {
    err::invalid_yaml_value(
        "reject_invalid",
        "Expected values: `:field`, `:row`, `:file`, or boolean"
    )
}
