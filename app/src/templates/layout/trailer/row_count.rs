use serde::Deserialize;
use crate::templates::FieldPosition;
use crate::templates::defaults::default_true;


#[derive(Debug, Deserialize)]
pub(crate) struct RowCountTemplate {
    pos: FieldPosition,

    #[serde(default = "default_true")]
    include_header: bool,

    #[serde(default = "default_true")]
    include_trailer: bool
}


impl RowCountTemplate {
    #[allow(dead_code)]
    fn pos(&self) -> FieldPosition {
        self.pos
    }


    pub(super) fn include_header(&self) -> bool {
        self.include_header
    }


    pub(super) fn include_trailer(&self) -> bool {
        self.include_trailer
    }
}
