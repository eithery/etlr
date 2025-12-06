use serde::Deserialize;
use super::traits::FileTemplate;


#[derive(Debug, Deserialize)]
pub(crate) struct FileExportTemplate {
}


impl FileTemplate for FileExportTemplate {
}
