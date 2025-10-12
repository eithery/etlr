use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};
use etlr::{load_files, export_files, anonymize_files, create_database, drop_database, create_tables, drop_tables};


const DEFAULT_HELP_TEMPLATE: &str = "\n\
{before-help}
{about-with-newline}
USAGE: etl <COMMAND> [OPTIONS]...

COMMANDS:
{subcommands}

OPTIONS:
{options}
\u{200B}";

const COMMAND_HELP_TEMPLATE: &str = "\n\
{before-help}
{about-with-newline}
USAGE: {usage}

OPTIONS:
{options}
\u{200B}";

const APP_TITLE: &str = concat!("\x1b[36mETL Toolkit, version ", env!("CARGO_PKG_VERSION"), "\x1b[0m");

#[derive(Parser)]
#[command(
    name = "etl",
    version,
    propagate_version = true,
    about = APP_TITLE,
    long_about = None,
    disable_help_subcommand = true,
    disable_version_flag = true,
    help_template = DEFAULT_HELP_TEMPLATE
)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(short = 'V', long, global = true, help = "Print version")]
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
        #[arg(short, long, value_name = "TEMPLATE_NAME")]
        template: String
    },
    #[command(
        about = "Export files from a data source",
        help_template = COMMAND_HELP_TEMPLATE
    )]
    Export {
        #[arg(short, long, value_name = "TEMPLATE_NAME")]
        template: String
    },
    #[command(
        about = "Anonymize PII data in files",
        help_template = COMMAND_HELP_TEMPLATE
    )]
    Anonify {
        #[arg(short, long, value_name = "TEMPLATE_NAME")]
        template: String
    },
    #[command(
        about = "The set of database related subcommands",
        help_template = DEFAULT_HELP_TEMPLATE
    )]
    Db {
        #[command(subcommand)]
        command: DbCommands
    }
}


#[derive(Subcommand)]
#[command(help_template = DEFAULT_HELP_TEMPLATE)]
pub enum DbCommands {
    #[command(about = "Create ETL database.")]
    Create,
    #[command(about = "Drop ETL database.")]
    Drop,
    #[command(about = "Create database tables for the given template.")]
    CreateTables {
        #[arg(short, long, value_name = "TEMPLATE_NAME")]
        template: String
    },
    #[command(about = "Drop database tables for the given template.")]
    DropTables {
        #[arg(short, long, value_name = "TEMPLATE_NAME")]
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
        Some(Commands::Load { template }) => load_files(template),
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
