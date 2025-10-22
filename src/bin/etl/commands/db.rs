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
        template: String
    },
    #[command(about = "Drop database tables for the given template")]
    DropTables {
        #[arg(short, long, value_name = "TEMPLATE_NAME", help = "Template used to drop database tables")]
        template: String
    }
}
