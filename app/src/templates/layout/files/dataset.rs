use serde::Deserialize;
use super::data_columns::DataColumnsTemplate;
use super::join::DatasetJoinTemplate;


#[derive(Debug, Deserialize)]
pub(crate) struct DatasetTemplate {
    id: String,
    source: String,
    columns: DataColumnsTemplate,

    #[serde(default)]
    join: Vec<DatasetJoinTemplate>
}


impl DatasetTemplate {
    #[allow(dead_code)]
    fn id(&self) -> &str {
        &self.id
    }


    #[allow(dead_code)]
    fn source(&self) -> &str {
        &self.source
    }


    #[allow(dead_code)]
    fn columns(&self) -> &DataColumnsTemplate {
        &self.columns
    }


    #[allow(dead_code)]
    fn column_names(&self) -> impl Iterator<Item = &str> {
        self.columns.column_names()
    }


    fn joins(&self) -> impl Iterator<Item = &DatasetJoinTemplate> {
        self.join.iter()
    }


    #[allow(dead_code)]
    fn ignored_columns(&self) -> impl Iterator<Item = &str> {
        self.joins()
            .filter_map(|join| join.foreign_key())
            .chain(self.columns.excluded_columns())
    }


    #[allow(dead_code)]
    fn labeled_columns(&self) -> impl Iterator<Item = (&str, &str)> {
        self.columns.labeled_columns()
    }
}
