use rstest::*;
use rxpect::expect;
use rxpect::expectations::*;
use crate::templates::{ColumnTemplate, ColumnSize};


const STR_COLUMN: &str = r#"
account_number:
  size: 20
  key: true
  required: true
"#;

const DATE_COLUMN: &str = r#"
date_of_birth:
  type: :date
  format: CCYY-MM-DD
  pii: :dob
"#;

const REFERENCE_COLUMN: &str = r#"
risk_tolerance:
  type: :ref
  validate: true
"#;

const BOOL_COLUMN: &str = r#"
active:
  type: :bool
"#;

const BLANK_COLUMN: &str = r#"
approver_name:
"#;

const ANY_SIZE_COLUMN: &str = r#"
notes:
  size: :any
"#;

const INVALID_COLUMN_FORMAT: &str = r#"
account_number: 10..20
"#;

const MULTIENTRY_MAP: &str = r#"
account_number:
  size: 20
  key: true
  required: true
description:
"#;

const INVALID_DATA_ELEMENT: &str = r#"
account_number:
  size: 20
  key: unknown
  required: default
"#;

const INVALID_SIZE: &str = r#"
account_number:
  size: large
  key: true
  required: true
"#;

const INVALID_VALIDATE: &str = r#"
account_number:
  size: 20
  validate: 200
"#;

const INVALID_YAML_FORMAT: &str = r#"
account_number: unknown: invalid
"#;


mod deserialize {
    use super::*;


    #[rstest]
    #[case(STR_COLUMN, "account_number", ":str", None, ColumnSize::Sized(20), None, true, true, false, None)]
    #[case(BLANK_COLUMN, "approver_name", ":str", None, ColumnSize::Default, None, false, false, false, None)]
    #[case(DATE_COLUMN, "date_of_birth", ":date", Some("CCYY-MM-DD"), ColumnSize::Default, None, false, false, false, Some(":dob"))]
    #[case(REFERENCE_COLUMN, "risk_tolerance", ":ref", None, ColumnSize::Default, None, false, false, true, None)]
    #[case(BOOL_COLUMN, "active", ":bool", None, ColumnSize::Default, None, false, false, false, None)]
    #[case(ANY_SIZE_COLUMN, "notes", ":str", None, ColumnSize::Any, None, false, false, false, None)]
    fn it_deserializes_column_from_yaml(
        #[case] yaml: &str,
        #[case] column_name: &str,
        #[case] data_type: &str,
        #[case] data_format: Option<&str>,
        #[case] size: ColumnSize,
        #[case] value: Option<&str>,
        #[case] key: bool,
        #[case] required: bool,
        #[case] validate: bool,
        #[case] pii: Option<&str>
    ) {
        let column = serde_yaml::from_str::<ColumnTemplate>(yaml).unwrap();
        expect(column.name()).to_equal(column_name);
        expect(column.data_type()).to_equal(data_type);
        expect(column.data_format()).to_equal(data_format);
        expect(column.size()).to_equal(size);
        expect(column.value()).to_equal(value);
        expect(column.key()).to_equal(key);
        expect(column.required()).to_equal(required);
        expect(column.pii()).to_equal(pii);
        if validate {
            expect(column.validate()).to_be_some_matching(|v| v.validate());
        } else {
            expect(column.value()).to_be_none();
        }
    }


    #[rstest]
    #[case(INVALID_COLUMN_FORMAT, "Invalid format for `column`.")]
    #[case(MULTIENTRY_MAP, "YAML entries must be single-entry maps.")]
    #[case(INVALID_DATA_ELEMENT, "YAML deserialization. invalid type:")]
    #[case(INVALID_SIZE, "Invalid value for `column.size`.")]
    #[case(INVALID_VALIDATE, "Invalid value for `validate`")]
    #[case(INVALID_YAML_FORMAT, "mapping values are not allowed")]
    fn it_returns_error_for_invalid_column_format(#[case] yaml: &str, #[case] error_message: &str) {
        let column = serde_yaml::from_str::<ColumnTemplate>(yaml);
        expect(column.as_ref()).to_be_err();
        let err = column.unwrap_err().to_string();
        expect(err).to_contain(error_message);
    }
}
