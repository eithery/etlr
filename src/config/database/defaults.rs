pub(super) mod db {
    pub(crate) fn name() -> &'static str {
        "etl_db"
    }


    pub(crate) fn host() -> &'static str {
        "localhost"
    }


    pub(crate) fn driver() -> &'static str {
        "ODBC Driver 17 for SQL Server"
    }
}
