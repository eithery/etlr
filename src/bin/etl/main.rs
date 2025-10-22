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
            preserve_inbox,
            update_only,
            skip_audit
        }) =>
            load_files(
                template,
                config.as_deref(),
                *postprocess,
                *all_stages,
                *force_update,
                *update_only,
                *preserve_inbox,
                *skip_audit
            ),

        Some(Commands::Export {
            template,
            config,
            files,
            file_prefix,
            skip_column_names
        }) =>
            export_files(
                template,
                config.as_deref(),
                files.as_deref(),
                file_prefix.as_deref(),
                *skip_column_names
            ),

        Some(Commands::Anonify { template, config }) => anonymize_files(template, config.as_deref()),

        Some(Commands::Db { command }) =>
            match command {
                db::Commands::Migrate {
                    template,
                    config,
                    force
                } => create_tables(template, config.as_deref(), *force),

                db::Commands::Rollback { template, config } => drop_tables(template, config.as_deref()),
                db::Commands::Create { config} => create_database(config.as_deref()),
                db::Commands::Drop { config } => drop_database(config.as_deref())
            },
        None => Cli::command().print_help()?
    }
    Ok(())
}
