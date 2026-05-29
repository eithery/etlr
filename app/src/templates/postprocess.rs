use std::fmt::Display;
use serde::Deserialize;
use crate::templates::defaults::default_true;


#[derive(Debug, Deserialize)]
pub(crate) struct PostprocessTemplate {
    #[serde(rename = "stage_2")]
    stage2: PostprocessStageTemplate,

    #[serde(rename = "stage_3")]
    stage3: PostprocessStageTemplate
}


impl PostprocessTemplate {
    #[allow(dead_code)]
    fn stage2(&self) -> &PostprocessStageTemplate {
        &self.stage2
    }


    #[allow(dead_code)]
    fn stage3(&self) -> &PostprocessStageTemplate {
        &self.stage3
    }
}


#[derive(Debug, Deserialize)]
struct PostprocessStageTemplate {
    #[serde(default = "default_true")]
    enabled: bool,

    #[serde(rename = "type")]
    plugin_type: Option<PostprocessPluginType>,

    data_source: Option<String>,

    #[serde(rename = "plugin")]
    plugin_name: Option<String>,

    run: Option<String>
}


impl PostprocessStageTemplate {
    #[allow(dead_code)]
    fn enabled(&self) -> bool {
        self.enabled
    }


    #[allow(dead_code)]
    fn plugin_type(&self) -> Option<PostprocessPluginType> {
        self.plugin_type
    }


    #[allow(dead_code)]
    fn data_source(&self) -> &str {
        self.data_source
            .as_deref()
            .unwrap_or(":default")
    }


    #[allow(dead_code)]
    fn plugin_name(&self) -> Option<&str> {
        self.plugin_name.as_deref()
    }


    #[allow(dead_code)]
    fn run(&self) -> Option<&str> {
        self.run.as_deref()
    }
}


#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
enum PostprocessPluginType {
    #[serde(rename = ":sql")]
    Sql,

    #[serde(rename = ":python")]
    Python
}


impl Display for PostprocessPluginType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Sql => ":sql",
            Self::Python => ":python"
        };
        write!(f, "{s}")
    }
}
