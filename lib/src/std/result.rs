use crate::errors::EtlError;


pub type Result<T> = std::result::Result<T, EtlError>;
