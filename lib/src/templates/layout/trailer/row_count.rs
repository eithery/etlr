use serde::Deserialize;
use crate::templates::prelude::FieldPosition;


#[derive(Debug, Deserialize)]
pub(crate) struct RowCountTemplate {
    pos: FieldPosition,
}


impl RowCountTemplate {
    #[allow(dead_code)]
    fn pos(&self) -> FieldPosition {
        self.pos
    }
}
