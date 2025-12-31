use serde::Deserialize;


#[allow(dead_code)]
pub(crate) trait Blank {
    fn is_blank(&self) -> bool;


    fn is_not_blank(&self) -> bool {
        !self.is_blank()
    }
}


impl<T: AsRef<str>> Blank for T {
    fn is_blank(&self) -> bool {
        self.as_ref().trim().is_empty()
    }
}


pub(crate) trait Normalize {
    type Output;

    fn normalize(&self) -> Self::Output;
}


impl Normalize for str {
    type Output = String;

    fn normalize(&self) -> String {
        self.trim().to_lowercase()
    }
}


impl<T: AsRef<str>> Normalize for Option<T> {
    type Output = Option<String>;

    fn normalize(&self) -> Option<String> {
        Some(self.as_ref()?.as_ref().normalize())
    }
}


pub(crate) fn deserialize<'a, D>(deserializer: D) -> Result<Option<String>, D::Error>
    where D: serde::Deserializer<'a>
{
    let opt = Option::<String>::deserialize(deserializer)?;
    Ok(opt.map(|s| s.normalize()))
}
