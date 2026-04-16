mod columns;
pub(super) mod dataset;
pub(super) mod inbound;
mod join;
pub(super) mod outbound;

use serde::Deserializer;


#[allow(dead_code)]
pub(crate) trait FileEntry {
    fn file_type(&self) -> &str;


    fn deserialize_files<'de, D>(deserializer: D) -> Result<Vec<Self>, D::Error>
        where Self: Sized, D: Deserializer<'de>;
}
