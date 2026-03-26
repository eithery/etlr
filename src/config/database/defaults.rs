pub(super) mod db {
    pub(crate) const NAME: &str = "etl_db";
    pub(crate) const HOST: &str = "localhost";
    pub(crate) const DRIVER: &str = "ODBC Driver 17 for SQL Server";


    pub(crate) fn name() -> String {
        NAME.to_string()
    }


    pub(crate) fn host() -> String {
        HOST.to_string()
    }


    pub(crate) fn driver() -> String {
        DRIVER.to_string()
    }
}
