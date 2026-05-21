use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub(crate) struct WorkflowTemplate {
    hooks: WorkflowHooks
}


impl WorkflowTemplate {
    pub(crate) fn hooks(&self) -> &WorkflowHooks {
        &self.hooks
    }
}


#[derive(Debug, Deserialize)]
pub(crate) struct WorkflowHooks {
    #[serde(default)]
    before: Vec<WorkflowHookTemplate>,

    #[serde(default)]
    after: Vec<WorkflowHookTemplate>
}


impl WorkflowHooks {
    pub(crate) fn before(&self) -> impl Iterator<Item = &WorkflowHookTemplate> {
        self.before.iter()
    }


    pub(crate) fn after(&self) -> impl Iterator<Item = &WorkflowHookTemplate> {
        self.after.iter()
    }
}


#[derive(Debug, Deserialize)]
pub(crate) struct WorkflowHookTemplate {
    #[serde(rename = "type")]
    hook_type: String,

    data_source: String,

    run: String
}


impl WorkflowHookTemplate {
    #[allow(dead_code)]
    fn hook_type(&self) -> &str {
        &self.hook_type
    }


    #[allow(dead_code)]
    fn data_source(&self) -> &str {
        &self.data_source
    }


    #[allow(dead_code)]
    fn run(&self) -> &str {
        &self.run
    }
}
