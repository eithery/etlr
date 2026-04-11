use std::ops::Deref;
use serde::Deserialize;
use crate::templates::defaults::default_false;
use super::FieldTemplate;
use super::position::FieldPosition;


#[derive(Debug, Deserialize)]
pub(crate) struct FieldTemplateBase {
    pub(super) name: String,
    pub(super) pos: FieldPosition,
    pub(super) value: Option<String>,

    #[serde(default = "default_false")]
    pub(super) required: bool,

    pub(super) pii: Option<String>
}


impl<F> FieldTemplate for F
    where F: Deref<Target = FieldTemplateBase>
{
    fn name(&self) -> &str {
        &self.name
    }


    fn pos(&self) -> FieldPosition {
        self.pos
    }


    fn value(&self) -> Option<&str> {
        self.value.as_deref()
    }


    #[allow(dead_code)]
    fn required(&self) -> bool {
        self.required
    }


    #[allow(dead_code)]
    fn pii(&self) -> Option<&str> {
        self.pii.as_deref()
    }
}
