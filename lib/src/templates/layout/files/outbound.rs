use etl_macros::DeserializeYaml;
use serde::Deserialize;
use serde_yaml::Value;
use crate::errors::EtlError;
use crate::templates::FileEntry;
use crate::yaml::{YamlNameValue, YamlReader, errors as err};
use super::dataset::DatasetTemplate;


#[derive(Debug, DeserializeYaml)]
pub(crate) struct OutboundFileEntryTemplate {
    file_type: String,
    file_name: String,
    dataset: DatasetTemplate
}


impl OutboundFileEntryTemplate {
    #[allow(dead_code)]
    fn outbound_file_name(&self) -> &str {
        &self.file_name
    }


    #[allow(dead_code)]
    fn dataset(&self) -> &DatasetTemplate {
        &self.dataset
    }
}


impl FileEntry for OutboundFileEntryTemplate {
    fn file_type(&self) -> &str {
        &self.file_type
    }
}


impl TryFrom<&Value> for OutboundFileEntryTemplate {
    type Error = EtlError;

    fn try_from(payload: &Value) -> Result<Self, Self::Error> {
        let (name, value) = payload.to_name_value()?;
        match value {
            Value::Mapping(m) => {
                Ok(Self {
                    file_type: name.to_string(),
                    file_name: m.get_string("file_name")?,
                    dataset: m.deserialize("dataset")?
                })
            }
            _ => Err(invalid_file_entry_format())
        }
    }
}


fn invalid_file_entry_format() -> EtlError {
    err::invalid_yaml_format("file entry", "Expected a mapping (object)")
}
