use serde::Deserialize;


pub(crate) fn deserialize<'a, D>(deserializer: D) -> Result<Option<String>, D::Error>
    where D: serde::Deserializer<'a>
{
    let opt = Option::<String>::deserialize(deserializer)?;
    Ok(opt.map(|s| s.normalize()))
}


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


#[allow(dead_code)]
pub(crate) trait Chomp {
    fn chomp(&self) -> &str;
}


impl<T: AsRef<str>> Chomp for T {
    fn chomp(&self) -> &str {
        self.as_ref().trim_end_matches(&['\r', '\n'])
    }
}


#[allow(dead_code)]
pub(crate) trait Remove {
    fn remove_chars(&self, chars: &str) -> String;
}


impl<T: AsRef<str>> Remove for T {
    fn remove_chars(&self, chars: &str) -> String {
        self.as_ref()
            .chars()
            .filter(|c| !chars.contains(*c))
            .collect()
    }
}


pub(crate) trait Normalize {
    fn normalize(&self) -> String;
}


impl<T: AsRef<str>> Normalize for T {
    fn normalize(&self) -> String {
        self.as_ref().trim().to_lowercase()
    }
}


#[allow(dead_code)]
pub(crate) trait TryParse<T> {
    fn try_parse(&self) -> Option<T>;
}


impl<T: AsRef<str>> TryParse<bool> for T {
    fn try_parse(&self) -> Option<bool> {
        match self.normalize().as_str() {
            "1" | "y" | "yes" | "t" | "true" => Some(true),
            "0" | "n" | "no" | "f" | "false" => Some(false),
            _ => None
        }
    }
}
