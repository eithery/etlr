use std::io;
use std::path::Path;
use anyhow::{anyhow, Error};


pub(crate) fn file_does_not_exist(file_path: &Path) -> Error {
    let error_msg = format!("The file '{file_path:?}' doesn't exist.");
    anyhow!(io::Error::new(io::ErrorKind::NotFound, error_msg))
}
