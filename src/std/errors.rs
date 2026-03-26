use crate::errors::{EtlError, ErrorKind};


pub(super) fn invalid_date_format(date_format: &str) -> EtlError {
    EtlError::new(
        format!("Invalid date format `{date_format}`."),
        ErrorKind::InvalidFormat
    )
}


#[allow(dead_code)]
pub(super) fn value_exceeds_number_of_digits(str_value: &str) -> EtlError {
    EtlError::new(
        format!("The string value `{str_value}` exceeds expected number of digits in Cobol PIC format."),
        ErrorKind::InvalidFormat
    )
}


#[allow(dead_code)]
pub(super) fn incorrect_last_char_for_signed_value(overpunch_char: char) -> EtlError {
    EtlError::new(
        format!("Invalid last char `{overpunch_char}` for signed decimal value."),
        ErrorKind::InvalidFormat
    )
}


#[allow(dead_code)]
pub(super) fn invalid_cobol_pic_decimal_value(str_value: &str) -> EtlError {
    EtlError::new(
        format!("Cannot parse the integer part of PIC value: `{str_value}`."),
        ErrorKind::InvalidFormat
    )
}


#[allow(dead_code)]
pub(super) fn invalid_cobol_pic_format(pic_format: &str) -> EtlError {
    EtlError::new(
        format!("Invalid Cobol decimal PIC format: `{pic_format}`."),
        ErrorKind::InvalidFormat
    )
}


#[allow(dead_code)]
pub(super) fn blank_pic_decimal_string() -> EtlError {
    EtlError::new(
        "Cannot parse a blank string to PIC decimal format.",
        ErrorKind::InvalidFormat
    )
}
