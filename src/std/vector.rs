use super::string::Normalize;


pub(crate) trait Prepend {
    fn prepend(self, other: Self) -> Self;
}


impl Prepend for Vec<String> {
    fn prepend(self, other: Self) -> Self {
        match (self.is_empty(), other.is_empty()) {
            (true, _) => other,
            (_, true) => self,
            _ => {
                let mut head: Vec<_> = other.into_iter().map(|x| x.normalize()).collect();
                for x in self.into_iter().map(|x| x.normalize()) {
                    if !head.contains(&x) {
                        head.push(x);
                    }
                }
                head
            }
        }
    }
}
