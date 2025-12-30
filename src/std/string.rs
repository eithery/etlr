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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_str() {
        assert_eq!("\r\n \t  Some STRING   \n\t\r".normalize(), "some string");
        assert_eq!("".normalize(), "");
        assert_eq!("   ".normalize(), "");
        assert_eq!("   VaLUE ".to_string().normalize(), "value");
    }


    #[test]
    fn normalize_option_str() {
        assert_eq!(Some("\r\n \t  Some STRING   \n\t\r").normalize(), Some("some string".to_string()));
        assert_eq!(Some("").normalize(), Some("".to_string()));
        assert_eq!(Some("   ".normalize()), Some("".to_string()));
        assert_eq!(Some("   VaLUE ".to_string()).normalize(), Some("value".to_string()));

        let none: Option<String> = None;
        assert_eq!(none.normalize(), None);
    }
}
