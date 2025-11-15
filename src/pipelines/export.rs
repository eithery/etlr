use anyhow::Result;
use crate::config::app::AppConfiguration;
use crate::templates::export::FileExportTemplate;
use super::options::FileExportOptions;
use super::traits::DataPipeline;


pub(crate) struct FileExportPipeline {
    template: FileExportTemplate,
    config: AppConfiguration
}


impl DataPipeline for FileExportPipeline {
    type Options<'a> = FileExportOptions<'a>;
    type Template = FileExportTemplate;

    const PIPELINE_NAME: &'static str = "Export pipeline";


    fn new(template: FileExportTemplate, config: AppConfiguration) -> Self {
        Self {
            template,
            config
        }
    }


    fn run<'a>(&self, options: FileExportOptions<'a>) -> Result<()> {
        Ok(())
    }
}
