mod size;
mod validation;

use std::collections::HashMap;
use std::ops::Deref;
use serde::{Deserialize, Deserializer};
use crate::templates::defaults::default_false;
use super::data_element::DataElementTemplate;


#[derive(Debug, Deserialize)]
pub(crate) struct DataColumnTemplate {
    name: String,

    #[serde(flatten)]
    base: DataElementTemplate,

    size: Option<usize>,

    #[serde(default = "default_false")]
    validate: bool
}


impl DataColumnTemplate {
    #[allow(dead_code)]
    pub(crate) fn name(&self) -> &str {
        &self.name
    }


    #[allow(dead_code)]
    fn size(&self) -> Option<usize> {
        self.size
    }


    #[allow(dead_code)]
    fn validate(&self) -> bool {
        self.validate
    }


    #[allow(dead_code)]
    fn from_yaml<'de, D>(_deserializer: D) -> Result<HashMap<String, Self>, D::Error>
        where D: Deserializer<'de>
    {
        Ok(HashMap::new())
    }
}


impl Deref for DataColumnTemplate {
    type Target = DataElementTemplate;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
