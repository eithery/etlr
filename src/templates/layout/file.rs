use serde::{Deserialize, Deserializer, de};
use serde_yaml::Value;
use super::dataset::DatasetTemplate;


#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub(super) struct OutboundFileTemplate {
    file_type: String,
    file_name: String,
    dataset: DatasetTemplate
}


impl OutboundFileTemplate {
    pub(super) fn deserialize_files<'de, D>(deserializer: D) -> Result<Vec<Self>, D::Error>
        where D: Deserializer<'de>
    {
        let payload = Value::deserialize(deserializer)?;
        match payload {
            Value::Sequence(files) => {
                files
                    .into_iter()
                    .map(|payload| Self::from_yaml::<D>(payload))
                    .collect()
            }
            _ => Err(de::Error::custom("`files` element must contain a sequence of file metadata elements."))
        }
    }


    fn from_yaml<'de, D>(payload: Value) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        match payload {
            Value::Mapping(map) if map.len() == 1 => {
                let (key, val) = map.iter().next().unwrap();
                let file_type = key
                    .as_str()
                    .map(str::to_string)
                    .ok_or_else(|| de::Error::custom("File type key element must be a string."))?;

                let mapping = val
                    .as_mapping()
                    .ok_or_else(|| de::Error::custom("`files` metadata elements must be a map."))?;

                let file_name = mapping
                    .get("file_name")
                    .and_then(Value::as_str)
                    .map(str::to_string)
                    .ok_or_else(|| de::Error::custom("Missing or invalid `file_name` metadata element."))?;

                let dataset = DatasetTemplate::from_yaml::<D>(mapping)?;
                Ok(Self { file_type, file_name, dataset })
            }
            _ => Err(de::Error::custom("`files` entries must be single-entry maps."))
        }
    }
}
