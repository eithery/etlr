use crate::errors::{EtlError, ErrorKind};


pub(super) fn invalid_connection_type(connection_type: &str) -> EtlError {
    EtlError::new(
        format!("Invalid database connection type `{connection_type}`.\n\
            Expected one of `:default`, `:auto`, `:sspi`, `:windows`, `:username`."),
        ErrorKind::InvalidConfiguration
    )
}


pub(super) fn user_credentials_not_specified() -> EtlError {
    EtlError::new(
        "DB user credentials are missing or incomplete.",
        ErrorKind::InvalidConfiguration
    )
}
