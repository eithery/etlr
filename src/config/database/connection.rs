use serde::Deserialize;


#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub(super) enum ConnectionType {
    #[serde(rename = ":default")]
    Default,

    #[serde(rename = ":auto")]
    Auto,

    #[serde(rename = ":sspi")]
    Sspi,

    #[serde(rename = ":windows")]
    Windows,

    #[serde(rename = ":username")]
    Username,

    #[serde(other)]
    Unknown
}
