use std::error::Error;
use std::fmt::{Display, Formatter};
use std::path::Path;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ErrorKind {
    #[allow(dead_code)]
    DBConnectionError,

    FileDoesNotExist,
    InvalidConfiguration,
    InvalidFormat,
    InvalidTemplateFormat,
    InvalidTemplateName,
    IOError,
    ParseError,
    RegexError,
    TemplateNotFound,
    YamlDeserializationError
}


#[derive(Debug)]
pub struct EtlError {
    message: String,
    kind: ErrorKind,
    row_id: Option<usize>,
    file_path: Option<String>,
    inner_error: Option<Box<dyn Error + Send + Sync>>
}


impl Display for EtlError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}


impl Error for EtlError {
}


impl From<std::io::Error> for EtlError {
    fn from(err: std::io::Error) -> Self {
        EtlError::from_error(err, ErrorKind::IOError, "IO error")
    }
}


impl From<regex::Error> for EtlError {
    fn from(err: regex::Error) -> Self {
        EtlError::from_error(err, ErrorKind::RegexError, "Regex error")
    }
}


impl From<serde_yaml::Error> for EtlError {
    fn from(err: serde_yaml::Error) -> Self {
        EtlError::from_error(err, ErrorKind::YamlDeserializationError, "YAML deserialization")
    }
}


impl From<chrono::ParseError> for EtlError {
    fn from(err: chrono::ParseError) -> Self {
        EtlError::from_error(err, ErrorKind::ParseError, "Parse date error")
    }
}


impl EtlError {
    fn message(&self) -> &str {
        &self.message
    }


    pub(crate) fn kind(&self) -> ErrorKind {
        self.kind
    }


    #[allow(dead_code)]
    fn row_id(&self) -> Option<usize> {
        self.row_id
    }


    #[allow(dead_code)]
    fn file_path(&self) -> Option<&str> {
        self.file_path.as_deref()
    }


    #[allow(dead_code)]
    fn inner_error(&self) -> Option<String> {
        self.inner_error
            .as_ref()
            .map(ToString::to_string)
    }


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
}


pub(crate) fn file_does_not_exist(file_path: &Path) -> EtlError {
    EtlError::new(
        format!("The file {file_path:?} doesn't exist."),
        ErrorKind::FileDoesNotExist
    )
}
