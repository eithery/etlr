use anyhow::Result;
use crate::config::app::AppConfiguration;
use crate::templates::import::FileImportTemplate;
use super::options::FileImportOptions;
use super::traits::DataPipeline;


pub(crate) struct FileImportPipeline {
    template: FileImportTemplate,
    config: AppConfiguration
}


impl DataPipeline for FileImportPipeline {
    type Options<'a> = FileImportOptions;
    type Template = FileImportTemplate;

    const PIPELINE_NAME: &'static str = "Import pipeline";


    fn new(template: FileImportTemplate, config: AppConfiguration) -> Self {
        Self {
            template,
            config
        }
    }


    fn run(&self, options: FileImportOptions) -> Result<()> {
        Ok(())
    }
}
