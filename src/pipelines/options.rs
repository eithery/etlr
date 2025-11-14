pub(crate) struct FileExportOptions<'a> {
    pub(crate) files: Option<&'a str>,
    pub(crate) file_prefix: Option<&'a str>,
    pub(crate) skip_column_names: bool
}


pub(crate) struct FileImportOptions {
    pub(crate) postprocess: bool,
    pub(crate) all_stages: bool,
    pub(crate) force_update: bool,
    pub(crate) update_only: bool,
    pub(crate) preserve_inbox: bool,
    pub(crate) skip_audit: bool
}
