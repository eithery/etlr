use serde::Deserialize;
use crate::templates::layout::field::position::FieldPosition;
use crate::templates::defaults::default_true;


#[derive(Debug, Deserialize)]
pub(super) struct RecordCountTemplate {
    pos: FieldPosition,

    #[serde(default = "default_true")]
    include_header: bool,

    #[serde(default = "default_true")]
    include_trailer: bool
}


impl RecordCountTemplate {
    #[allow(dead_code)]
    pub(super) fn pos(&self) -> FieldPosition {
        self.pos
    }


    #[allow(dead_code)]
    pub(super) fn include_header(&self) -> bool {
        self.include_header
    }


    #[allow(dead_code)]
    pub(super) fn include_trailer(&self) -> bool {
        self.include_trailer
    }
}
