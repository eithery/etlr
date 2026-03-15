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
