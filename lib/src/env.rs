use std::env::{self, VarError};
use std::fmt::Display;
use crate::config::database::connection::ConnectionType;
use crate::std::result::Result;
use crate::std::string::Normalize;


const ENVIRONMENT_ENV_VAR: &str = "ETL_ENVIRONMENT";
const DB_HOST_ENV_VARS: [&str; 2] = ["ETL_DB_HOST", "DB_HOST"];
const DB_PORT_ENV_VARS: [&str; 2] = ["ETL_DB_PORT", "DB_PORT"];
const DB_INSTANCE_ENV_VAR: &str = "ETL_DB_INSTANCE";
const DB_NAME_ENV_VAR: &str = "ETL_DB_NAME";
const DB_USER_ENV_VARS: [&str; 2] = ["ETL_DB_USER", "DB_USER_APP"];
const DB_PWD_ENV_VARS : [&str; 2] = ["ETL_DB_PWD", "DB_PASSWORD_APP"];
const DB_CONNECTION_TYPE_ENV_VARS: [&str; 3] = ["ETL_DB_CONNECTION_TYPE", "ETL_DB_AUTH_TYPE", "DB_AUTH_TYPE"];

const SPLUNK_HOST_ENV_VARS: [&str; 2] = ["SPLUNK_HOST", "SPLUNK_URL"];
const SPLUNK_PORT_ENV_VAR: &str = "SPLUNK_PORT";
const SPLUNK_TOKEN_ENV_VAR: &str = "SPLUNK_TOKEN";

pub(crate) const CONFIG_HOME_ENV_VAR: &str = "ETL_CONFIG_HOME";

const DEV_ENV_NAMES: [&str; 2] = ["dev", "development"];
const TEST_ENV_NAMES: [&str; 2] = ["test", "testing"];
const PROD_ENV_NAMES: [&str; 2] = ["prod", "production"];


#[derive(Debug)]
pub(crate) enum Environment {
    Development,
    Testing,
    Production
}


impl Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Environment::Development => write!(f, "{}", DEV_ENV_NAMES[1]),
            Environment::Testing => write!(f, "{}", TEST_ENV_NAMES[1]),
            Environment::Production => write!(f, "{}", PROD_ENV_NAMES[1])
        }
    }
}


pub(crate) fn current_environment() -> Environment {
    match env::var(ENVIRONMENT_ENV_VAR) {
        Ok(value) => match value.normalize().as_str() {
            v if DEV_ENV_NAMES.contains(&v) => Environment::Development,
            v if TEST_ENV_NAMES.contains(&v) => Environment::Testing,
            v if PROD_ENV_NAMES.contains(&v) => Environment::Production,
            _ => Environment::Development
        },
        Err(_) => Environment::Development
    }
}


pub(crate) fn config_home() -> std::result::Result<String, VarError> {
    env::var(CONFIG_HOME_ENV_VAR)
}


pub(crate) fn db_host() -> Option<String> {
    DB_HOST_ENV_VARS.iter().find_map(|var| env::var(var).ok())
}


pub(crate) fn db_port() -> Option<u16> {
    DB_PORT_ENV_VARS.iter().find_map(|var| env::var(var).ok()?.parse().ok())
}


pub(crate) fn db_instance_name() -> Option<String> {
    env::var(DB_INSTANCE_ENV_VAR).ok()
}


pub(crate) fn database_name() -> Option<String> {
    env::var(DB_NAME_ENV_VAR).ok()
}


pub(crate) fn db_user() -> Option<String> {
    DB_USER_ENV_VARS.iter().find_map(|var| env::var(var).ok())
}


pub(crate) fn db_pwd() -> Option<String> {
    DB_PWD_ENV_VARS.iter().find_map(|var| env::var(var).ok())
}


pub(crate) fn db_connection_type() -> Result<Option<ConnectionType>> {
    DB_CONNECTION_TYPE_ENV_VARS
        .iter()
        .find_map(|var| env::var(var).ok())
        .map(|conn| conn.parse())
        .transpose()
}


pub(crate) fn splunk_host() -> Option<String> {
    SPLUNK_HOST_ENV_VARS.iter().find_map(|var| env::var(var).ok())
}


pub(crate) fn splunk_port() -> Option<u16> {
    env::var(SPLUNK_PORT_ENV_VAR).ok()?.parse().ok()
}


pub(crate) fn splunk_token() -> Option<String> {
    env::var(SPLUNK_TOKEN_ENV_VAR).ok()
}
