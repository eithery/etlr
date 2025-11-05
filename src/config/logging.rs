mod database;
mod file;
mod severity;

pub(crate) use severity::LogLevel;
use serde::Deserialize;
use database::DatabaseLogConfiguration;
use file::FileLogConfiguration;


#[derive(Debug, Deserialize, Default)]
pub(super) struct LoggingConfiguration {
    #[serde(default)]
    file: FileLogConfiguration,

    #[serde(default)]
    database: DatabaseLogConfiguration
}


impl LoggingConfiguration {
    pub(super) fn merge(self, other: Self) -> Self {
        Self {
            file: self.file.merge(other.file),
            database: self.database.merge(other.database)
        }
    }
}
