use serde::Deserialize;
use crate::std::string;


const DEFAULT_PATH_VALUE: &str = ":default";


#[derive(Debug, Deserialize)]
pub(super) struct FailedImportsConfiguration {
    enabled: Option<bool>,

    #[serde(default, deserialize_with = "string::deserialize")]
    path: Option<String>
}


impl Default for FailedImportsConfiguration {
    fn default() -> Self {
        Self {
            enabled: Some(true),
            path: Some(DEFAULT_PATH_VALUE.to_string())
        }
    }
}


impl FailedImportsConfiguration {
    pub(super) fn merge(self, other: Self) -> Self {
        Self {
            enabled: other.enabled.or(self.enabled),
            path: other.path.or(self.path)
        }
    }
}
