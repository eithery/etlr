use serde::Deserialize;
use super::columns::ColumnSelectionTemplate;


#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub(super) struct DatasetTemplate {
    id: String,
    source: String,
    columns: ColumnSelectionTemplate
}
