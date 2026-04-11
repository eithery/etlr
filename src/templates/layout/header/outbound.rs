use std::ops::Deref;
use chrono::NaiveDateTime;
use serde::Deserialize;
use crate::std::datetime::DateTime;
use crate::std::result::Result;
use crate::templates::defaults::{default_true, default_false, default_date_format};
use super::FileHeaderTemplate;
use super::base::FileHeaderTemplateBase;


const FILE_VERSION: &str = "v1.0";
const DEFAULT_TAG: &str = "H";


#[derive(Debug, Deserialize)]
pub(crate) struct OutboundFileHeaderTemplate {
    #[serde(flatten)]
    base: FileHeaderTemplateBase,

    #[serde(rename = "date", default = "default_date_format")]
    date_format: String,

    #[serde(default = "default_true")]
    include_file_type: bool,

    #[serde(default = "default_false")]
    include_file_name: bool,

    #[serde(default = "default_true")]
    include_file_version: bool
}


impl Deref for OutboundFileHeaderTemplate {
    type Target = FileHeaderTemplateBase;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}


impl OutboundFileHeaderTemplate {
    pub(super) fn date_format(&self) -> &str {
        self.date_format.as_str()
    }


    #[allow(dead_code)]
    pub(super) fn include_file_type(&self) -> bool {
        self.include_file_type
    }


    #[allow(dead_code)]
    pub(super) fn include_file_name(&self) -> bool {
        self.include_file_name
    }


    #[allow(dead_code)]
    pub(super) fn include_file_version(&self) -> bool {
        self.include_file_version
    }


    #[allow(dead_code)]
    pub(crate) fn build(&self, file_type: &str, file_name: &str) -> Result<impl Iterator<Item = String>>
    {
        Ok([
            Some(self.tag().unwrap_or(DEFAULT_TAG).to_string()),
            self.include_file_type.then(|| file_type.to_string()),
            self.include_file_name.then(|| file_name.to_string()),
            Some(NaiveDateTime::format_timestamp_with(self.date_format(), false)?),
            self.include_file_version.then(|| FILE_VERSION.to_string())
        ]
        .into_iter()
        .flatten())
    }
}
