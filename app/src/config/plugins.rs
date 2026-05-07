pub(super) mod account_loader;

use serde::Deserialize;
use self::account_loader::AccountLoaderConfiguration;


#[derive(Debug, Deserialize, Default)]
pub(super) struct PluginsConfiguration {
    #[serde(default)]
    pub(super) account_loader: AccountLoaderConfiguration
}


impl PluginsConfiguration {
    pub(super) fn merge(self, other: Self) -> Self {
        Self {
            account_loader: self.account_loader.merge(other.account_loader)
        }
    }
}
