use std::ops::Deref;
use serde::Deserialize;
use super::base::LayoutTemplateBase;
use super::header::inbound::InboundFileHeaderTemplate;
use super::multitenant::MultitenantLayoutTemplate;
use super::record::FileRecordTemplate;
use super::record_id::RecordIdTemplate;
use super::trailer::inbound::InboundFileTrailerTemplate;
use super::field::importable::ImportableFieldTemplate;


#[derive(Debug, Deserialize)]
pub(crate) struct InboundLayoutTemplate {
    #[serde(flatten)]
    base: LayoutTemplateBase<InboundFileHeaderTemplate, InboundFileTrailerTemplate>,

    record_size: Option<usize>,
    record_id: Option<RecordIdTemplate>,
    multitenant: Option<MultitenantLayoutTemplate>,

    #[serde(default)]
    records: Vec<FileRecordTemplate<ImportableFieldTemplate>>
}


impl Deref for InboundLayoutTemplate {
    type Target = LayoutTemplateBase<InboundFileHeaderTemplate, InboundFileTrailerTemplate>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}


impl InboundLayoutTemplate {
    #[allow(dead_code)]
    pub(super) fn record_size(&self) -> Option<usize> {
        self.record_size
    }


    #[allow(dead_code)]
    pub(super) fn record_id(&self) -> Option<&RecordIdTemplate> {
        self.record_id.as_ref()
    }


    #[allow(dead_code)]
    pub(super) fn multitenant(&self) -> Option<&MultitenantLayoutTemplate> {
        self.multitenant.as_ref()
    }


    #[allow(dead_code)]
    pub(super) fn records(&self) -> impl Iterator<Item = &FileRecordTemplate<ImportableFieldTemplate>> {
        self.records.iter()
    }
}
