use rstest::*;
use rxpect::expect;
use rxpect::expectations::*;
use rxpect::expectations::iterables::*;
use super::DataColumnsTemplate;


const VALID_COLUMNS: &str = r#"
include:
  - fund_account_number
  - cusip
  - office_id: office_code
  - rep_id: rep_code

exclude:
  - id
  - export_value
"#;

const ALL_COLUMNS: &str = r#"
include: :all
"#;

const SAME_COLUMNS_EXCLUDED: &str = r#"
include:
  - account_number
  - office_id: office_code
  - rep_id: rep_code

exclude:
  - account_number
  - rep_id
"#;


mod column_names {
    use super::*;

    #[rstest]
    fn it_returns_dataset_column_names() {
        let columns = serde_yaml::from_str::<DataColumnsTemplate>(VALID_COLUMNS).unwrap();
        let column_names: Vec<_> = columns.column_names().collect();
        expect(column_names).to_contain_equal_to_all_of(["fund_account_number", "cusip", "office_id", "rep_id"]);
    }


    #[rstest]
    fn it_is_empty_for_columns_marked_via_all_tag() {
        let columns = serde_yaml::from_str::<DataColumnsTemplate>(ALL_COLUMNS).unwrap();
        let column_names: Vec<_> = columns.column_names().collect();
        expect(column_names).to_be_empty();
    }


    #[rstest]
    fn it_does_not_include_excluded_columns() {
        let columns = serde_yaml::from_str::<DataColumnsTemplate>(SAME_COLUMNS_EXCLUDED).unwrap();
        let column_names: Vec<_> = columns.column_names().collect();
        expect(column_names.len()).to_equal(1);
        expect(column_names).to_contain_equal_to_all_of(["office_id"]);
    }
}


mod excluded_columns {
    use super::*;

    #[rstest]
    fn it_returns_columns_excluded_from_data_source() {
        let columns = serde_yaml::from_str::<DataColumnsTemplate>(VALID_COLUMNS).unwrap();
        let excluded: Vec<_> = columns.excluded_columns().collect();
        expect(excluded).to_contain_equal_to_all_of(["id", "export_value"]);
    }
}


mod labeled_columns {
    use super::*;

    #[rstest]
    fn it_returns_columns_with_custom_labels() {
        let columns = serde_yaml::from_str::<DataColumnsTemplate>(VALID_COLUMNS).unwrap();
        let labeled: Vec<_> = columns.labeled_columns().collect();
        expect(labeled.len()).to_equal(2);
        let expected = [
            ("office_id", "office_code"),
            ("rep_id", "rep_code")
        ];
        expect(labeled).to_contain_equal_to_all_of(expected);
    }
}
