mod base;
mod date;
mod file_type;
pub(super) mod inbound;
pub(super) mod outbound;


pub(crate) trait FileHeaderTemplate {
    fn enabled(&self) -> bool;

    fn tag(&self) -> Option<&str>;
}
