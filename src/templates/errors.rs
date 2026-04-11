use crate::errors::{EtlError, ErrorKind};


pub(crate) fn template_not_found(template_name: &str) -> EtlError {
    EtlError::new(
        format!("Template `{template_name}` is not found."),
        ErrorKind::TemplateNotFound
    )
}


pub(crate) fn invalid_template_format(template_name: &str) -> EtlError {
    EtlError::new(
        format!("Template `{template_name}` has invalid format."),
        ErrorKind::InvalidTemplateFormat
    )
}


#[allow(dead_code)]
pub(crate) fn invalid_dataset_template_format(template_name: &str) -> EtlError {
    EtlError::new(
        format!("Template `{template_name}` contains invalid or not properly defined `dataset` element."),
        ErrorKind::InvalidTemplateFormat
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


pub(crate) fn missing_section_selector() -> EtlError {
    EtlError::new(
        "Missing `section_selector` element in the template.",
        ErrorKind::InvalidTemplateFormat
    )
}


pub(crate) fn missing_discriminator_field(section_selector: &str) -> EtlError {
    EtlError::new(
        format!("Missing `{section_selector}` key field used to build fixed length row."),
        ErrorKind::InvalidTemplateFormat
    )
}


pub(crate) fn missing_file_section_template(section_id: &str) -> EtlError {
    EtlError::new(
        format!("File section with ID `{section_id}` does not exist in the template."),
        ErrorKind::InvalidTemplateFormat
    )
}
