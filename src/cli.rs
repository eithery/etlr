use std::path::Path;
use colored::{ColoredString, Colorize};
use crate::env;


const APP_TITLE: &str = concat!("ETL Toolkit, version ", env!("CARGO_PKG_VERSION"));


pub(crate) fn display_app_header(template_name: &str, command: &str) {
    blank_line();
    println!("{} ({})", cyan(APP_TITLE), cyan(command));
    println!("Current environment: '{}'", cyan(&env::current_environment().to_string()));
    println!("Template name: '{}'", cyan(template_name));
    blank_line();
    println!("Load app configuration:")
}


pub(crate) fn file_loaded(file_path: &Path) {
    println!("{:?} \t{}", file_path, green("LOADED"));
}


pub(crate) fn blank_line() {
    echo("");
}


fn echo(message: &str) {
    println!("{message}")
}


fn cyan(message: &str) -> ColoredString {
    message.bright_cyan()
}


fn green(message: &str) -> ColoredString {
    message.green()
}
