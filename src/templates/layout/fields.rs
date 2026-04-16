pub(super) mod base;
pub(super) mod column;
pub(super) mod exportable;
pub(super) mod importable;
pub(super) mod position;

use std::ops::Deref;
use base::DataElementTemplate;
use serde::Deserializer;
use position::FieldPosition;


pub(super) trait DataElement {
    fn name(&self) -> &str;

    fn value(&self) -> Option<&str>;

    #[allow(dead_code)]
    fn required(&self) -> bool;

    #[allow(dead_code)]
    fn pii(&self) -> Option<&str>;
}


#[allow(dead_code)]
pub(super) trait Importable {
    fn data_type(&self) -> &str;

    fn key(&self) -> bool;
}


#[allow(dead_code)]
pub(super) trait DataColumn: DataElement + Importable {
    fn size(&self) -> Option<usize>;

    fn validate(&self) -> bool;
}


pub(super) trait Field: DataElement {
    fn pos(&self) -> FieldPosition;


    fn len(&self) -> usize {
        self.pos().len()
    }


    fn deserialize_fields<'de, D>(deserializer: D) -> Result<Vec<Self>, D::Error>
        where Self: Sized + Deref<Target = DataElementTemplate>, D: Deserializer<'de>;
}


#[allow(dead_code)]
pub(super) trait ImportableField: Field + Importable {
    fn format(&self) -> Option<&str>;

    fn exported(&self) -> bool;

    fn discriminator(&self) -> bool;

    fn preserve_invalid(&self) -> bool;
}


pub(super) trait ExportableField: Field {
    fn source(&self) -> Option<&str>;
}
