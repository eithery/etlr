use std::ops::Deref;
use serde::Deserialize;
use crate::std::string;
use super::BaseLogConfiguration;


const DEFAULT_LOG_DIR: &str = "./log";


#[derive(Debug, Deserialize)]
pub(super) struct FileLogConfiguration {
    #[serde(flatten)]
    base: BaseLogConfiguration,

    #[serde(default, deserialize_with = "string::deserialize")]
    path: Option<String>,
}


impl Default for FileLogConfiguration {
    fn default() -> Self {
        Self {
            base: BaseLogConfiguration::default(),
            path: Some(DEFAULT_LOG_DIR.to_string()),
        }
    }
}


impl Deref for FileLogConfiguration {
    type Target = BaseLogConfiguration;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}


impl FileLogConfiguration {
    pub(super) fn merge(self, other: Self) -> Self {
        Self {
            base: self.base.merge(other.base),
            path: other.path.or(self.path),
        }
    }
}
