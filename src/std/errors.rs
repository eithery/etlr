use crate::errors::{EtlError, ErrorKind};


pub(super) fn invalid_date_format(date_format: &str) -> EtlError {
    EtlError::new(
        format!("Invalid date format `{date_format}`."),
        ErrorKind::InvalidFormat
    )
}
