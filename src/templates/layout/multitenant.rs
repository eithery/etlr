use serde::Deserialize;
use super::trailer::inbound::InboundFileTrailerTemplate;


#[derive(Debug, Deserialize)]
pub(super) struct MultitenantLayoutTemplate {
    subheader: String,
    subtrailer: String,
    trailer: InboundFileTrailerTemplate
}


impl MultitenantLayoutTemplate {
    #[allow(dead_code)]
    pub(super) fn subheader(&self) -> &str {
        &self.subheader
    }


    #[allow(dead_code)]
    pub(super) fn subtrailer(&self) -> &str {
        &self.subtrailer
    }


    #[allow(dead_code)]
    pub(super) fn trailer(&self) -> &InboundFileTrailerTemplate {
        &self.trailer
    }
}
