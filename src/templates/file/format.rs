use serde::Deserialize;


#[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "lowercase")]
pub(crate) enum FileFormat {
    #[serde(rename = ":pipe")]
    Pipe,

    #[serde(rename = ":csv")]
    CSV,

    #[serde(rename = ":tab")]
    Tab,

    #[serde(rename = ":excel")]
    Excel,

    #[serde(rename = ":fixed-length")]
    FixedLength
}


impl FileFormat {
    pub(super) fn is_pipe_delimited(&self) -> bool {
        self == &FileFormat::Pipe
    }


    pub(super) fn is_csv(&self) -> bool {
        self == &FileFormat::CSV
    }


    pub(super) fn is_tab_delimited(&self) -> bool {
        self == &FileFormat::Tab
    }


    #[allow(dead_code)]
    pub(super) fn is_excel(&self) -> bool {
        self == &FileFormat::Excel
    }


    #[allow(dead_code)]
    pub(super) fn is_fixed_length(&self) -> bool {
        self == &FileFormat::FixedLength
    }


    pub(super) fn is_delimited(&self) -> bool {
        self.is_csv() || self.is_pipe_delimited() || self.is_tab_delimited()
    }


    #[allow(dead_code)]
    pub(super) fn delimiter(&self) -> Option<char> {
        match &self {
            Self::Pipe => Some('|'),
            Self::CSV => Some(','),
            Self::Tab => Some('\t'),
            Self::Excel => None,
            Self::FixedLength => None
        }
    }
}
