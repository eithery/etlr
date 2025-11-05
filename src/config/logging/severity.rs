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
    Critical,

    #[serde(other)]
    Unknown
}
