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
    #[allow(dead_code)]
    pub(super) fn enabled(&self) -> bool {
        self.enabled.unwrap_or(false)
    }


    #[allow(dead_code)]
    pub(super) fn level(&self) -> Option<LogLevel> {
        self.level
    }


    pub(super) fn merge(self, other: Self) -> Self {
        Self {
            enabled: other.enabled.or(self.enabled),
            level: other.level.or(self.level)
        }
    }
}
