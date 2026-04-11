mod base;
pub(super) mod format;
pub(super) mod inbound;
mod name;
pub(super) mod outbound;

use self::format::FileFormat;


pub(crate) trait FileInfoTemplate {
    fn file_type(&self) -> &str;

    fn format(&self) -> FileFormat;


    #[allow(dead_code)]
    fn category(&self) -> Option<&str> {
        self.file_type()
            .split_once('.')
            .map(|(category, _)| category)
    }


    #[allow(dead_code)]
    fn is_pipe_delimited(&self) -> bool {
        self.format().is_pipe_delimited()
    }


    #[allow(dead_code)]
    fn is_csv(&self) -> bool {
        self.format().is_csv()
    }


    #[allow(dead_code)]
    fn is_tab_delimited(&self) -> bool {
        self.format().is_tab_delimited()
    }


    #[allow(dead_code)]
    fn is_excel(&self) -> bool {
        self.format().is_excel()
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
}
