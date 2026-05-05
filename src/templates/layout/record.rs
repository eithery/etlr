use serde::Deserialize;
use crate::templates::{Fields, FieldTemplate};
use crate::templates::defaults::default_false;


#[derive(Debug, Deserialize)]
pub(super) struct FileRecordTemplate {
    id: String,

    name: Option<String>,

    pattern: Option<String>,

    #[serde(default = "default_false")]
    required: bool,

    #[serde(default = "default_false")]
    multiple: bool,

    #[allow(dead_code)]
    #[serde(default)]
    fields: Vec<FieldTemplate>
}


impl FileRecordTemplate {
    #[allow(dead_code)]
    pub(crate) fn id(&self) -> &str {
        &self.id
    }


    #[allow(dead_code)]
    fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }


    #[allow(dead_code)]
    fn pattern(&self) -> Option<&str> {
        self.pattern.as_deref()
    }


    #[allow(dead_code)]
    fn required(&self) -> bool {
        self.required
    }


    #[allow(dead_code)]
    fn multiple(&self) -> bool {
        self.multiple
    }
}


impl Fields for FileRecordTemplate {
    fn fields(&self) -> impl Iterator<Item = &FieldTemplate> {
        self.fields.iter()
    }
}
