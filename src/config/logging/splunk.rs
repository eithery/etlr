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
            source: Some(defaults::splunk::source().to_string()),
            host: Some(defaults::splunk::host().to_string()),
            port: Some(defaults::splunk::port()),
            token: None,
            index: Some(defaults::splunk::index().to_string()),
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
    #[allow(dead_code)]
    pub(super) fn source(&self) -> &str {
        self.source.as_deref().unwrap_or(defaults::splunk::source())
    }


    #[allow(dead_code)]
    pub(super) fn host(&self) -> &str {
        self.host.as_deref().unwrap_or(defaults::splunk::host())
    }


    #[allow(dead_code)]
    pub(super) fn port(&self) -> u16 {
        self.port.unwrap_or(defaults::splunk::port())
    }


    #[allow(dead_code)]
    pub(super) fn index(&self) -> &str {
        self.index.as_deref().unwrap_or(defaults::splunk::index())
    }


    #[allow(dead_code)]
    pub(super) fn token(&self) -> Option<&str> {
        self.token.as_deref()
    }


    #[allow(dead_code)]
    pub(super) fn verify(&self) -> bool {
        self.verify.unwrap_or(true)
    }


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
