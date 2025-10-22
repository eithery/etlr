use clap::Subcommand;
use crate::help;


#[derive(Subcommand)]
#[command(help_template = help::COMMAND_GROUP_TEMPLATE)]
pub(crate) enum Commands {
    #[command(about = "Create ETL database")]
    Create,
    #[command(about = "Drop ETL database")]
    Drop,
    #[command(about = "Create database tables for the given template")]
    CreateTables {
        #[arg(short, long, value_name = "TEMPLATE_NAME", help = "Template used to create database tables")]
        template: String,
        #[arg(short, long, value_name = "CONFIG_PATH", help = "Path to the custom app configuration file")]
        config: Option<String>,
        #[arg(short, long, help = "Drop and recreate data tables, if exists")]
        force: bool
    },
    #[command(about = "Drop database tables for the given template")]
    DropTables {
        #[arg(short, long, value_name = "TEMPLATE_NAME", help = "Template used to drop database tables")]
        template: String
    }
}
