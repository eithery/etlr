pub mod cli;
mod config;
mod env;
mod errors;
mod fs;
mod path;
mod pipelines;
pub mod std;
mod templates;

use crate::pipelines::{import::FileImportPipeline, export::FileExportPipeline};
use crate::pipelines::options::{FileExportOptions, FileImportOptions};
use crate::pipelines::traits::DataPipeline;
use crate::std::result::Result;


pub fn import_files(
    template_name: &str,
    config_path: Option<&str>,
    postprocess: bool,
    all_stages: bool,
    force_update: bool,
    update_only: bool,
    preserve_inbox: bool,
    skip_audit: bool
) -> Result<()> {
    FileImportPipeline::build(template_name, config_path)?
        .run(FileImportOptions {
            postprocess,
            all_stages,
            force_update,
            update_only,
            preserve_inbox,
            skip_audit
        })
}


pub fn export_files(
    template_name: &str,
    config_path: Option<&str>,
    files: Option<&str>,
    file_prefix: Option<&str>,
    skip_column_names: bool
) -> Result<()> {
    FileExportPipeline::build(template_name, config_path)?
        .run(FileExportOptions {
            files,
            file_prefix,
            skip_column_names
        })
}


pub fn anonymize_files(template_name: &str, config_path: Option<&str>) -> Result<()> {
    println!("\nAnonymize files for the template: `{template_name}`.");
    println!("{config_path:?}");
    Ok(())
}


pub fn create_database(config_path: Option<&str>) -> Result<()> {
    println!("\nCreate internal ETL database.");
    println!("{config_path:?}");
    Ok(())
}


pub fn drop_database(config_path: Option<&str>) -> Result<()> {
    println!("\nDrop internal ETL database.");
    println!("{config_path:?}");
    Ok(())
}


pub fn create_tables(template_name: &str, config_path: Option<&str>, force: bool) -> Result<()> {
    println!("\nCreate DB tables for the template: `{template_name}`.");
    println!("{config_path:?}");
    println!("{force}");
    Ok(())
}


pub fn drop_tables(template_name: &str, config_path: Option<&str>) -> Result<()> {
    println!("\nDrop DB tables for the template: `{template_name}`.");
    println!("{config_path:?}");
    Ok(())
}
