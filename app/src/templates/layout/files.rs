mod columns;
mod data_columns;
pub(super) mod dataset;
pub(crate) mod inbound;
mod join;
pub(crate) mod outbound;

#[cfg(test)]
mod tests;


pub(crate) trait FileEntry {
    fn file_type(&self) -> &str;
}
