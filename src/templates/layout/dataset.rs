use serde::Deserialize;
use super::columns::ColumnSelectionTemplate;
use super::join::DatasetJoinTemplate;


#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub(super) struct DatasetTemplate {
    id: String,
    source: String,
    columns: ColumnSelectionTemplate,

    #[serde(default)]
    join: Vec<DatasetJoinTemplate>
}
