use std::collections::HashMap;
use etl_macros::DeserializeYaml;
use serde::Deserialize;
use serde_yaml::Value;
use crate::errors::EtlError;
use crate::templates::defaults::default_false;
use crate::templates::prelude::*;
use crate::yaml::YamlNameValue;


#[derive(Debug, Deserialize)]
pub(super) struct FileRecordTemplate {
    id: String,

    name: Option<String>,

    pattern: Option<String>,

    #[serde(default = "default_false")]
    required: bool,

    #[serde(default = "default_false")]
    multiple: bool,

    #[serde(default)]
    fields: Vec<RecordField>
}


impl FileRecordTemplate {
    #[allow(dead_code)]
    fn id(&self) -> &str {
        &self.id
    }


    #[allow(dead_code)]
    fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }


    #[allow(dead_code)]
    fn pattern(&self) -> Option<&str> {
        self.pattern.as_deref()
    }


    #[allow(dead_code)]
    fn required(&self) -> bool {
        self.required
    }


    #[allow(dead_code)]
    fn multiple(&self) -> bool {
        self.multiple
    }


    fn fields(&self) -> impl Iterator<Item = &RecordField> {
        self.fields.iter()
    }


    pub(super) fn build_fixed_length_row(&self, field_values: &HashMap<&str, Option<&str>>, record_size: usize) -> Option<String> {
        if !self.exportable_fields().any(|f| field_values.contains_key(f.field_name())) {
            return None
        }
        self.build_fixed_length(field_values, record_size).ok()
    }
}


impl ExportableFields for FileRecordTemplate {
    fn exportable_fields(&self) -> impl Iterator<Item = &FieldTemplate> {
        self.fields()
            .filter_map(RecordField::as_field_template)
    }
}


#[derive(Debug, DeserializeYaml)]
enum RecordField {
    Field(FieldTemplate),

    #[allow(dead_code)]
    OneOf(OneOfSelectorTemplate),

    GroupDescriptor
}


impl RecordField {
    fn as_field_template(&self) -> Option<&FieldTemplate> {
        match self {
            Self::Field(f) => Some(f),
            _ => None
        }
    }
}


impl TryFrom<&Value> for RecordField {
    type Error = EtlError;

    fn try_from(payload: &Value) -> Result<Self, Self::Error> {
        match payload.to_name_value()? {
            (":group", _) => Ok(Self::GroupDescriptor),
            (":one_of", value) => Ok(Self::OneOf(OneOfSelectorTemplate::deserialize(value)?)),
            _ => Ok(Self::Field(payload.try_into()?))
        }
    }
}


#[derive(Debug, Deserialize)]
struct OneOfSelectorTemplate {
    scope: String,
    groups: Vec<FieldGroupSelectorTemplate>
}


impl OneOfSelectorTemplate {
    #[allow(dead_code)]
    fn scope(&self) -> &str {
        &self.scope
    }


    #[allow(dead_code)]
    fn groups(&self) -> impl Iterator<Item = &FieldGroupSelectorTemplate> {
        self.groups.iter()
    }
}


#[derive(Debug, Deserialize)]
struct FieldGroupSelectorTemplate {
    when: String,
    fields: Vec<FieldTemplate>
}


impl FieldGroupSelectorTemplate {
    #[allow(dead_code)]
    fn when(&self) -> &str {
        &self.when
    }


    #[allow(dead_code)]
    fn fields(&self) -> impl Iterator<Item = &FieldTemplate> {
        self.fields.iter()
    }
}
