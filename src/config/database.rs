pub(crate) mod connection;
mod datasource;
mod defaults;
mod dialect;
mod errors;

use std::collections::HashMap;
use serde::Deserialize;
use crate::env;
use crate::std::result::Result;
use connection::ConnectionType;
use datasource::DataSource;
use dialect::DatabaseDialect;
use errors as err;


#[derive(Debug, Deserialize)]
pub(super) struct DatabaseConfiguration {
    dialect: Option<DatabaseDialect>,
    driver: Option<String>,
    host: Option<String>,
    db_name: Option<String>,
    instance_name: Option<String>,
    port: Option<u16>,

    #[serde(rename = "connection")]
    connection_type: Option<ConnectionType>,

    uid: Option<String>,
    pwd: Option<String>,

    #[serde(default, deserialize_with = "DataSource::deserialize")]
    data_sources: HashMap<String, String>
}


impl Default for DatabaseConfiguration {
    fn default() -> Self {
        Self {
            dialect: Some(DatabaseDialect::default()),
            driver: Some(defaults::db::driver().to_string()),
            host: Some(defaults::db::host().to_string()),
            db_name: Some(defaults::db::name().to_string()),
            instance_name: None,
            port: None,
            connection_type: Some(ConnectionType::default()),
            uid: None,
            pwd: None,
            data_sources: DataSource::default().to_map()
        }
    }
}


impl DatabaseConfiguration {
    #[allow(dead_code)]
    pub(super) fn connection_type(&self) -> ConnectionType {
        self.connection_type.unwrap_or_default()
    }


    #[allow(dead_code)]
    pub(super) fn connection_string(&self, data_source_id: Option<&str>) -> Result<String> {
        let prefix = if self.is_trusted_connection() {
            ""
        } else {
            &format!("{}:{}@", self.uid()?.unwrap(), self.pwd()?.unwrap())
        };
        Ok(format!(
            "mssql+pyodbc://{}{}/{}?driver={}",
            prefix,
            self.combined_host_name(":"),
            self.get_db_name(data_source_id),
            self.driver()
        ))
    }


    pub(super) fn get_db_name(&self, data_source_id: Option<&str>) -> &str {
        match data_source_id {
            Some(ds) => self.data_sources.get(ds).expect("Invalid data source ID passed."),
            None => self.db_name.as_deref().expect("Database name is not defined.")
        }
    }


    pub(super) fn merge(self, other: Self) -> Self {
        Self {
            dialect: other.dialect.or(self.dialect),
            driver: other.driver.or(self.driver),
            host: other.host.or(self.host),
            db_name: other.db_name.or(self.db_name),
            instance_name: other.instance_name.or(self.instance_name),
            port: other.port.or(self.port),
            connection_type: other.connection_type.or(self.connection_type),
            uid: other.uid.or(self.uid),
            pwd: other.pwd.or(self.pwd),
            data_sources: DataSource::merge(self.data_sources, other.data_sources)
        }
    }


    pub(super) fn apply_env_vars(self) -> Result<Self> {
        Ok(Self {
            host: env::db_host().or(self.host),
            instance_name: env::db_instance_name().or(self.instance_name),
            port: env::db_port().or(self.port),
            db_name: env::database_name().or(self.db_name),
            connection_type: env::db_auth_type()?.or(self.connection_type),
            uid: env::db_user(),
            pwd: env::db_pwd(),
            ..self
        })
    }


    fn combined_host_name(&self, sep: &str) -> String {
        let db_host = self.host();
        if let Some(port) = self.port {
            return format!("{db_host}{sep}{port}");
        }
        match self.instance_name.as_deref() {
            Some(instance_name) if !instance_name.is_empty() => format!("{db_host}\\{instance_name}"),
            _ => db_host.to_string()
        }
    }


    fn host(&self) -> &str {
        self.host.as_deref().unwrap_or(defaults::db::host())
    }


    #[allow(dead_code)]
    fn db_name(&self) -> &str {
        self.db_name.as_deref().unwrap_or(defaults::db::name())
    }


    fn is_trusted_connection(&self) -> bool {
        self.connection_type.map_or(false, |c| c.is_trusted())
    }


    fn driver(&self) -> &str {
        self.driver.as_deref().unwrap_or(defaults::db::driver())
    }


    fn uid(&self) -> Result<Option<&str>> {
        if self.is_trusted_connection() {
            return Ok(None)
        }
        self.uid.as_deref()
            .map(|val| Some(val))
            .ok_or_else(|| err::user_credentials_not_specified())
    }


    fn pwd(&self) -> Result<Option<&str>> {
        if self.is_trusted_connection() {
            return Ok(None)
        }
        self.pwd.as_deref()
            .map(|val| Some(val))
            .ok_or_else(|| err::user_credentials_not_specified())
    }
}
