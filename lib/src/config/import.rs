mod archive;
mod failed;

use serde::Deserialize;
use archive::ArchiveConfiguration;
use failed::FailedImportsConfiguration;


#[derive(Debug, Deserialize, Default)]
pub(super) struct ImportConfiguration {
    #[serde(default)]
    archive: ArchiveConfiguration,

    #[serde(default)]
    failed_imports: FailedImportsConfiguration,

    preserve_data_file: Option<bool>
}


impl ImportConfiguration {
    pub(super) fn merge(self, other: Self) -> Self {
        Self {
            archive: self.archive.merge(other.archive),
            failed_imports: self.failed_imports.merge(other.failed_imports),
            preserve_data_file: other.preserve_data_file.or(self.preserve_data_file)
        }
    }
}
