use std::ops::Deref;
use serde::Deserialize;
use crate::templates::prelude::*;
use super::{FileTemplate, FileTemplateBase};
use super::format::FileFormat;
use super::layout::outbound::OutboundLayoutTemplate;


#[derive(Debug, Deserialize)]
pub(crate) struct FileExportTemplate {
    #[serde(flatten)]
    base: FileTemplateBase,

    layout: OutboundLayoutTemplate,

    workflow: Option<WorkflowTemplate>
}


impl Deref for FileExportTemplate {
    type Target = FileTemplateBase;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}


impl FileEntry for FileExportTemplate {
    fn file_type(&self) -> &str {
        self.file.file_type()
    }
}


impl FileTemplate for FileExportTemplate {
    const TEMPLATES_ROOT: &'static str = "export";

    type Layout = OutboundLayoutTemplate;


    fn layout(&self) -> &OutboundLayoutTemplate {
        &self.layout
    }
}


impl FileExportTemplate {
    #[allow(dead_code)]
    fn outbound_file_name(&self) -> &str {
        self.file.file_name()
    }


    #[allow(dead_code)]
    fn file_format(&self) -> FileFormat {
        self.file.format()
    }


    #[allow(dead_code)]
    fn workflow(&self) -> Option<&WorkflowTemplate> {
        self.workflow.as_ref()
    }


    #[allow(dead_code)]
    fn before_export_hooks(&self) -> impl Iterator<Item = &WorkflowHookTemplate> {
        self.workflow()
            .into_iter()
            .flat_map(|wf| wf.hooks().before())
    }


    #[allow(dead_code)]
    fn after_export_hooks(&self) -> impl Iterator<Item = &WorkflowHookTemplate> {
        self.workflow()
            .into_iter()
            .flat_map(|wf| wf.hooks().after())
    }
}
