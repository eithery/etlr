use std::str::FromStr;
use serde::Deserialize;
use crate::errors::EtlError;
use crate::std::result::Result;
use crate::std::string::Normalize;
use super::errors as err;


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
    Username
}


impl Default for ConnectionType {
    fn default() -> Self {
        Self::Default
    }
}


impl FromStr for ConnectionType {
    type Err = EtlError;

    fn from_str(str_value: &str) -> Result<Self> {
        match str_value.normalize().as_str() {
            ":default" => Ok(ConnectionType::Default),
            ":auto" => Ok(ConnectionType::Auto),
            ":sspi" => Ok(ConnectionType::Sspi),
            ":windows" => Ok(ConnectionType::Windows),
            ":username" => Ok(ConnectionType::Username),
            value => Err(err::invalid_connection_type(value))
        }
    }
}


impl ConnectionType {
    pub(crate) fn is_trusted(&self) -> bool {
        matches!(self, ConnectionType::Auto | ConnectionType::Sspi | ConnectionType::Windows)
    }
}
