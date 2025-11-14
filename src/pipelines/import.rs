use anyhow::Result;
use crate::config::app::AppConfiguration;
use super::options::FileImportOptions;
use super::traits::DataPipeline;


pub(crate) struct FileImportPipeline {
    config: AppConfiguration
}


impl DataPipeline for FileImportPipeline {
    type Options<'a> = FileImportOptions;

    const PIPELINE_NAME: &'static str = "Import pipeline";


    fn new(config: AppConfiguration) -> Self {
        Self {
            config
        }
    }


    fn run(&self, options: FileImportOptions) -> Result<()> {
        Ok(())
    }
}
