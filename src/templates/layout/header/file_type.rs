use serde::Deserialize;
use crate::templates::layout::fields::position::FieldPosition;


#[derive(Debug, Deserialize)]
pub(super) struct FileTypeTemplate {
    tag: String,
    pos: FieldPosition
}


impl FileTypeTemplate {
    #[allow(dead_code)]
    fn tag(&self) -> &str {
        &self.tag
    }


    #[allow(dead_code)]
    fn pos(&self) -> FieldPosition {
        self.pos
    }
}
