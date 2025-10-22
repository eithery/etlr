pub fn load_files(
    template_name: &str,
    config_path: Option<&str>,
    postprocess: bool,
    all_stages: bool,
    force_update: bool,
    preserve_inbox: bool
) {
    println!("\nLoad files for the template '{template_name}'.");
    println!("{config_path:?}");
    println!("{postprocess}");
    println!("{all_stages}");
    println!("{force_update}");
    println!("{preserve_inbox}");
}


pub fn export_files(template_name: &str) {
    println!("\nExport files for the template: '{template_name}'.\n");
}


pub fn anonymize_files(template_name: &str) {
    println!("\nAnonymize files for the template: '{template_name}'.\n");
}


pub fn create_database() {
    println!("\nCreate internal ETL database.\n")
}


pub fn drop_database() {
    println!("\nDrop internal ETL database.\n")
}


pub fn create_tables(template_name: &str) {
    println!("\nCreate DB tables for the template: '{template_name}'.\n");
}


pub fn drop_tables(template_name: &str) {
    println!("\nDrop DB tables for the template: '{template_name}'.\n")
}
