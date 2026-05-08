mod base;
mod category;
mod defaults;
pub(crate) mod errors;
pub(crate) mod export;
mod file;
pub(crate) mod import;
pub(crate) mod layout;
pub(crate) mod traits;

use include_dir::{include_dir, Dir};

pub(crate) use layout::fields::Fields;
pub(crate) use layout::fields::column::ColumnTemplate;
pub(crate) use layout::fields::field::FieldTemplate;
pub(crate) use layout::fields::position::FieldPosition;
pub(crate) use layout::files::FileEntry;
pub(crate) use layout::files::inbound::InboundFileEntryTemplate;

#[allow(unused_imports)]
pub(crate) use layout::fields::column::size::ColumnSize;
#[allow(unused_imports)]
pub(crate) use layout::fields::column::validation::ColumnValidationTemplate;
#[allow(unused_imports)]
pub(crate) use layout::fields::column::validation::InvalidValueHandling;


static INTEGRATED_TEMPLATES: Dir = include_dir!("$CARGO_MANIFEST_DIR/templates");
