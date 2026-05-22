use etl_macros::DeserializeYaml;
use serde::Deserialize;
use serde_yaml::Value;
use crate::errors::EtlError;
use crate::yaml::{YamlNameValueMap, YamlReader, errors as err};
use super::FileEntry;
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
        let (file_type, value) = payload.to_name_value_map()?;
        match value {
            Value::Mapping(m) => {
                Ok(Self {
                    file_type,
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
