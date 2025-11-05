use serde::Deserialize;
use crate::std::string;
use super::LogLevel;


const DEFAULT_LOG_DIR: &str = "./log";


#[derive(Debug, Deserialize)]
pub(super) struct FileLogConfiguration {
    enabled: Option<bool>,

    #[serde(default, deserialize_with = "string::deserialize")]
    path: Option<String>,

    level: Option<LogLevel>
}


impl Default for FileLogConfiguration {
    fn default() -> Self {
        Self {
            enabled: Some(true),
            path: Some(DEFAULT_LOG_DIR.to_string()),
            level: Some(LogLevel::Warning)
        }
    }
}


impl FileLogConfiguration {
    pub(super) fn merge(self, other: Self) -> Self {
        Self {
            enabled: other.enabled.or(self.enabled),
            path: other.path.or(self.path),
            level: other.level.or(self.level)
        }
    }
}
