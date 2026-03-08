pub(super) mod splunk {
    pub(crate) fn source() -> &'static str {
        "ETL Toolkit"
    }


    pub(crate) fn host() -> &'static str {
        "localhost"
    }


    pub(crate) fn port() -> u16 {
        8088
    }


    pub(crate) fn index() -> &'static str {
        "main"
    }
}
