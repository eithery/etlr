pub(super) mod db {
    use crate::config::database::{ConnectionType, DatabaseDialect};


    pub(crate) fn name() -> Option<String> {
        Some("etl_db".to_string())
    }


    pub(crate) fn host() -> Option<String> {
        Some("localhost".to_string())
    }


    pub(crate) fn dialect() -> Option<DatabaseDialect> {
        Some(DatabaseDialect::MSSql)
    }


    pub(crate) fn connection_type() -> Option<ConnectionType> {
        Some(ConnectionType::Default)
    }
}
