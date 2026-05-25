use serde::Deserialize;
use crate::templates::defaults::{default_false, default_true};


#[derive(Debug, Deserialize)]
pub(crate) struct PostprocessTemplate {
    #[serde(rename = "stage_2")]
    stage2: PostprocessStage2Template,

    #[serde(rename = "stage_3")]
    stage3: PostprocessStage3Template
}


impl PostprocessTemplate {
    #[allow(dead_code)]
    fn stage2(&self) -> &PostprocessStage2Template {
        &self.stage2
    }


    #[allow(dead_code)]
    fn stage3(&self) -> &PostprocessStage3Template {
        &self.stage3
    }
}


#[derive(Debug, Deserialize)]
struct PostprocessStage2Template {
    #[serde(default = "default_true")]
    enabled: bool
}


impl PostprocessStage2Template {
    #[allow(dead_code)]
    fn enabled(&self) -> bool {
        self.enabled
    }
}


#[derive(Debug, Deserialize)]
struct PostprocessStage3Template {
    #[serde(default = "default_true")]
    enabled: bool,

    #[serde(default = "default_false")]
    default: bool,

    #[serde(rename = "type")]
    plugin_type: PostprocessPluginType,

    data_source: String,

    #[serde(rename = "plugin")]
    plugin_name: String
}


impl PostprocessStage3Template {
    #[allow(dead_code)]
    fn enabled(&self) -> bool {
        self.enabled
    }


    #[allow(dead_code)]
    fn default(&self) -> bool {
        self.default
    }


    #[allow(dead_code)]
    fn plugin_type(&self) -> PostprocessPluginType {
        self.plugin_type
    }


    #[allow(dead_code)]
    fn data_source(&self) -> &str {
        &self.data_source
    }


    #[allow(dead_code)]
    fn plugin_name(&self) -> &str {
        &self.plugin_name
    }
}


#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
enum PostprocessPluginType {
    #[serde(rename = ":sql")]
    Sql,

    #[serde(rename = ":python")]
    Python
}
