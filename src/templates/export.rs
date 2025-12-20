use serde::Deserialize;
use super::traits::FileTemplate;


#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub(crate) struct FileExportTemplate {
    etl_template: String,
    version: u8,
    // file: String,
    description: String
    // layout: String
}


impl FileTemplate for FileExportTemplate {
    const TEMPLATES_ROOT: &'static str = "export";
}
