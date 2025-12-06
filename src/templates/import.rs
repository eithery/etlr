use serde::Deserialize;
use super::traits::FileTemplate;


#[derive(Debug, Deserialize)]
pub(crate) struct FileImportTemplate {
}


impl FileTemplate for FileImportTemplate {
}
