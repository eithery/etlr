use std::fmt::Display;
use serde::Deserialize;
use crate::templates::defaults::{default_false, default_true};


#[derive(Debug, Deserialize)]
pub(crate) struct ProcessingTemplate {
    update_strategy: Option<UpdateStrategy>,

    #[serde(default = "default_false")]
    legacy: bool,

    #[serde(default = "default_true")]
    check_file_loaded: bool,

    #[serde(default = "default_false")]
    multiple_daily_files: bool,

    #[serde(default = "default_false")]
    fail_on_errors: bool
}


impl ProcessingTemplate {
    fn update_strategy(&self) -> UpdateStrategy {
        self.update_strategy.unwrap_or_default()
    }


    #[allow(dead_code)]
    fn legacy(&self) -> bool {
        self.legacy
    }


    fn check_file_loaded(&self) -> bool {
        self.check_file_loaded
    }


    #[allow(dead_code)]
    fn multiple_daily_files(&self) -> bool {
        self.multiple_daily_files
    }


    #[allow(dead_code)]
    fn fail_on_errors(&self) -> bool {
        self.fail_on_errors
    }


    #[allow(dead_code)]
    fn required_per_file(&self) -> bool {
        self.check_file_loaded() && self.update_strategy() == UpdateStrategy::Append
    }
}


#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
enum UpdateStrategy {
    #[serde(rename = ":default", alias = ":append", alias = ":insert")]
    Append,

    #[serde(rename = ":delete_insert")]
    DeleteInsert
}


impl Default for UpdateStrategy {
    fn default() -> Self {
        UpdateStrategy::Append
    }
}


impl Display for UpdateStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Append => ":default",
            Self::DeleteInsert => ":delete_insert"
        };
        write!(f, "{s}")
    }
}
