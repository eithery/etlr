use serde::Deserialize;


#[derive(Debug, Deserialize, Default)]
pub(super) struct ExportConfiguration {
    #[serde(default)]
    file_name: FileNameConfiguration
}


impl ExportConfiguration {
    pub(super) fn merge(self, other: Self) -> Self {
        Self {
            file_name: self.file_name.merge(other.file_name)
        }
    }
}


#[derive(Debug, Deserialize)]
pub(super) struct FileNameConfiguration {
    include_time: Option<bool>
}


impl Default for FileNameConfiguration {
    fn default() -> Self {
        Self { include_time: Some(true) }
    }
}


impl FileNameConfiguration {
    fn merge(self, other: Self) -> Self {
        Self {
            include_time: other.include_time.or(self.include_time)
        }
    }
}
