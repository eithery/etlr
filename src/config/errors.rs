use crate::errors::{EtlError, ErrorKind};


pub(super) fn inbox_not_defined() -> EtlError {
    EtlError::new(
        "Inbox path is not defined.",
        ErrorKind::InvalidConfiguration
    )
}


pub(super) fn outbox_not_defined() -> EtlError {
    EtlError::new(
        "Outbox path is not defined.",
        ErrorKind::InvalidConfiguration
    )
}
