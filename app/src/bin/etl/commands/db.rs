use clap::Subcommand;
use crate::help;


#[derive(Subcommand)]
#[command(help_template = help::COMMAND_GROUP_TEMPLATE)]
pub(crate) enum Commands {
    #[command(about = "Create ETL database", help_template = help::COMMAND_TEMPLATE)]
    Create {
        #[arg(short, long, value_name = "CONFIG_PATH", help = "Path to the custom app configuration file")]
        config: Option<String>
    },

    #[command(about = "Drop ETL database", help_template = help::COMMAND_TEMPLATE)]
    Drop {
        #[arg(short, long, value_name = "CONFIG_PATH", help = "Path to the custom app configuration file")]
        config: Option<String>
    },

    #[command(about = "Create DB objects (migrations) for the given template", help_template = help::COMMAND_TEMPLATE)]
    Migrate {
        #[arg(short, long, value_name = "TEMPLATE_NAME", help = "Template used to create database tables")]
        template: String,
        #[arg(short, long, value_name = "CONFIG_PATH", help = "Path to the custom app configuration file")]
        config: Option<String>,
        #[arg(short, long, help = "Override existing DB objects, if any")]
        force: bool
    },

    #[command(
        about = "Drop DB objects (rollback migrations) for the given template",
        help_template = help::COMMAND_TEMPLATE
    )]
    Rollback {
        #[arg(short, long, value_name = "TEMPLATE_NAME", help = "Template used to drop database tables")]
        template: String,
        #[arg(short, long, value_name = "CONFIG_PATH", help = "Path to the custom app configuration file")]
        config: Option<String>,
    }
}
