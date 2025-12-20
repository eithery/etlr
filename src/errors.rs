use std::error::Error;
use std::fmt::{Display, Formatter};
use std::path::Path;
use crate::std::result::Result;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ErrorKind {
    FileDoesNotExist,
    InvalidTemplateName,
    IOError,
    TemplateNotFound,
    YamlDeserializationError
}


#[derive(Debug)]
#[allow(dead_code)]
pub struct EtlError {
    message: String,
    kind: ErrorKind,
    row_id: Option<u32>,
    file_path: Option<String>,
    inner_error: Option<Box<dyn Error + Send + Sync>>
}


impl Display for EtlError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}


impl Error for EtlError {
}


impl From<std::io::Error> for EtlError {
    fn from(err: std::io::Error) -> Self {
        EtlError::from_error(err, ErrorKind::IOError, "IO error")
    }
}


impl From<serde_yaml::Error> for EtlError {
    fn from(err: serde_yaml::Error) -> Self {
        EtlError::from_error(err, ErrorKind::YamlDeserializationError, "YAML deserialization")
    }
}


impl EtlError {
    pub(crate) fn new<S: Into<String>>(message: S, kind: ErrorKind) -> Self {
        Self {
            message: message.into(),
            kind,
            row_id: None,
            file_path: None,
            inner_error: None
        }
    }


    fn from_error<E>(error: E, kind: ErrorKind, prefix: &str) -> Self
        where E: Error + Send + Sync + 'static
    {
        Self {
            message: format!("{prefix}. {error}"),
            kind,
            row_id: None,
            file_path: None,
            inner_error: Some(Box::new(error))
        }
    }


    pub(crate) fn kind(&self) -> ErrorKind {
        self.kind
    }
}


pub(crate) fn file_does_not_exist<T>(file_path: &Path) -> Result<T> {
    Err(EtlError::new(
        format!("The file {file_path:?} doesn't exist."),
        ErrorKind::FileDoesNotExist
    ))
}
