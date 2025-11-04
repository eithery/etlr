use serde::Deserialize;
use crate::std::string::Normalize;


#[derive(Debug, Deserialize, Default)]
pub(crate) struct Inbox(Vec<String>);


impl Inbox {
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }


    pub(crate) fn prepend(self, other: Self) -> Self {
        match (self.is_empty(), other.is_empty()) {
            (true, _) => other,
            (_, true) => self,
            _ => {
                let mut head: Vec<_> = other.0.into_iter().map(|x| x.normalize()).collect();
                for x in self.0.into_iter().map(|x| x.normalize()) {
                    if !head.contains(&x) {
                        head.push(x);
                    }
                }
                Self(head)
            }
        }
    }
}
