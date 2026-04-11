pub(super) mod base;
pub(super) mod exportable;
pub(super) mod importable;
pub(super) mod position;

use position::FieldPosition;


pub(super) trait FieldTemplate {
    fn name(&self) -> &str;

    fn pos(&self) -> FieldPosition;


    fn len(&self) -> usize {
        self.pos().len()
    }


    fn value(&self) -> Option<&str>;

    #[allow(dead_code)]
    fn required(&self) -> bool;

    #[allow(dead_code)]
    fn pii(&self) -> Option<&str>;
}
