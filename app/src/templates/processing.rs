use serde::Deserialize;
use crate::templates::defaults::{default_false, default_true};


#[derive(Debug, Deserialize)]
pub(crate) struct ProcessingTemplate {
    #[serde(default = "default_false")]
    staging: bool,

    update_strategy: Option<UpdateStrategy>,

    #[serde(default = "default_true")]
    check_file_loaded: bool
}


impl ProcessingTemplate {
    #[allow(dead_code)]
    fn staging(&self) -> bool {
        self.staging
    }


    #[allow(dead_code)]
    fn update_strategy(&self) -> UpdateStrategy {
        self.update_strategy.unwrap_or_default()
    }


    #[allow(dead_code)]
    fn check_file_loaded(&self) -> bool {
        self.check_file_loaded
    }
}


#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
enum UpdateStrategy {
    #[serde(rename = ":default", alias = ":append")]
    Append,

    #[serde(rename = ":delete_insert")]
    DeleteInsert
}


impl Default for UpdateStrategy {
    fn default() -> Self {
        UpdateStrategy::Append
    }
}
