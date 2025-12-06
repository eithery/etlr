use std::path::Path;
use anyhow::Result;
use serde::de::DeserializeOwned;
use crate::cli;
use crate::config::yaml;
use super::errors as e;


pub(crate) trait FileTemplate: Sized + DeserializeOwned {
    fn load<I, P>(template_name: &str, template_dirs: I) -> Result<Self>
        where I: IntoIterator<Item = P>, P: AsRef<Path>
    {
        cli::echo("Lookup file template:");
        let (source, name) = split_template_name(template_name)?;
        for path in template_dirs {
            let template_path = path.as_ref();
            let res = Self::load_template(source, name, template_path);
            if res.is_ok() {
                return res
            }
        }
        Err(e::template_not_found(template_name))
    }


    fn load_template(file_source: &str, file_name: &str, template_path: &Path) -> Result<Self> {
        let file_path = template_path.join(file_source).join(format!("{file_name}.yml"));
        match yaml::load_from_file(&file_path) {
            Ok(template) => {
                return Ok(template)
            },
            Err(_) => Err(e::template_not_found(&format!("{file_source}.{file_name}")))
        }
    }
}



fn split_template_name(template_name: &str) -> Result<(&str, &str)> {
    let err = || e::invalid_template_name(template_name);
    let separators = ['.', '/'];

    let (source, name) = template_name.split_once(separators).ok_or_else(err)?;
    if source.is_empty() || name.is_empty() || name.contains(separators) {
        return Err(err());
    }
    Ok((source, name))
}
