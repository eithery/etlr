use rstest::*;
use rxpect::expect;
use rxpect::expectations::*;
use rxpect::expectations::iterables::*;
use super::Columns;


const VALID_COLUMNS: &str = r#"
- fund_account_number
- cusip
- office_id: office_code
- rep_id: rep_code
"#;


mod deserialize {
    use super::*;

    #[rstest]
    fn it_deserializes_columns_included_to_dataset() {
        let columns = serde_yaml::from_str::<Columns>(VALID_COLUMNS).unwrap();
        let included = match columns {
            Columns::All => None,
            Columns::Included(cols) => Some(cols)
        };

        expect(included.as_ref()).to_be_some();
        let cols = included.unwrap();
        expect(cols.len()).to_equal(4);

        let expected = [
            ("fund_account_number".to_string(), None),
            ("cusip".to_string(), None),
            ("office_id".to_string(), Some("office_code".to_string())),
            ("rep_id".to_string(), Some("rep_code".to_string()))
        ];
        expect(cols).to_contain_equal_to_all_of(expected);
    }


    #[rstest]
    fn it_considers_all_columns_via_the_special_tag() {
        let columns = serde_yaml::from_str::<Columns>(":all").unwrap();
        expect(columns).to_equal(Columns::All);
    }


    #[rstest]
    fn it_returns_error_for_invalid_columns_tag() {
        let columns = serde_yaml::from_str::<Columns>(":invalid");
        expect(columns.as_ref()).to_be_err();
        let err = columns.unwrap_err().to_string();
        expect(err).to_contain("Invalid format for `include`.");
    }
}


mod labeled_columns {
    use super::*;

    #[rstest]
    fn it_returns_columns_with_custom_labels() {
        let columns = serde_yaml::from_str::<Columns>(VALID_COLUMNS).unwrap();
        let labeled: Vec<_> = columns.labeled_columns().collect();

        expect(labeled.len()).to_equal(2);
        let expected = [
            ("office_id", "office_code"),
            ("rep_id", "rep_code")
        ];
        expect(labeled).to_contain_equal_to_all_of(expected);
    }


    #[rstest]
    fn it_is_empty_for_columns_marked_via_all_tag() {
        let cols: Vec<_> = Columns::All.labeled_columns().collect();
        expect(cols).to_be_empty();
    }
}
