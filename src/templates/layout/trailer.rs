mod base;
pub(super) mod inbound;
pub(super) mod outbound;
mod record_count;


pub(crate) trait FileTrailerTemplate {
    #[allow(dead_code)]
    fn enabled(&self) -> bool;

    #[allow(dead_code)]
    fn tag(&self) -> Option<&str>;

    #[allow(dead_code)]
    fn date_format(&self) -> &str;
}
