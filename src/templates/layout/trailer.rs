mod base;
pub(super) mod inbound;
pub(super) mod outbound;
mod row_count;

use super::trailer::row_count::RowCountTemplate;


pub(crate) trait FileTrailerTemplate {
    fn enabled(&self) -> bool;

    fn tag(&self) -> Option<&str>;

    fn date_format(&self) -> &str;

    fn row_count(&self) -> Option<&RowCountTemplate>;


    #[allow(dead_code)]
    fn row_count_includes_header(&self) -> bool {
        self.row_count()
            .map_or(false, |rc| rc.include_header())
    }


    #[allow(dead_code)]
    fn row_count_includes_trailer(&self) -> bool {
        self.row_count()
            .map_or(false, |rc| rc.include_trailer())
    }
}
