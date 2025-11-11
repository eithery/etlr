pub mod cli;
mod config;
mod env;
mod path;
mod pipelines;
mod std;

use crate::config::app::AppConfiguration;
use crate::pipelines::builder;


pub fn load_files(
    _template_name: &str,
    config_path: Option<&str>,
    _postprocess: bool,
    _all_stages: bool,
    _force_update: bool,
    _update_only: bool,
    _preserve_inbox: bool,
    _skip_audit: bool
) {
    let _ = AppConfiguration::load(config_path);
    // crate::echo!("Loading configuration file...");
}


pub fn export_files(
    template_name: &str,
    config_path: Option<&str>,
    _files: Option<&str>,
    _file_prefix: Option<&str>,
    _skip_column_names: bool
) {
    builder::build_export_pipeline(template_name, config_path);
}


pub fn anonymize_files(template_name: &str, config_path: Option<&str>) {
    println!("\nAnonymize files for the template: '{template_name}'.");
    println!("{config_path:?}");
}


pub fn create_database(config_path: Option<&str>) {
    println!("\nCreate internal ETL database.");
    println!("{config_path:?}");
}


pub fn drop_database(config_path: Option<&str>) {
    println!("\nDrop internal ETL database.");
    println!("{config_path:?}");
}


pub fn create_tables(template_name: &str, config_path: Option<&str>, force: bool) {
    println!("\nCreate DB tables for the template: '{template_name}'.");
    println!("{config_path:?}");
    println!("{force}")
}


pub fn drop_tables(template_name: &str, config_path: Option<&str>) {
    println!("\nDrop DB tables for the template: '{template_name}'.");
    println!("{config_path:?}");
}
