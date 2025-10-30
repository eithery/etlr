use std::default::Default;
use std::path::Path;
use serde::Deserialize;
use super::database::DatabaseConfiguration;
use super::yaml;


const CONFIG_FILE_NAME: &str = ".etlrc.yml";
// const DEV_CONFIG_FILE_NAME: &str = ".etlrc.dev.yml";
// const TEST_CONFIG_FILE_NAME: &str = ".etlrc.test.yml";
// const PROD_CONFIG_FILE_NAME: &str = ".etlrc.prod.yml";
const CONFIG_DIR: &str = "";


#[derive(Debug, Deserialize, Default)]
pub(crate) struct AppConfiguration {
    #[serde(default)]
    pub(crate) database: DatabaseConfiguration
}


impl AppConfiguration {
    pub(crate) fn load(_config_option: Option<&str>) -> Self {
        println!("Load application configuration");
        Self::load_dotenv();
        Self::default()
            .load_from_home()
            .load_from_dir(Path::new(CONFIG_DIR))
            .load_env_config(Path::new(CONFIG_DIR))
            .load_config(Path::new("./config"), ".etlrc.yml")
    }


    fn load_dotenv() {
    }


    fn load_from_home(self) -> Self {
        let home = home::home_dir().expect("Could NOT found home directory.");
        println!("Load configuration from home directory: '{home:?}'");
        self.load_config(&home, "")
    }


    fn load_from_dir(self, config_dir: &Path) -> Self {
        self.load_config(config_dir, CONFIG_FILE_NAME)
    }


    fn load_env_config(self, config_dir: &Path) -> Self {
        self.load_config(config_dir, CONFIG_FILE_NAME)
    }


    // fn load_from_env(self, env_var: &str) -> Self {
    //     self
    // }


    // fn load_from_option(self) -> Self {
    //     self
    // }


    // fn apply_env_vars(self) -> Self {
    //     self
    // }


    fn load_config(self, config_dir: &Path, file_name: &str) -> Self {
        let config_path = config_dir.join(file_name);
        match yaml::load_from_file(&config_path) {
            Ok(settings) => self.merge_config(settings, config_dir),
            Err(_) => self
        }
    }


    fn merge_config(self, other: Self, _config_dir: &Path) -> Self {
        Self {
            database: self.database.merge(other.database)
        }
    }
}
