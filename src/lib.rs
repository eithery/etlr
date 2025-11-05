mod config;
mod env;
mod path;
mod std;

use crate::config::app::AppConfiguration;


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
    let config = AppConfiguration::load(config_path);
    println!("{config:#?}");
}


pub fn export_files(
    template_name: &str,
    config_path: Option<&str>,
    files: Option<&str>,
    file_prefix: Option<&str>,
    skip_column_names: bool
) {
    println!("\nExport files for the template: '{template_name}'.");
    println!("{config_path:?}");
    println!("{files:?}");
    println!("{file_prefix:?}");
    println!("{skip_column_names}");
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
