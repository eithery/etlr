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


static INTEGRATED_TEMPLATES: Dir = include_dir!("$CARGO_MANIFEST_DIR/templates");
