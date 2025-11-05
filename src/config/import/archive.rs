use serde::Deserialize;
use crate::std::string;


const DEFAULT_PATH_VALUE: &str = ":default";


#[derive(Debug, Deserialize)]
pub(super) struct ArchiveConfiguration {
    enabled: Option<bool>,
    compressed: Option<bool>,

    #[serde(default, deserialize_with = "string::deserialize")]
    path: Option<String>
}


impl Default for ArchiveConfiguration {
    fn default() -> Self {
        Self {
            enabled: Some(true),
            compressed: Some(true),
            path: Some(DEFAULT_PATH_VALUE.to_string())
        }
    }
}


impl ArchiveConfiguration {
    pub(super) fn merge(self, other: Self) -> Self {
        Self {
            enabled: other.enabled.or(self.enabled),
            compressed: other.compressed.or(self.compressed),
            path: other.path.or(self.path)
        }
    }
}
