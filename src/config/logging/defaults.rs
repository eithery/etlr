pub(super) mod splunk {
    pub(crate) fn source() -> Option<String> {
        Some("ETL Toolkit".to_string())
    }


    pub(crate) fn host() -> Option<String> {
        Some("localhost".to_string())
    }


    pub(crate) fn port() -> Option<u16> {
        Some(8088)
    }


    pub(crate) fn index() -> Option<String> {
        Some("main".to_string())
    }
}
