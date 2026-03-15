use serde::Deserialize;
use super::format::FileFormat;


#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub(super) struct FileInfoTemplate {
    #[serde(rename = "type")]
    file_type: String,

    #[serde(rename = "name")]
    file_name: Option<String>,

    format: FileFormat
}


impl FileInfoTemplate {
    pub(super) fn file_type(&self) -> &str {
        self.file_type.as_str()
    }


    pub(super) fn category(&self) -> &str {
        self.file_type
            .split_once('.')
            .map_or("", |(category, _)| category)
    }


    pub(super) fn file_name(&self) -> &str {
        self.file_name.as_deref().unwrap_or_else(|| {
            self.file_type
                .split_once('.')
                .map_or(self.file_type(), |(_, name)| name)
        })
    }


    pub(super) fn format(&self) -> FileFormat {
        self.format
    }


    #[allow(dead_code)]
    pub(super) fn delimiter(&self) -> Option<&str> {
        match self.format {
            FileFormat::Pipe => Some("|"),
            FileFormat::CSV => Some(","),
            FileFormat::Tab => Some("\t"),
            FileFormat::Excel => None,
            FileFormat::FixedLength => None
        }
    }


    #[allow(dead_code)]
    pub(super) fn is_csv(&self) -> bool {
        self.format == FileFormat::CSV
    }


    #[allow(dead_code)]
    pub(super) fn is_pipe_delimited(&self) -> bool {
        self.format == FileFormat::Pipe
    }


    #[allow(dead_code)]
    pub(super) fn is_tab_delimited(&self) -> bool {
        self.format == FileFormat::Tab
    }


    #[allow(dead_code)]
    pub(super) fn is_delimited(&self) -> bool {
        self.is_csv() || self.is_pipe_delimited() || self.is_tab_delimited()
    }


    #[allow(dead_code)]
    pub(super) fn is_excel(&self) -> bool {
        self.format == FileFormat::Excel
    }


    #[allow(dead_code)]
    pub(super) fn is_fixed_length(&self) -> bool {
        self.format == FileFormat::FixedLength
    }
}
