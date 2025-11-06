pub(crate) mod connection;
mod datasource;
mod defaults;
mod dialect;

use std::collections::HashMap;
use serde::Deserialize;
use crate::env;
use connection::ConnectionType;
use datasource::DataSource;
use dialect::DatabaseDialect;


#[derive(Debug, Deserialize)]
pub(super) struct DatabaseConfiguration {
    dialect: Option<DatabaseDialect>,
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
            dialect: defaults::db::dialect(),
            host: defaults::db::host(),
            db_name: defaults::db::name(),
            instance_name: None,
            port: None,
            connection_type: defaults::db::connection_type(),
            uid: None,
            pwd: None,
            data_sources: DataSource::default().to_map()
        }
    }
}


impl DatabaseConfiguration {
    pub(super) fn merge(self, other: Self) -> Self {
        Self {
            dialect: other.dialect.or(self.dialect),
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


    pub(super) fn apply_env_vars(self) -> Self {
        Self {
            host: env::db_host().or(self.host),
            instance_name: env::db_instance_name().or(self.instance_name),
            port: env::db_port().or(self.port),
            db_name: env::database_name().or(self.db_name),
            connection_type: env::db_auth_type().or(self.connection_type),
            uid: env::db_user(),
            pwd: env::db_pwd(),
            ..self
        }
    }
}
