mod date;
mod file_type;
pub(super) mod inbound;
pub(super) mod outbound;

use crate::templates::prelude::ControlRecord;


pub(crate) trait FileHeaderTemplate: ControlRecord {
}
