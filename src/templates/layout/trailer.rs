mod base;
pub(super) mod inbound;
pub(super) mod outbound;
mod record_count;


pub(crate) trait FileTrailerTemplate {
    fn enabled(&self) -> bool;

    fn tag(&self) -> Option<&str>;

    fn date_format(&self) -> &str;
}
