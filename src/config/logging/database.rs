use std::ops::Deref;
use serde::Deserialize;
use super::BaseLogConfiguration;


#[derive(Debug, Deserialize, Default)]
pub(super) struct DatabaseLogConfiguration {
    #[serde(flatten)]
    base: BaseLogConfiguration
}


impl Deref for DatabaseLogConfiguration {
    type Target = BaseLogConfiguration;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}


impl DatabaseLogConfiguration {
    pub(super) fn merge(self, other: Self) -> Self {
        Self {
            base: self.base.merge(other.base)
        }
    }
}
