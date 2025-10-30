use serde::Deserialize;


#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub(super) enum DatabaseDialect {
    #[serde(rename = ":mssql")]
    MSSql,

    #[serde(rename = ":postgres")]
    Postgres,

    #[serde(rename = ":mysql")]
    MySql,

    #[serde(rename = ":sqlite")]
    Sqlite,

    #[serde(other)]
    Unknown
}
