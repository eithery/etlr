use super::traits::FileTemplate;


pub(crate) struct FileImportTemplate {
}


impl FileTemplate for FileImportTemplate {
    fn load() -> Self {
        Self {}
    }
}
