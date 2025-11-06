mod base;
mod database;
mod defaults;
mod file;
mod severity;
mod splunk;

pub(self) use base::BaseLogConfiguration;
pub(self) use severity::LogLevel;
use serde::Deserialize;
use database::DatabaseLogConfiguration;
use file::FileLogConfiguration;
use splunk::SplunkConfiguration;


#[derive(Debug, Deserialize, Default)]
pub(super) struct LoggingConfiguration {
    #[serde(default)]
    file: FileLogConfiguration,

    #[serde(default)]
    database: DatabaseLogConfiguration,

    #[serde(default)]
    splunk: SplunkConfiguration
}


impl LoggingConfiguration {
    pub(super) fn merge(self, other: Self) -> Self {
        Self {
            file: self.file.merge(other.file),
            database: self.database.merge(other.database),
            splunk: self.splunk.merge(other.splunk)
        }
    }


    pub(super) fn apply_env_vars(self) -> Self {
        Self {
            splunk: self.splunk.apply_env_vars(),
            ..self
        }
    }
}
