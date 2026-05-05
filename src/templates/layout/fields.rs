pub(super) mod data_element;
pub(super) mod column;
pub(crate) mod field;
pub(crate) mod position;

#[cfg(test)]
mod tests;

use serde::Deserializer;
use field::FieldTemplate;


pub(crate) trait Fields {
    #[allow(dead_code)]
    fn fields(&self) -> impl Iterator<Item = &FieldTemplate>;


    #[allow(dead_code)]
    fn deserialize<'de, D>(deserializer: D) -> Result<Vec<FieldTemplate>, D::Error>
        where D: Deserializer<'de>;
}
