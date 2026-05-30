use crate::errors::{EtlError, ErrorKind};


pub(crate) fn invalid_yaml_value(tag_name: &str, details: &str) -> EtlError {
    invalid_yaml_error("Invalid value", tag_name, details)
}


pub(crate) fn invalid_yaml_format(tag_name: &str, details: &str) -> EtlError {
    invalid_yaml_error("Invalid format", tag_name, details)
}


pub(super) fn missing_required_yaml_value(tag_name: &str) -> EtlError {
    let error_msg = format!("Missing required `{tag_name}` value.");
    EtlError::new(error_msg, ErrorKind::YamlFormatError)
}


fn invalid_yaml_error(kind: &str, tag_name: &str, details: &str) -> EtlError {
    let error_msg = format!("{kind} for `{tag_name}`. {details}.");
    EtlError::new(error_msg, ErrorKind::YamlFormatError)
}
