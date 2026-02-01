pub(super) mod db {
    use crate::config::database::{ConnectionType, DatabaseDialect};


    pub(crate) const NAME: &str = "etl_db";
    pub(crate) const HOST: &str = "localhost";
    pub(crate) const DIALECT: DatabaseDialect = DatabaseDialect::MSSql;
    pub(crate) const CONNECTION_TYPE: ConnectionType = ConnectionType::Default;


    pub(crate) fn name() -> Option<String> {
        Some(NAME.to_string())
    }


    pub(crate) fn host() -> Option<String> {
        Some(HOST.to_string())
    }


    pub(crate) fn dialect() -> Option<DatabaseDialect> {
        Some(DIALECT)
    }


    pub(crate) fn connection_type() -> Option<ConnectionType> {
        Some(CONNECTION_TYPE)
    }
}
