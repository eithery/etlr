use std::path::Path;
use serde::de::DeserializeOwned;
use crate::cli;
use crate::fs::yaml;
use crate::errors::ErrorKind;
use crate::std::result::Result;
use super::errors as err;


pub(crate) trait FileTemplate: Sized + DeserializeOwned {
    const TEMPLATES_ROOT: &'static str;


    fn load<I, P>(template_name: &str, template_dirs: I) -> Result<Self>
        where I: IntoIterator<Item = P>, P: AsRef<Path>
    {
        cli::echo("Lookup file template:");
        let (source, name) = split_template_name(template_name)?;
        for path in template_dirs {
            let template_path = path.as_ref();
            match Self::load_template(source, name, template_path) {
                Ok(template) => return Ok(template),
                Err(err) if err.kind() == ErrorKind::FileDoesNotExist => continue,
                Err(err) => return Err(err)
            }
        }
        err::template_not_found(template_name)
    }


    fn load_template(file_source: &str, file_name: &str, template_path: &Path) -> Result<Self> {
        let file_path = template_path
            .join(Self::TEMPLATES_ROOT)
            .join(file_source)
            .join(format!("{file_name}.yml"));
        let template = yaml::load_from_file(&file_path)?;
        cli::file_loaded(&file_path);
        cli::blank_line();
        return Ok(template)
    }
}



fn split_template_name(template_name: &str) -> Result<(&str, &str)> {
    let err = || err::invalid_template_name_error(template_name);
    let separators = ['.', '/'];

    let (source, name) = template_name.split_once(separators).ok_or_else(err)?;
    if source.is_empty() || name.is_empty() || name.contains(separators) {
        return Err(err::invalid_template_name_error(template_name));
    }
    Ok((source, name))
}
