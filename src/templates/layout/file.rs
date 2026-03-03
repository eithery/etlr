use serde::Deserialize;
use super::dataset::DatasetTemplate;


#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub(super) struct OutboundFileTemplate {
    file_name: String,
    dataset: DatasetTemplate
}
