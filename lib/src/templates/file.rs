use serde::Deserialize;
use super::format::FileFormat;


#[derive(Debug, Deserialize)]
pub(crate) struct FileInfoTemplate {
    #[serde(rename = "type")]
    file_type: String,

    format: FileFormat,

    file_name: Option<String>,

    nameless: Option<NamelessFileInfo>
}


impl FileInfoTemplate {
    pub(super) fn file_type(&self) -> &str {
        self.file_type.as_str()
    }


    pub(super) fn format(&self) -> FileFormat {
        self.format
    }


    pub(crate) fn file_name(&self) -> &str {
        self.file_name.as_deref().unwrap_or_else(|| {
            self.file_type()
                .split_once('.')
                .map_or(self.file_type(), |(_, name)| name)
        })
    }


    #[allow(dead_code)]
    fn category(&self) -> Option<&str> {
        self.file_type()
            .split_once('.')
            .map(|(category, _)| category)
    }


    #[allow(dead_code)]
    fn nameless(&self) -> Option<&NamelessFileInfo> {
        self.nameless.as_ref()
    }


    #[allow(dead_code)]
    fn is_fixed_length(&self) -> bool {
        self.format().is_fixed_length()
    }


    #[allow(dead_code)]
    fn is_delimited(&self) -> bool {
        self.format().is_delimited()
    }


    #[allow(dead_code)]
    fn delimiter(&self) -> Option<char> {
        self.format().delimiter()
    }


    #[allow(dead_code)]
    fn is_pipe_delimited(&self) -> bool {
        self.format().is_pipe_delimited()
    }


    #[allow(dead_code)]
    fn is_tab_delimited(&self) -> bool {
        self.format().is_tab_delimited()
    }


    #[allow(dead_code)]
    fn is_csv(&self) -> bool {
        self.format().is_csv()
    }


    #[allow(dead_code)]
    fn is_excel(&self) -> bool {
        self.format().is_excel()
    }
}


#[derive(Debug, Deserialize)]
struct NamelessFileInfo {
    file_mask: String,
    date_format: String
}


impl NamelessFileInfo {
    #[allow(dead_code)]
    fn file_mask(&self) -> &str {
        &self.file_mask
    }


    #[allow(dead_code)]
    fn date_format(&self) -> &str {
        &self.date_format
    }
}
