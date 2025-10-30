pub(crate) trait Normalize {
    fn normalize(&self) -> String;
}


impl Normalize for str {
    fn normalize(&self) -> String {
        self.trim().to_lowercase()
    }
}
