use serde::{Deserialize, Deserializer, de};
use serde_yaml::Value;
use super::FileEntry;


#[derive(Debug, Deserialize)]
pub(crate) struct InboundFileTemplate {
    file_type: String,

    // #[serde(default, deserialize_with = "DataColumnTemplate::from_yaml")]
    // columns: HashMap<String, DataColumnTemplate>
}


impl FileEntry for InboundFileTemplate {
    fn file_type(&self) -> &str {
        &self.file_type
    }


    fn deserialize_files<'de, D>(deserializer: D) -> Result<Vec<Self>, D::Error>
        where D: Deserializer<'de>
    {
        let value = Value::deserialize(deserializer)?;
        match value {
            Value::Sequence(files) => {
                files.into_iter()
                    .map(|file| Self::from_yaml::<D>(file))
                    .collect()
            }
            _ => Err(de::Error::custom("`files` element must contain a sequence of file metadata elements."))
        }
    }
}


impl InboundFileTemplate {
    // fn columns(&self) -> impl Iterator<Item = (&str, &DataColumnTemplate)> {
    //     self.columns
    //         .iter()
    //         .map(|(name, col)| (name.as_str(), col))
    // }


    fn from_yaml<'de, D>(value: Value) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        match value {
            Value::Mapping(mapping) if mapping.len() == 1 => {
                let (key, _) = mapping.iter().next().unwrap();
                let file_type = key
                    .as_str()
                    .map(str::to_string)
                    .ok_or_else(|| de::Error::custom("File type key element must be a string."))?;
                Ok(Self { file_type })
            }
            _ => Err(de::Error::custom("`files` entries must be single-entry maps."))
        }
    }
}
