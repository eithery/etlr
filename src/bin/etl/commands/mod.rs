pub(crate) mod db;

use clap::Subcommand;
use crate::help;


#[derive(Subcommand)]
#[command(help_template = help::DEFAULT_TEMPLATE)]
pub(crate) enum Commands {
    #[command(about = "Load files belong to the given template", help_template = help::COMMAND_TEMPLATE)]
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
        help_template = help::COMMAND_TEMPLATE
    )]
    Export {
        #[arg(short, long, value_name = "TEMPLATE_NAME", help = "Template used to the file(s) being exported")]
        template: String
    },

    #[command(
        about = "Anonymize PII data in files",
        help_template = help::COMMAND_TEMPLATE
    )]
    Anonify {
        #[arg(short, long, value_name = "TEMPLATE_NAME", help = "Template used to the file(s) being anonymized")]
        template: String
    },

    #[command(about = "The set of database related subcommands")]
    Db {
        #[command(subcommand)]
        command: db::Commands
    }
}
