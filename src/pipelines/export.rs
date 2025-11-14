use anyhow::Result;
use crate::config::app::AppConfiguration;
use super::options::FileExportOptions;
use super::traits::DataPipeline;


pub(crate) struct FileExportPipeline {
    config: AppConfiguration
}


impl DataPipeline for FileExportPipeline {
    type Options<'a> = FileExportOptions<'a>;

    const PIPELINE_NAME: &'static str = "Export pipeline";


    fn new(config: AppConfiguration) -> Self {
        Self {
            config
        }
    }


    fn run<'a>(&self, options: FileExportOptions<'a>) -> Result<()> {
        Ok(())
    }
}
