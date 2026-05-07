use serde::Deserialize;
use crate::templates::FieldPosition;


#[derive(Debug, Deserialize)]
pub(super) struct RecordIdTemplate {
    pos: FieldPosition
}


impl RecordIdTemplate {
    #[allow(dead_code)]
    fn pos(&self) -> FieldPosition {
        self.pos
    }
}
