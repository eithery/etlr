use std::path::Path;
use colored::{ColoredString, Colorize};
use crate::env;


const APP_TITLE: &str = concat!("ETL Toolkit, version ", env!("CARGO_PKG_VERSION"));


pub fn error(message: &str) {
    display_message(message, Some("Error:"), true, true);
}


pub(crate) fn display_app_header(template_name: &str, command: &str) {
    blank_line();
    println!("{} ({})", cyan(APP_TITLE), cyan(command));
    println!("Current environment: '{}'", cyan(&env::current_environment().to_string()));
    println!("Template name: '{}'", cyan(template_name));
    blank_line();
    println!("Load app configuration:")
}


pub(crate) fn file_loaded(file_path: &Path) {
    println!("{}  {}", file_path.display(), green("LOADED"));
}


pub(crate) fn env_vars_applied() {
    println!("Apply environment variables  {}", green("DONE"));
}


pub(crate) fn blank_line() {
    echo("");
}


pub(crate) fn echo(message: &str) {
    println!("{message}")
}


fn display_message(message: &str, prefix: Option<&str>, prepend_line: bool, append_line: bool) {
    let combined_message = match prefix {
        Some(p) => format!("{p} {message}"),
        None => message.to_string()
    };
    if prepend_line {
        blank_line();
    }
    eprintln!("{}", red(&combined_message));
    if append_line {
        blank_line();
    }
}


fn cyan(message: &str) -> ColoredString {
    message.bright_cyan()
}


fn green(message: &str) -> ColoredString {
    message.green()
}


fn red(message: &str) -> ColoredString {
    message.red()
}
