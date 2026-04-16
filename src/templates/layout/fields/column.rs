mod size;
mod validation;

use std::collections::HashMap;
use std::ops::Deref;
use serde::{Deserialize, Deserializer};
use crate::templates::defaults::default_false;
use super::{Importable, DataColumn};
use super::base::DataElementTemplate;


#[derive(Debug, Deserialize)]
pub(crate) struct DataColumnTemplate {
    #[serde(flatten)]
    base: DataElementTemplate,

    #[serde(rename = "type")]
    data_type: String,

    #[serde(default = "default_false")]
    key: bool,

    size: Option<usize>,

    #[serde(default = "default_false")]
    validate: bool
}


impl Deref for DataColumnTemplate {
    type Target = DataElementTemplate;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}


impl Importable for DataColumnTemplate {
    #[allow(dead_code)]
    fn data_type(&self) -> &str {
        &self.data_type
    }


    #[allow(dead_code)]
    fn key(&self) -> bool {
        self.key
    }
}


impl DataColumn for DataColumnTemplate {
    #[allow(dead_code)]
    fn size(&self) -> Option<usize> {
        self.size
    }


    #[allow(dead_code)]
    fn validate(&self) -> bool {
        self.validate
    }
}


impl DataColumnTemplate {
    #[allow(dead_code)]
    fn from_yaml<'de, D>(_deserializer: D) -> Result<HashMap<String, Self>, D::Error>
        where D: Deserializer<'de>
    {
        Ok(HashMap::new())
    }
}
