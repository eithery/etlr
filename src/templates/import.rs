use serde::Deserialize;
use super::traits::FileTemplate;


#[derive(Debug, Deserialize)]
pub(crate) struct FileImportTemplate {
}


impl FileTemplate for FileImportTemplate {
    const TEMPLATES_ROOT: &'static str = "import";
}
