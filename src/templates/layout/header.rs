mod base;
mod date;
mod file_type;
pub(super) mod inbound;
pub(super) mod outbound;


pub(crate) trait FileHeaderTemplate {
    #[allow(dead_code)]
    fn enabled(&self) -> bool;

    #[allow(dead_code)]
    fn tag(&self) -> Option<&str>;
}
