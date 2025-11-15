use super::traits::FileTemplate;


pub(crate) struct FileExportTemplate {
}


impl FileTemplate for FileExportTemplate {
    fn load() -> Self {
        Self {}
    }
}
