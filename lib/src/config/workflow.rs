use serde::Deserialize;
use super::export::ExportConfiguration;
use super::import::ImportConfiguration;


#[derive(Debug, Deserialize, Default)]
pub(super) struct WorkflowConfiguration {
    #[serde(default)]
    import: ImportConfiguration,

    #[serde(default)]
    export: ExportConfiguration
}


impl WorkflowConfiguration {
    pub(super) fn merge(self, other: Self) -> Self {
        Self {
            import: self.import.merge(other.import),
            export: self.export.merge(other.export)
        }
    }
}
