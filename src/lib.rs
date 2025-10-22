pub fn load_files(
    template_name: &str,
    config_path: Option<&str>,
    postprocess: bool,
    all_stages: bool,
    force_update: bool,
    update_only: bool,
    preserve_inbox: bool,
    skip_audit: bool
) {
    println!("\nLoad files for the template '{template_name}'.");
    println!("{config_path:?}");
    println!("{postprocess}");
    println!("{all_stages}");
    println!("{force_update}");
    println!("{update_only}");
    println!("{preserve_inbox}");
    println!("{skip_audit}");
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
