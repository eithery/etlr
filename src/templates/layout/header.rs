use serde::Deserialize;
use crate::templates::defaults::{default_true, default_date_format};
use crate::std::date;
use crate::std::result::Result;


const FILE_VERSION: &str = "v1.0";


#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub(super) struct FileHeaderTemplate {
    tag: String,

    #[serde(rename = "date", default = "default_date_format")]
    date_format: String,

    #[serde(default = "default_true")]
    include_file_type: bool,

    include_file_name: bool,

    #[serde(default = "default_true")]
    include_file_version: bool
}


#[allow(dead_code)]
impl FileHeaderTemplate {
    pub(super) fn tag(&self) -> &str {
        self.tag.as_str()
    }


    pub(super) fn date_format(&self) -> &str {
        self.date_format.as_str()
    }


    pub(super) fn include_file_type(&self) -> bool {
        self.include_file_type
    }


    pub(super) fn include_file_name(&self) -> bool {
        self.include_file_name
    }


    pub(super) fn include_file_version(&self) -> bool {
        self.include_file_version
    }


    pub(super) fn build(&self, file_type: &str, file_name: &str) -> Result<impl Iterator<Item = String>>
    {
        Ok([
            Some(self.tag.clone()),
            self.include_file_type.then(|| file_type.to_string()),
            self.include_file_name.then(|| file_name.to_string()),
            Some(date::timestamp(self.date_format(), false)?),
            self.include_file_version.then(|| FILE_VERSION.to_string())
        ]
        .into_iter()
        .flatten())
    }
}
