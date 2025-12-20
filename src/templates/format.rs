use serde::Deserialize;


#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub(super) enum FileFormat {
    #[serde(rename = ":pipe")]
    Pipe,

    #[serde(rename = ":csv")]
    CSV,

    #[serde(rename = ":excel")]
    Excel,

    #[serde(rename = ":fixed-length")]
    FixedLength
}
