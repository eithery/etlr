mod header;
mod trailer;

use serde::Deserialize;
use self::header::FileHeaderTemplate;
use self::trailer::FileTrailerTemplate;
use super::defaults::default_true;


#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub(crate) struct RecordLayoutTemplate {
    header: FileHeaderTemplate,
    trailer: FileTrailerTemplate,

    #[serde(default = "default_true")]
    include_column_names: bool
}
