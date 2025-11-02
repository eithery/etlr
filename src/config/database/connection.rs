use std::str::FromStr;
use serde::Deserialize;

use crate::std::string::Normalize;


#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub(crate) enum ConnectionType {
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


impl FromStr for ConnectionType {
    type Err = ();

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        Ok(match string.normalize().as_str() {
            ":default" => ConnectionType::Default,
            ":auto" => ConnectionType::Auto,
            ":sspi" => ConnectionType::Sspi,
            ":windows" => ConnectionType::Windows,
            ":username" => ConnectionType::Username,
            _ => ConnectionType::Unknown,
        })
    }
}
