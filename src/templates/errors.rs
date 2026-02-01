use crate::errors::{EtlError, ErrorKind};


pub(crate) fn template_not_found(template_name: &str) -> EtlError {
    EtlError::new(
        format!("Template `{template_name}` is not found."),
        ErrorKind::TemplateNotFound
    )
}


pub(crate) fn invalid_template_content(template_name: &str) -> EtlError {
    EtlError::new(
        format!("Template `{template_name}` has invalid content."),
        ErrorKind::InvalidTemplate
    )
}


pub(crate) fn invalid_template_name(template_name: &str) -> EtlError {
    let message = format!(
        "`{}` is invalid template name.\n\n\
        Template name should be specified in `<source>.<template_file_name>` form without file extension.\n\
        Only `/`, `\\`, and `.` symbols can be used as separators.\n\n\
        Example: `nfs.accounts` or `nfs/accounts`.",
        template_name
    );
    EtlError::new(message, ErrorKind::InvalidTemplateName)
}
