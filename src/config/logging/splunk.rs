use std::ops::Deref;
use serde::Deserialize;
use crate::env;
use super::BaseLogConfiguration;
use super::defaults;


#[derive(Debug, Deserialize)]
pub(super) struct SplunkConfiguration {
    #[serde(flatten)]
    base: BaseLogConfiguration,

    source: Option<String>,
    host: Option<String>,
    port: Option<u16>,
    token: Option<String>,
    index: Option<String>,
    verify: Option<bool>
}


impl Default for SplunkConfiguration {
    fn default() -> Self {
        Self {
            base: BaseLogConfiguration::default(),
            source: defaults::splunk::source(),
            host: defaults::splunk::host(),
            port: defaults::splunk::port(),
            token: None,
            index: defaults::splunk::index(),
            verify: Some(true)
        }
    }
}


impl Deref for SplunkConfiguration {
    type Target = BaseLogConfiguration;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}


impl SplunkConfiguration {
    pub(super) fn merge(self, other: Self) -> Self {
        Self {
            base: self.base.merge(other.base),
            source: other.source.or(self.source),
            host: other.host.or(self.host),
            port: other.port.or(self.port),
            token: self.token,
            index: other.index.or(self.index),
            verify: other.verify.or(self.verify)
        }
    }


    pub(super) fn apply_env_vars(self) -> Self {
        Self {
            host: env::splunk_host().or(self.host),
            port: env::splunk_port().or(self.port),
            token: env::splunk_token().or(self.token),
            ..self
        }
    }
}
