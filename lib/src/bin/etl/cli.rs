use clap::Parser;
use crate::commands::Commands;
use crate::help;


pub(crate) const APP_TITLE: &str = concat!("\x1b[96mETL Toolkit, version ", env!("CARGO_PKG_VERSION"), "\x1b[0m");


#[derive(Parser)]
#[command(
    name = "etl",
    version,
    about = APP_TITLE,
    long_about = None,
    disable_help_subcommand = true,
    disable_version_flag = true,
    help_template = help::DEFAULT_TEMPLATE
)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Option<Commands>,

    #[arg(short = 'V', long, help = "Print version", hide = true)]
    pub(crate) version: bool
}
