mod cli;
mod commands;
mod help;

use anyhow::Result;
use clap::{CommandFactory, Parser};
use etlr::{load_files, export_files, anonymize_files, create_database, drop_database, create_tables, drop_tables};
use crate::cli::{Cli, APP_TITLE};
use crate::commands::{Commands, db};


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
                db::Commands::CreateTables { template } => create_tables(template),
                db::Commands::DropTables { template } => drop_tables(template),
                db::Commands::Create => create_database(),
                db::Commands::Drop => drop_database()
            },
        None => Cli::command().print_help()?
    }
    Ok(())
}
