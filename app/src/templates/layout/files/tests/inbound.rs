use rstest::*;
use rxpect::expect;
use rxpect::expectations::*;
use crate::templates::{FileEntry, InboundFileEntryTemplate};


const VALID_CUSTOMERS: &str = r#"
std.customers:
  columns:
    - customer_type:
        size: 10
        type: :enum
        required: true
    - last_name:
    - gender:
        type: :ref
        validate: true
"#;

const INVALID_FORMAT: &str = r#"
std.master_accounts: false
"#;

const MULTIENTRY_MAP: &str = r#"
std.customers:
  columns:
    - last_name:
    - gender:
        type: :ref
        validate: true
std.master_accounts:
  columns:
    - account_number:
"#;

const INVALID_COLUMNS: &str = r#"
std.master_accounts:
  columns: true
"#;

const MISSING_COLUMNS: &str = r#"
std.master_accounts:
  allow_duplicates: false
"#;

const INVALID_ALLOW_DUPLICATES: &str = r#"
std.master_accounts:
  columns:
    - account_number:
  allow_duplicates: unknown
"#;


mod deserialize {
    use super::*;


    #[rstest]
    #[case(VALID_CUSTOMERS, "std.customers", 3, false)]
    fn it_deserializes_file_entry_template_from_yaml(
        #[case] yaml: &str,
        #[case] file_type: &str,
        #[case] column_count: usize,
        #[case] allow_duplicates: bool 
    ) {
        let file_entry = serde_yaml::from_str::<InboundFileEntryTemplate>(yaml).unwrap();

        expect(file_entry.file_type()).to_equal(file_type);
        expect(file_entry.columns().count()).to_equal(column_count);
        expect(file_entry.allow_duplicates()).to_equal(allow_duplicates);
    }


    #[rstest]
    #[case(INVALID_FORMAT, "Invalid format for `file entry`.")]
    #[case(MULTIENTRY_MAP, "YAML entries must be single-entry maps.")]
    #[case(INVALID_COLUMNS, "Invalid format for `columns`.")]
    #[case(MISSING_COLUMNS, "Missing required `columns` value.")]
    #[case(INVALID_ALLOW_DUPLICATES, "Invalid value for `allow_duplicates`.")]
    fn it_returns_error_for_invalid_file_entry_format(#[case] yaml: &str, #[case] error_message: &str) {
        let file_entry = serde_yaml::from_str::<InboundFileEntryTemplate>(yaml);
        expect(file_entry.as_ref()).to_be_err();
        let err = file_entry.unwrap_err().to_string();
        expect(err).to_contain(error_message);
    }
}
