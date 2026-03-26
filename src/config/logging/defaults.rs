pub(super) mod splunk {
    pub(crate) const SOURCE: &str = "ETL Toolkit";
    pub(crate) const HOST: &str = "localhost";
    pub(crate) const PORT: u16 = 8088;
    pub(crate) const INDEX: &str = "main";


    pub(crate) fn source() -> String {
        SOURCE.to_string()
    }


    pub(crate) fn host() -> String {
        HOST.to_string()
    }


    pub(crate) fn port() -> u16 {
        PORT
    }


    pub(crate) fn index() -> String {
        INDEX.to_string()
    }
}
