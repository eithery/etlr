use std::path::{Path, PathBuf};
use serde::de::DeserializeOwned;
use include_dir::File;
use crate::cli;
use crate::fs::yaml;
use crate::errors::ErrorKind;
use crate::std::result::Result;
use super::errors as err;
use super::INTEGRATED_TEMPLATES;


pub(crate) trait FileTemplate: Sized + DeserializeOwned {
    const TEMPLATES_ROOT: &'static str;


    fn load<I, P>(template_name: &str, template_dirs: I) -> Result<Self>
        where I: IntoIterator<Item = P>, P: AsRef<Path>
    {
        cli::echo("Lookup file template:");
        for path in template_dirs {
            match Self::load_template(template_name, path.as_ref()) {
                Ok(template) => return Ok(template),
                Err(err) if err.kind() == ErrorKind::FileDoesNotExist => continue,
                Err(err) => return Err(err)
            }
        }
        Self::load_integrated_template(template_name)
    }


    fn load_integrated_template(template_name: &str) -> Result<Self> {
        let file_path = Self::build_template_path(template_name, None)?;
        let template_file = read_integrated_template(template_name, &file_path)?;
        let content = read_template_content(template_name, template_file)?;
        let template = yaml::load_from_str(content)?;
        cli::template_loaded(&file_path);
        cli::blank_line();
        Ok(template)
    }


    fn load_template(template_name: &str, template_path: &Path) -> Result<Self> {
        let file_path = Self::build_template_path(template_name, Some(template_path))?;
        let template = yaml::load_from_file(&file_path)?;
        cli::file_loaded(&file_path);
        cli::blank_line();
        Ok(template)
    }


    fn build_template_path(template_name: &str, template_path: Option<&Path>) -> Result<PathBuf> {
        let (file_source, file_name) = split_template_name(template_name)?;
        let file_path = Path::new(Self::TEMPLATES_ROOT)
            .join(file_source)
            .join(format!("{file_name}.yml"));
        match template_path {
            Some(path) => Ok(path.join(file_path)),
            None => Ok(file_path)
        }
    }
}



fn split_template_name(template_name: &str) -> Result<(&str, &str)> {
    let err = || err::invalid_template_name(template_name);
    let separators = ['.', '/'];

    let (source, name) = template_name.split_once(separators).ok_or_else(err)?;
    if source.is_empty() || name.is_empty() || name.contains(separators) {
        return Err(err::invalid_template_name(template_name));
    }
    Ok((source, name))
}


fn read_integrated_template<'a>(template_name: &str, file_path: &Path) -> Result<&'a File<'a>> {
    INTEGRATED_TEMPLATES.get_file(file_path).ok_or_else(|| err::template_not_found(template_name))
}


fn read_template_content<'a>(template_name: &str, template_file: &'a File) -> Result<&'a str> {
    template_file.contents_utf8().ok_or_else(|| err::invalid_template_content(template_name))
}
