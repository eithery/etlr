use either::Either;
use serde::Deserialize;
use super::columns::Columns;


#[derive(Debug, Deserialize)]
pub(super) struct DataColumnsTemplate {
    include: Columns,

    #[serde(default)]
    exclude: Vec<String>
}


impl DataColumnsTemplate {
    pub(super) fn column_names(&self) -> impl Iterator<Item = &str> {
        match &self.include {
            Columns::All => Either::Left(std::iter::empty()),
            Columns::Included(cols) => Either::Right(
                cols.iter()
                    .filter_map(move |(name, _)| {
                        if self.exclude.contains(name) { None } else { Some(name.as_str()) }
                    })
            )
        }
    }


    pub(super) fn excluded_columns(&self) -> impl Iterator<Item = &str> {
        self.exclude.iter().map(|s| s.as_str())
    }


    pub(super) fn labeled_columns(&self) -> impl Iterator<Item = (&str, &str)> {
        self.include.labeled_columns()
    }
}
