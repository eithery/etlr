use std::default::Default;
use std::{fs, io};
use std::path::{Path, PathBuf};
use serde::Deserialize;
use crate::{cli, path};
use crate::config::defaults;
use crate::env::{self, Environment};
use crate::fs::yaml;
use crate::std::string;
use crate::std::vector::Prepend;
use super::database::DatabaseConfiguration;
use super::logging::LoggingConfiguration;
use super::workflow::WorkflowConfiguration;


const CONFIG_FILE_NAME: &str = ".etlrc.yml";
const DEV_CONFIG_FILE_NAME: &str = ".etlrc.dev.yml";
const TEST_CONFIG_FILE_NAME: &str = ".etlrc.test.yml";
const PROD_CONFIG_FILE_NAME: &str = ".etlrc.prod.yml";


#[derive(Debug, Deserialize)]
pub(crate) struct AppConfiguration {
    #[serde(default)]
    database: DatabaseConfiguration,

    #[serde(default)]
    inbox: Vec<String>,

    #[serde(default, deserialize_with = "string::deserialize")]
    outbox: Option<String>,

    #[serde(default)]
    templates: Vec<String>,

    #[serde(default)]
    workflow: WorkflowConfiguration,

    #[serde(default)]
    logging: LoggingConfiguration,

    #[serde(default)]
    load_paths: Vec<PathBuf>
}


impl Default for AppConfiguration {
    fn default() -> Self {
        Self {
            database: Default::default(),
            inbox: defaults::app::inbox(),
            outbox: defaults::app::outbox(),
            templates: Vec::new(),
            workflow: Default::default(),
            logging: Default::default(),
            load_paths: Vec::new()
        }
    }
}


impl AppConfiguration {
    pub(crate) fn load(config_path: Option<&str>) -> Self {
        Self::default()
            .load_from_home()
            .load_from_dir(path::config_dir)
            .load_env_config()
            .load_from_dir(path::current_config_dir)
            .load_from_dir(path::current_dir)
            .load_from_env()
            .load_from_option(config_path)
            .apply_env_vars()
    }


    pub(crate) fn template_dirs(&self) -> impl Iterator<Item = PathBuf> {
        self.templates.iter().map(|p| {
            let path = Path::new(p);
            fs::canonicalize(path).expect(&format!("Invalid template path `{path:?}`"))
        })
    }


    fn load_from_home(self) -> Self {
        self.load_config(path::home_dir, CONFIG_FILE_NAME)
    }


    fn load_from_env(self) -> Self {
        self.load_config(path::config_env_dir, CONFIG_FILE_NAME)
    }


    fn load_from_option(self, config_path: Option<&str>) -> Self {
        match config_path {
            Some(path) => self.load_config(|| Ok(PathBuf::from(path)), CONFIG_FILE_NAME),
            None => self
        }
    }


    fn load_from_dir<F>(self, config_dir: F) -> Self
        where F: FnOnce() -> io::Result<PathBuf> {
        self.load_config(config_dir, CONFIG_FILE_NAME)
    }


    fn load_env_config(self) -> Self {
        let file_name = match env::current_environment() {
            Environment::Development => DEV_CONFIG_FILE_NAME,
            Environment::Testing => TEST_CONFIG_FILE_NAME,
            Environment::Production => PROD_CONFIG_FILE_NAME
        };
        self.load_config(path::config_dir, file_name)
    }


    fn load_config<F>(self, config_dir: F, file_name: &str) -> Self
        where F: FnOnce() -> io::Result<PathBuf> {
        match config_dir() {
            Ok(path) => {
                let config_path = path.join(file_name);
                if self.load_paths.contains(&config_path) {
                    return self;
                }
                match yaml::load_from_file(&config_path) {
                    Ok(config) => {
                        cli::file_loaded(&config_path);
                        self.merge(config, config_path)
                    }
                    Err(_) => self // TODO: Log all invalid config cases
                }
            },
            Err(_) => self
        }
    }


    fn merge(self, other: Self, config_path: PathBuf) -> Self {
        Self {
            database: self.database.merge(other.database),
            inbox: self.inbox.prepend(other.inbox),
            outbox: other.outbox.or(self.outbox),
            templates: self.templates.prepend(other.templates),
            workflow: self.workflow.merge(other.workflow),
            logging: self.logging.merge(other.logging),
            load_paths: [self.load_paths.as_slice(), &[config_path]].concat()
        }
    }


    fn apply_env_vars(self) -> Self {
        cli::env_vars_applied();
        Self {
            database: self.database.apply_env_vars(),
            logging: self.logging.apply_env_vars(),
            ..self
        }
    }
}
