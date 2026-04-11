pub(super) mod base;
pub(super) mod exportable;
pub(super) mod importable;
pub(super) mod position;

use position::FieldPosition;


pub(super) trait FieldTemplate {
    fn name(&self) -> &str;

    #[allow(dead_code)]
    fn pos(&self) -> FieldPosition;

    fn value(&self) -> Option<&str>;

    #[allow(dead_code)]
    fn required(&self) -> bool;

    #[allow(dead_code)]
    fn pii(&self) -> Option<&str>;
}
