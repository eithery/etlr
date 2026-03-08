use std::fmt::Display;
use serde::Deserialize;


#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub(crate) enum LogLevel {
    #[serde(rename = ":debug")]
    Debug,

    #[serde(rename = ":info")]
    Info,

    #[serde(rename = ":warning")]
    Warning,

    #[serde(rename = ":error")]
    Error,

    #[serde(rename = ":critical")]
    Critical
}


impl Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Debug => ":debug",
            Self::Info => ":info",
            Self::Warning => ":warning",
            Self::Error => ":error",
            Self::Critical => ":critical"
        };
        f.write_str(value)
    }
}
