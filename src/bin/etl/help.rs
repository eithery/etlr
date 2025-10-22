use constcat::concat;


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


pub(crate) const DEFAULT_TEMPLATE: &str = concat!(HELP_PREFIX, CUSTOM_USAGE, COMMANDS, CUSTOM_OPTIONS);
pub(crate) const COMMAND_GROUP_TEMPLATE: &str = concat!(HELP_PREFIX, DEFAULT_USAGE, COMMANDS, DEFAULT_OPTIONS);
pub(crate) const COMMAND_TEMPLATE: &str = concat!(HELP_PREFIX, DEFAULT_USAGE, DEFAULT_OPTIONS);
