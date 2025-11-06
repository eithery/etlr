use serde::Deserialize;
use super::LogLevel;


#[derive(Debug, Deserialize)]
pub(super) struct BaseLogConfiguration {
    enabled: Option<bool>,
    level: Option<LogLevel>
}


impl Default for BaseLogConfiguration {
    fn default() -> Self {
        Self {
            enabled: Some(false),
            level: Some(LogLevel::Warning)
        }
    }
}


impl BaseLogConfiguration {
    pub(super) fn merge(self, other: Self) -> Self {
        Self {
            enabled: other.enabled.or(self.enabled),
            level: other.level.or(self.level)
        }
    }
}
