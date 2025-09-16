use anyhow::Result;
use clap::{Parser, Subcommand};
use etlr::{load_files, export_files, anonymize_files, create_tables};


#[derive(Parser)]
#[command(
    name = "etl",
    version,
    propagate_version = true,
    about = "ETL Toolkit",
    long_about = None
)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands
}


#[derive(Subcommand)]
pub enum Commands {
    Load {
        #[arg(short, long)]
        template: String
    },
    Export {
        #[arg(short, long)]
        template: String
    },
    Anonify {
        #[arg(short, long)]
        template: String
    },
    CreateTables {
        #[arg[short, long]]
        template: String
    }
}


fn main() -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Load { template } => {
            load_files(template);
        }
        Commands::Export { template } => {
            export_files(template);
        }
        Commands::Anonify { template } => {
            anonymize_files(&template);
        }
        Commands::CreateTables { template } => {
            create_tables(template);
        }
    }
    Ok(())
}
