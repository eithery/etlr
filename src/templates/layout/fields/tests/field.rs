use rstest::*;
use rstest_reuse::{self, *};
use rxpect::expect;
use rxpect::expectations::{StringExpectations, EqualityExpectations, ResultExpectations};
use crate::templates::FieldTemplate;


const STR_FIELD: &str = r#"
account_number:
  pos: 10..20
  key: true
  required: true
  exported: true
  pii: :account_number
"#;

const DATE_FIELD: &str = r#"
date_of_birth:
  pos: 14..23
  type: :date
  format: CCYY-MM-DD
  pii: :dob
"#;

const MIN_STR_FIELD: &str = r#"
description: 220..299
"#;

const CHAR_FLAG_FIELD: &str = r#"
network_level: 10
"#;

const EXPORTED_VALUE_FIELD: &str = r#"
sequence_number:
  pos: 4..5
  value: '01'
"#;

const EXPORTED_SOURCE_FIELD: &str = r#"
record_count:
  pos: 40..46
  source: row_count
"#;

const INVALID_FIELD_FORMAT: &str = r#"
account_number: false
"#;

const MULTIENTRY_MAP: &str = r#"
record_count:
  pos: 40..46
  source: row_count
description: 220..299
"#;

const INVALID_DATA_ELEMENT: &str = r#"
account_number:
  pos: 10..20
  key: unknown
  required: default
"#;

const MISSING_POSITION: &str = r#"
account_number:
  key: true
  required: true
"#;

const INVALID_POSITION: &str = r#"
account_number: unknown
"#;

const INVALID_SOURCE: &str = r#"
record_count:
  pos: 40..46
  source: 120
"#;

const INVALID_EXPORTED: &str = r#"
account_number:
  pos: 10..20
  exported: maybe
"#;

const INVALID_DISCRIMINATOR: &str = r#"
account_number:
  pos: 10..20
  discriminator: unknown
"#;

const INVALID_PRESERVE_INVALID: &str = r#"
account_number:
  pos: 10..20
  preserve_invalid: xz
"#;

const INVALID_YAML_FORMAT: &str = r#"
account_number: unknown: invalid
"#;


#[template]
#[rstest]
#[case(STR_FIELD, "account_number", 10, 20, 11, ":str", None, None, None, true, true, true, false, false, Some(":account_number"))]
#[case(DATE_FIELD, "date_of_birth", 14, 23, 10, ":date", Some("CCYY-MM-DD"), None, None, false, false, false, false, false, Some(":dob"))]
#[case(MIN_STR_FIELD, "description", 220, 299, 80,  ":str", None, None, None, false, false, false, false, false, None)]
#[case(CHAR_FLAG_FIELD, "network_level", 10, 10, 1, ":str", None, None, None, false, false, false, false, false, None)]
#[case(EXPORTED_VALUE_FIELD, "sequence_number", 4, 5, 2, ":str", None, Some("01"), None, false, false, false, false, false, None)]
#[case(EXPORTED_SOURCE_FIELD, "record_count", 40, 46, 7, ":str", None, None, Some("row_count"), false, false, false, false, false, None)]
fn valid_fields(
    #[case] yaml: &str,
    #[case] field_name: &str,
    #[case] start_pos: usize,
    #[case] end_pos: usize,
    #[case] len: usize,
    #[case] data_type: &str,
    #[case] data_format: Option<&str>,
    #[case] value: Option<&str>,
    #[case] source: Option<&str>,
    #[case] key: bool,
    #[case] required: bool,
    #[case] exported: bool,
    #[case] discriminator: bool,
    #[case] preserve_invalid: bool,
    #[case] pii: Option<&str>
) { }


#[template]
#[rstest]
#[case(INVALID_FIELD_FORMAT, "Invalid format for `field`.")]
#[case(MULTIENTRY_MAP, "YAML entries must be single-entry maps.")]
#[case(INVALID_DATA_ELEMENT, "YAML deserialization. invalid type:")]
#[case(MISSING_POSITION, "Missing required `pos` value.")]
#[case(INVALID_POSITION, "Parse error.")]
#[case(INVALID_SOURCE, "Invalid value for `source`.")]
#[case(INVALID_EXPORTED, "Invalid value for `exported`.")]
#[case(INVALID_DISCRIMINATOR, "Invalid value for `discriminator`.")]
#[case(INVALID_PRESERVE_INVALID, "Invalid value for `preserve_invalid`.")]
#[case(INVALID_YAML_FORMAT, "mapping values are not allowed")]
fn invalid_fields(#[case] yaml: &str, #[case] error_message: &str) { }


mod deserialize {
    use super::*;


    #[apply(valid_fields)]
    fn it_deserializes_field_from_yaml(
        #[case] yaml: &str,
        #[case] field_name: &str,
        #[case] start_pos: usize,
        #[case] end_pos: usize,
        #[case] len: usize,
        #[case] data_type: &str,
        #[case] data_format: Option<&str>,
        #[case] value: Option<&str>,
        #[case] source: Option<&str>,
        #[case] key: bool,
        #[case] required: bool,
        #[case] exported: bool,
        #[case] discriminator: bool,
        #[case] preserve_invalid: bool,
        #[case] pii: Option<&str>
    ) {
        let field = serde_yaml::from_str::<FieldTemplate>(yaml).unwrap();
        expect(field.name()).to_equal(field_name);
        expect(field.pos().start()).to_equal(start_pos);
        expect(field.pos().end()).to_equal(end_pos);
        expect(field.len()).to_equal(len);
        expect(field.data_type()).to_equal(data_type);
        expect(field.data_format()).to_equal(data_format);
        expect(field.value()).to_equal(value);
        expect(field.source()).to_equal(source);
        expect(field.key()).to_equal(key);
        expect(field.required()).to_equal(required);
        expect(field.exported()).to_equal(exported);
        expect(field.discriminator()).to_equal(discriminator);
        expect(field.preserve_invalid()).to_equal(preserve_invalid);
        expect(field.pii()).to_equal(pii);
    }


    #[apply(invalid_fields)]
    fn it_returns_error_for_invalid_field_format(#[case] yaml: &str, #[case] error_message: &str) {
        let field = serde_yaml::from_str::<FieldTemplate>(yaml);
        expect(field.as_ref()).to_be_err();
        let err = field.unwrap_err().to_string();
        expect(err).to_contain(error_message);
    }
}
