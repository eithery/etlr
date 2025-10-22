use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};
use constcat::concat;
use etlr::{load_files, export_files, anonymize_files, create_database, drop_database, create_tables, drop_tables};


const HELP_PREFIX: &str = "\n{before-help}\n{about-with-newline}\n";
const CUSTOM_USAGE: &str = "USAGE: etl <COMMAND> [OPTIONS...]\n";
const DEFAULT_USAGE: &str = "USAGE: {usage}\n";
const COMMANDS: &str = "\nCOMMANDS:\n{subcommands}\n";
const DEFAULT_OPTIONS: &str = "\nOPTIONS:\n{options}";
const CUSTOM_OPTIONS: &str = "
OPTIONS:
  -h, --help     Print help
  -V, --version  Print version
";

const DEFAULT_HELP_TEMPLATE: &str = concat!(HELP_PREFIX, CUSTOM_USAGE, COMMANDS, CUSTOM_OPTIONS);
const COMMAND_GROUP_HELP_TEMPLATE: &str = concat!(HELP_PREFIX, DEFAULT_USAGE, COMMANDS, DEFAULT_OPTIONS);
const COMMAND_HELP_TEMPLATE: &str = concat!(HELP_PREFIX, DEFAULT_USAGE, DEFAULT_OPTIONS);
const APP_TITLE: &str = concat!("\x1b[96mETL Toolkit, version ", env!("CARGO_PKG_VERSION"), "\x1b[0m");


#[derive(Parser)]
#[command(
    name = "etl",
    version,
    about = APP_TITLE,
    long_about = None,
    disable_help_subcommand = true,
    disable_version_flag = true,
    help_template = DEFAULT_HELP_TEMPLATE
)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(short = 'V', long, help = "Print version", hide = true)]
    version: bool
}


#[derive(Subcommand)]
#[command(help_template = DEFAULT_HELP_TEMPLATE)]
pub enum Commands {
    #[command(
        about = "Load files belong to the given template",
        help_template = COMMAND_HELP_TEMPLATE
    )]
    Load {
        #[arg(short, long, value_name = "TEMPLATE_NAME", help = "Template used to the file(s) being loaded")]
        template: String,
        #[arg(short, long, value_name = "CONFIG_PATH", help = "Path to the custom app configuration file")]
        config: Option<String>,
        #[arg(short, long, help = "Run ETL postprocessing step")]
        postprocess: bool,
        #[arg(short, long, help = "Run ALL ETL stages with postprocessing")]
        all_stages: bool,
        #[arg(short, long, help = "Force update data in target tables")]
        force_update: bool,
        #[arg(long, help = "Preserve data files in the inbox after load")]
        preserve_inbox: bool
    },
    #[command(
        about = "Export files from a data source",
        help_template = COMMAND_HELP_TEMPLATE
    )]
    Export {
        #[arg(short, long, value_name = "TEMPLATE_NAME", help = "Template used to the file(s) being exported")]
        template: String
    },
    #[command(
        about = "Anonymize PII data in files",
        help_template = COMMAND_HELP_TEMPLATE
    )]
    Anonify {
        #[arg(short, long, value_name = "TEMPLATE_NAME", help = "Template used to the file(s) being anonymized")]
        template: String
    },
    #[command(about = "The set of database related subcommands")]
    Db {
        #[command(subcommand)]
        command: DbCommands
    }
}


#[derive(Subcommand)]
#[command(help_template = COMMAND_GROUP_HELP_TEMPLATE)]
pub enum DbCommands {
    #[command(about = "Create ETL database")]
    Create,
    #[command(about = "Drop ETL database")]
    Drop,
    #[command(about = "Create database tables for the given template")]
    CreateTables {
        #[arg(short, long, value_name = "TEMPLATE_NAME", help = "Template used to create database tables")]
        template: String
    },
    #[command(about = "Drop database tables for the given template")]
    DropTables {
        #[arg(short, long, value_name = "TEMPLATE_NAME", help = "Template used to drop database tables")]
        template: String
    }
}


fn main() -> Result<()> {
    let cli = Cli::parse();
    if cli.version {
        println!("\n{APP_TITLE}\n");
        return Ok(());
    }

    match &cli.command {
        Some(Commands::Load {
            template,
            config,
            postprocess,
            all_stages,
            force_update,
            preserve_inbox
        }) =>
            load_files(template, config.as_deref(), *postprocess, *all_stages, *force_update, *preserve_inbox),

        Some(Commands::Export { template }) => export_files(template),
        Some(Commands::Anonify { template }) => anonymize_files(&template),
        Some(Commands::Db { command }) =>
            match command {
                DbCommands::CreateTables { template } => create_tables(template),
                DbCommands::DropTables { template } => drop_tables(template),
                DbCommands::Create => create_database(),
                DbCommands::Drop => drop_database()
            },
        None => Cli::command().print_help()?
    }
    Ok(())
}
