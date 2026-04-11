use serde::Deserialize;
use super::field::position::FieldPosition;


#[derive(Debug, Deserialize)]
pub(super) struct RecordIdTemplate {
    pos: FieldPosition
}


impl RecordIdTemplate {
    #[allow(dead_code)]
    pub(super) fn pos(&self) -> FieldPosition {
        self.pos
    }
}
