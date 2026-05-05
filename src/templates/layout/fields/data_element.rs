use serde::Deserialize;
use crate::templates::defaults::default_false;


const DEFAULT_DATA_TYPE: &str = ":str";


#[derive(Debug, Deserialize)]
pub(crate) struct DataElementTemplate {
    #[serde(rename = "type")]
    data_type: Option<String>,

    #[serde(rename = "format")]
    data_format: Option<String>,

    value: Option<String>,

    #[serde(default = "default_false")]
    key: bool,

    #[serde(default = "default_false")]
    required: bool,

    pii: Option<String>
}


impl DataElementTemplate {
    #[allow(dead_code)]
    pub(crate) fn data_type(&self) -> &str {
        self.data_type.as_deref().unwrap_or(DEFAULT_DATA_TYPE)
    }


    #[allow(dead_code)]
    pub(crate) fn data_format(&self) -> Option<&str> {
        self.data_format.as_deref()
    }


    pub(crate) fn value(&self) -> Option<&str> {
        self.value.as_deref()
    }


    #[allow(dead_code)]
    pub(crate) fn key(&self) -> bool {
        self.key
    }


    #[allow(dead_code)]
    pub(crate) fn required(&self) -> bool {
        self.required
    }


    #[allow(dead_code)]
    pub(crate) fn pii(&self) -> Option<&str> {
        self.pii.as_deref()
    }
}


impl Default for DataElementTemplate {
    fn default() -> Self {
        Self
        {
            data_type: Some(DEFAULT_DATA_TYPE.to_string()),
            data_format: None,
            value: None,
            key: false,
            required: false,
            pii: None
        }
    }
}
