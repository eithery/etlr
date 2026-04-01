use std::path::{Path, PathBuf};
use either::Either;
use serde::de::DeserializeOwned;
use crate::cli;
use crate::fs::yaml;
use crate::errors::ErrorKind;
use crate::std::result::Result;
use crate::templates::layout::RecordLayoutTemplate;
use super::errors as err;
use super::INTEGRATED_TEMPLATES;


const ALL_YAML: &str = "**/*.yml";


pub(crate) trait FileTemplate: Sized + DeserializeOwned {
    const TEMPLATES_ROOT: &'static str;


    fn file_type(&self) -> &str;
    
    fn layout(&self) -> &RecordLayoutTemplate;


    fn load<I, P>(template_name: &str, template_dirs: I) -> Result<Self>
        where I: IntoIterator<Item = Result<P>>, P: AsRef<Path>
    {
        cli::echo("Lookup file template:");
        for path in template_dirs {
            match Self::load_template(template_name, path?.as_ref()) {
                Ok(template) => return Ok(template),
                Err(err) if err.kind() == ErrorKind::FileDoesNotExist => continue,
                Err(err) => return Err(err)
            }
        }
        Self::load_integrated_template(template_name)
    }


    #[allow(dead_code)]
    fn list<I, P>(_template_dirs: I) -> Vec<String>
        where I: IntoIterator<Item = Result<P>>, P: AsRef<Path>
    {
        Self::list_integrated_templates()
            .unwrap()
            .map(|(file_type, _)| file_type)
            .collect()
    }


    fn included_file_types(&self) -> impl Iterator<Item = &str> {
        if self.layout().has_single_file() {
            Either::Left(std::iter::once(self.file_type()))
        } else {
            Either::Right(self.layout().included_file_types())
        }
    }


    fn load_integrated_template(template_name: &str) -> Result<Self> {
        let file_path = Self::build_template_path(template_name, None)?;
        let template = Self::read_integrated_template(template_name, &file_path)?;
        cli::template_loaded(&file_path);
        cli::blank_line();
        Ok(template)
    }


    fn list_integrated_templates() -> Result<impl Iterator<Item = (String, String)>> {
        let templates_glob = format!("{}/{}", Self::TEMPLATES_ROOT, ALL_YAML);
        let iter = INTEGRATED_TEMPLATES
            .find(&templates_glob)
            .unwrap()
            .filter_map(|entry| {
                let file_path = entry.path();
                let name = file_path.to_str()?;
                let template = Self::read_integrated_template(name, file_path).ok()?;
                Some(
                    template
                        .included_file_types()
                        .map(|s| (s.to_string(), template.file_type().to_string()))
                        .collect::<Vec<_>>()
                        .into_iter()
                )
            })
            .flatten();
        Ok(iter)
    }


    #[allow(dead_code)]
    fn list_external_templates() -> Result<impl Iterator<Item = (String, String)>> {
        Ok(vec![].into_iter())
    }


    fn load_template(template_name: &str, template_path: &Path) -> Result<Self> {
        let file_path = Self::build_template_path(template_name, Some(template_path))?;
        let template = yaml::load_from_file(&file_path)?;
        cli::file_loaded(&file_path);
        cli::blank_line();
        Ok(template)
    }


    fn build_template_path(template_name: &str, template_path: Option<&Path>) -> Result<PathBuf> {
        let (file_category, file_name) = split_template_name(template_name)?;
        let file_path = Path::new(Self::TEMPLATES_ROOT)
            .join(file_category)
            .join(format!("{file_name}.yml"));
        match template_path {
            Some(path) => Ok(path.join(file_path)),
            None => Ok(file_path)
        }
    }


    fn read_integrated_template(template_name: &str, file_path: &Path) -> Result<Self> {
        INTEGRATED_TEMPLATES
            .get_file(file_path)
            .ok_or_else(|| err::template_not_found(template_name))
            .and_then(|file| {
                file.contents_utf8()
                    .ok_or_else(|| err::invalid_template_format(template_name))
            })
            .and_then(|content| yaml::load_from_str(content))
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
