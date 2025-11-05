use serde::Deserialize;
use super::LogLevel;


#[derive(Debug, Deserialize)]
pub(super) struct DatabaseLogConfiguration {
    enabled: Option<bool>,
    level: Option<LogLevel>
}


impl DatabaseLogConfiguration {
    pub(super) fn merge(self, other: Self) -> Self {
        Self {
            enabled: other.enabled.or(self.enabled),
            level: other.level.or(self.level)
        }
    }
}


impl Default for DatabaseLogConfiguration {
    fn default() -> Self {
        Self {
            enabled: Some(true),
            level: Some(LogLevel::Warning)
        }
    }
}
