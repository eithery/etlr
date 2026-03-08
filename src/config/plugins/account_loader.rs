use serde::Deserialize;


#[derive(Debug, Deserialize, Default)]
pub(crate) struct AccountLoaderConfiguration {
    #[serde(default)]
    pub(crate) pershing: AccountLoaderVendorConfiguration,

    #[serde(default)]
    pub(crate) schwab: AccountLoaderVendorConfiguration,

    #[serde(default)]
    pub(crate) nfs: AccountLoaderVendorConfiguration
}


impl AccountLoaderConfiguration {
    pub(super) fn merge(self, other: Self) -> Self {
        Self {
            pershing: self.pershing.merge(other.pershing),
            schwab: self.schwab.merge(other.schwab),
            nfs: self.nfs.merge(other.nfs)
        }
    }
}


#[derive(Debug, Deserialize, Default, Clone)]
pub(crate) struct AccountLoaderVendorConfiguration {
    default_rep: Option<String>,
    default_office: Option<String>,
    alternate_rep_code: Option<bool>
}


impl AccountLoaderVendorConfiguration {
    #[allow(dead_code)]
    fn default_rep(&self) -> Option<String> {
        self.default_rep.clone()
    }


    #[allow(dead_code)]
    fn default_office(&self) -> Option<String> {
        self.default_office.clone()
    }


    #[allow(dead_code)]
    fn alternate_rep_code(&self) -> bool {
        self.alternate_rep_code.unwrap_or(false)
    }
}


impl AccountLoaderVendorConfiguration {
    pub(super) fn merge(self, other: Self) -> Self {
        Self {
            default_rep: other.default_rep.or(self.default_rep),
            default_office: other.default_office.or(self.default_office),
            alternate_rep_code: other.alternate_rep_code.or(self.alternate_rep_code)
        }
    }
}
