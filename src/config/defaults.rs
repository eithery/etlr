pub(super) mod app {
    const DEFAULT_INBOX_HOME: &str = "./data/inbox";
    const DEFAULT_OUTBOX_HOME: &str = "./data/outbox";


    pub(crate) fn inbox() -> Vec<String> {
        vec![DEFAULT_INBOX_HOME.to_string()]
    }


    pub(crate) fn outbox() -> Option<String> {
        Some(DEFAULT_OUTBOX_HOME.to_string())
    }
}
