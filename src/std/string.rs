use serde::Deserialize;

pub(crate) trait Normalize {
    fn normalize(&self) -> String;
}


impl Normalize for str {
    fn normalize(&self) -> String {
        self.trim().to_lowercase()
    }
}


pub(crate) fn deserialize<'a, D>(deserializer: D) -> Result<Option<String>, D::Error>
    where D: serde::Deserializer<'a>
{
    let opt = Option::<String>::deserialize(deserializer)?;
    Ok(opt.map(|s| s.normalize()))
}
