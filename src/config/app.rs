use std::default::Default;
use std::io;
use std::path::PathBuf;
use dotenv::dotenv;
use serde::Deserialize;
use crate::env::{self, Environment};
use crate::path;
use super::database::DatabaseConfiguration;
use super::yaml;


const CONFIG_FILE_NAME: &str = ".etlrc.yml";
const DEV_CONFIG_FILE_NAME: &str = ".etlrc.dev.yml";
const TEST_CONFIG_FILE_NAME: &str = ".etlrc.test.yml";
const PROD_CONFIG_FILE_NAME: &str = ".etlrc.prod.yml";


#[derive(Debug, Deserialize, Default)]
pub(crate) struct AppConfiguration {
    #[serde(default)]
    pub(crate) database: DatabaseConfiguration
}


impl AppConfiguration {
    pub(crate) fn load(config_path: Option<&str>) -> Self {
        dotenv().ok();
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
        println!("Load file {file_name}");
        self.load_config(path::config_dir, file_name)
    }


    fn load_config<F>(self, config_dir: F, file_name: &str) -> Self
        where F: FnOnce() -> io::Result<PathBuf> {
        match config_dir() {
            Ok(path) => {
                let config_path = path.join(file_name);
                match yaml::load_from_file(&config_path) {
                    Ok(config) => self.merge_config(config),
                    Err(err) => {
                        println!("{err}");
                        self
                    }
                }
            },
            Err(_) => self
        }
    }


    fn merge_config(self, other: Self) -> Self {
        Self {
            database: self.database.merge(other.database)
        }
    }


    fn apply_env_vars(self) -> Self {
        Self {
            database: self.database.apply_env_vars()
        }
    }
}
