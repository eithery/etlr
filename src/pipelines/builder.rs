use dotenv::dotenv;
use crate::cli;
use crate::config::app::AppConfiguration;


pub(crate) fn build_export_pipeline(template_name: &str, config_path: Option<&str>) {    
    bootstrap_pipeline(template_name, "Export pipeline", config_path);
}


fn bootstrap_pipeline(template_name: &str, command: &str, config_path: Option<&str>) {
    dotenv().ok();
    cli::display_app_header(template_name, command);
    let _ = AppConfiguration::load(config_path);
    cli::blank_line();
}
