use etl_macros::DeserializeYaml;
use serde::Deserialize;
use serde_yaml::Value;
use crate::errors::EtlError;
use crate::templates::prelude::*;
use crate::yaml::{YamlNameValue, YamlReader, errors as err};


#[derive(Debug, DeserializeYaml)]
pub(crate) struct InboundFileEntryTemplate {
    file_type: String,
    columns: Vec<ColumnTemplate>,
    allow_duplicates: bool
}


impl InboundFileEntryTemplate {
    #[allow(dead_code)]
    pub(crate) fn columns(&self) -> impl Iterator<Item = &ColumnTemplate> {
        self.columns.iter()
    }


    #[allow(dead_code)]
    pub(crate) fn allow_duplicates(&self) -> bool {
        self.allow_duplicates
    }
}


impl FileEntry for InboundFileEntryTemplate {
    fn file_type(&self) -> &str {
        &self.file_type
    }
}


impl TryFrom<&Value> for InboundFileEntryTemplate {
    type Error = EtlError;

    fn try_from(payload: &Value) -> Result<Self, Self::Error> {
        let (name, value) = payload.to_name_value()?;
        match value {
            Value::Mapping(m) => {
                Ok(Self {
                    file_type: name.to_string(),
                    columns: m.get_vec("columns")?,
                    allow_duplicates: m.get_bool("allow_duplicates", false)?
                })
            }
            _ => Err(invalid_file_entry_format())
        }
    }
}


fn invalid_file_entry_format() -> EtlError {
    err::invalid_yaml_format("file entry", "Expected a mapping")
}
