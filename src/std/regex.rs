use regex::{RegexBuilder};
use crate::std::result::Result;


#[allow(dead_code)]
pub(crate) fn replace<F>(str_value: &str, pattern: &str, replacer: F, fallback_value: Option<&str>) -> Result<String>
    where F: FnOnce(&str) -> String
{
    let re = RegexBuilder::new(pattern)
        .case_insensitive(true)
        .build()?;

    let result = re.captures(str_value)
        .and_then(|caps| {
            caps.get(1)
                .map(|m| m.as_str())
                .filter(|s| !s.is_empty())
                .or(fallback_value)
        })
        .map(|value| re.replace(str_value, replacer(value).as_str()).into_owned())
        .unwrap_or_else(|| str_value.to_string());
    Ok(result)
}
