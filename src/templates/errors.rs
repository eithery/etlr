use crate::errors::{EtlError, ErrorKind};
use crate::std::result::Result;


pub(crate) fn template_not_found<T>(template_name: &str) -> Result<T> {
    Err(EtlError::new(
        format!("Template `{template_name}` is not found."),
        ErrorKind::TemplateNotFound
    ))
}


pub(crate) fn invalid_template_name_error(template_name: &str) -> EtlError {
    let message = format!(
        "`{}` is invalid template name.\n\n\
        Template name should be specified in `<source>.<template_file_name>` form without file extension.\n\
        Only `/`, `\\`, and `.` symbols can be used as separators.\n\n\
        Example: `nfs.accounts` or `nfs/accounts`.",
        template_name
    );
    EtlError::new(message, ErrorKind::InvalidTemplateName)
}
