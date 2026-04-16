use std::ops::Deref;
use serde::Deserialize;
use crate::templates::defaults::default_false;
use super::DataElement;


#[derive(Debug, Deserialize)]
pub(crate) struct DataElementTemplate {
    name: String,
    value: Option<String>,

    #[serde(default = "default_false")]
    required: bool,

    pii: Option<String>
}


impl<T> DataElement for T
    where T: Deref<Target = DataElementTemplate>
{
    fn name(&self) -> &str {
        &self.name
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


impl DataElementTemplate {
    pub(super) fn new(name: String, value: Option<String>, required: bool) -> Self {
        Self { name, value, required, pii: None }
    }
}
