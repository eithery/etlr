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


impl Blank for str {
    fn is_blank(&self) -> bool {
        self.trim().is_empty()
    }
}


impl<T: AsRef<str>> Blank for Option<T> {
    fn is_blank(&self) -> bool {
        self.as_ref().map_or(true, |s| s.as_ref().is_blank())
    }
}


#[allow(dead_code)]
pub(crate) trait OptionTrim{
    fn trim(&self) -> Option<&str>;
}


impl <T: AsRef<str>> OptionTrim for Option<T> {
    fn trim(&self) -> Option<&str> {
        self.as_ref().map(|s| s.as_ref().trim())
    }
}


#[allow(dead_code)]
pub(crate) trait Chomp {
    type Output<'a> where Self: 'a;

    fn chomp(&self) -> Self::Output<'_>;
}


impl Chomp for str {
    type Output<'a> = &'a str;

    fn chomp(&self) -> &str {
        self.trim_end_matches(&['\r', '\n'])
    }
}


impl<T: AsRef<str>> Chomp for Option<T> {
    type Output<'a> = Option<&'a str> where T: 'a;

    fn chomp(&self) -> Option<&str> {
        self.as_ref().map(|s| s.as_ref().chomp())
    }
}


#[allow(dead_code)]
pub(crate) trait Remove {
    type Output;

    fn remove_chars(&self, chars: &str) -> Self::Output;
}


impl Remove for str {
    type Output = String;

    fn remove_chars(&self, chars: &str) -> String {
        self.chars()
            .filter(|c| !chars.contains(*c))
            .collect()
    }
}


impl<T: AsRef<str>> Remove for Option<T> {
    type Output = Option<String>;

    fn remove_chars(&self, chars: &str) -> Option<String> {
        self.as_ref().map(|s| s.as_ref().remove_chars(chars))
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
        self.as_ref().map(|s| s.as_ref().normalize())
    }
}


#[allow(dead_code)]
pub(crate) trait TryParse<T> {
    fn try_parse(&self) -> Option<T>;
}


impl TryParse<bool> for str {
    fn try_parse(&self) -> Option<bool> {
        match self.normalize().as_str() {
            "1" | "y" | "yes" | "t" | "true" => Some(true),
            "0" | "n" | "no" | "f" | "false" => Some(false),
            _ => None
        }
    }
}


impl<T: AsRef<str>> TryParse<bool> for Option<T> {
    fn try_parse(&self) -> Option<bool> {
        self.as_ref().and_then(|s| s.as_ref().try_parse())
    }
}
