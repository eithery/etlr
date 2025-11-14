use anyhow::Result;
use dotenv::dotenv;
use crate::cli;
use crate::config::app::AppConfiguration;


pub(crate) trait DataPipeline {
    type Options<'a>;

    const PIPELINE_NAME: &'static str;

    fn new(config: AppConfiguration) -> Self;

    fn run<'a>(&self, options: Self::Options<'a>) -> Result<()>;


    fn build(template_name: &str, config_path: Option<&str>) -> Result<Self>
        where Self: Sized
    {
        dotenv().ok();
        cli::display_app_header(template_name, Self::PIPELINE_NAME);
        let config = AppConfiguration::load(config_path);
        cli::blank_line();
        Ok(Self::new(config))
    }
}
