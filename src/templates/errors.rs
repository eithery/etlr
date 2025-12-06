use anyhow::{anyhow, Error};


pub(crate) fn invalid_template_name(template_name: &str) -> Error {
    anyhow!(
        "'{}' is invalid template name.\n\n\
        Template name should be specified in '<source>.<template_file_name>' form without file extension.\n\
        Only '/', '\\', and '.' symbols can be used as separators.\n\n\
        Example: 'nfs.accounts' or 'nfs/accounts'.",
        template_name
    )
}


pub(crate) fn template_not_found(template_name: &str) -> Error {
    anyhow!("Template '{template_name}' is not found.")
}
