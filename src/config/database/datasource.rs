use std::collections::HashMap;
use serde::Deserialize;
use crate::std::string::Normalize;
use super::defaults;


#[derive(Debug, Deserialize)]
pub(super) struct DataSource {
    id: String,
    db_name: String
}


impl Default for DataSource {
    fn default() -> Self {
        Self { id: ":default".to_string(), db_name: defaults::db::name().unwrap() }
    }
}


impl DataSource {
    pub(super) fn deserialize<'a, D>(deserializer: D) -> Result<HashMap<String, String>, D::Error>
        where D: serde::Deserializer<'a>
    {
        let data_sources = Option::<Vec::<DataSource>>::deserialize(deserializer)?;
        match data_sources {
            Some(dss) => {
                Ok(dss.into_iter().map(|ds| (ds.id, ds.db_name)).collect())
            },
            None => Ok(HashMap::new())
        }
    }


    pub(super) fn to_map(self) -> HashMap<String, String> {
        HashMap::from([(self.id, self.db_name)])
    }


    pub(super) fn merge(mut this: HashMap<String, String>, other: HashMap<String, String>) -> HashMap<String, String> {
        if other.is_empty() {
            return this;
        }
        this.extend(other.into_iter().map(|(k, v)| (k.normalize(), v)));
        this
    }
}
