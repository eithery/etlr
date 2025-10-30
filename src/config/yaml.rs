use std::fs::File;
use std::path::Path;
use anyhow::Result;
use serde::de::DeserializeOwned;
use crate::config::errors as e;


pub(crate) fn load_from_file<T: DeserializeOwned>(file_path: &Path) -> Result<T> {
    if !file_path.is_file() {
        return Err(e::file_does_not_exist(file_path));
    }

    let file = File::open(file_path)?;
    Ok(serde_yaml::from_reader(file)?)
}
