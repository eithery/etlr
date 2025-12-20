use std::fs::File;
use std::path::Path;
use serde::de::DeserializeOwned;
use crate::errors as err;
use crate::std::result::Result;


pub(crate) fn load_from_file<T: DeserializeOwned>(file_path: &Path) -> Result<T> {
    if !file_path.is_file() {
        return err::file_does_not_exist(file_path);
    }

    let file = File::open(file_path)?;
    Ok(serde_yaml::from_reader(file)?)
}
