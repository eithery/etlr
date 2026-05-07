use rstest::*;
use rstest_reuse::{self, *};
use rxpect::expect;
use rxpect::expectations::*;
use crate::templates::{ColumnValidationTemplate, InvalidValueHandling};


const VALID_TEMPLATE: &str = r#"
validate: true
rule: us_state
reject_invalid: :row
"#;


#[template]
#[rstest]
#[case("true", true, None, None)]
#[case("false", false, None, None)]
#[case("debit_credit", true, Some("debit_credit"), None)]
#[case("rule: debit_credit", true, Some("debit_credit"), None)]
#[case("reject_invalid: true", true, None, Some(InvalidValueHandling::Reject))]
#[case("reject_invalid: false", true, None, Some(InvalidValueHandling::Preserve))]
#[case("reject_invalid: :field", true, None, Some(InvalidValueHandling::RejectField))]
#[case("reject_invalid: :row", true, None, Some(InvalidValueHandling::RejectRow))]
#[case("reject_invalid: :file", true, None, Some(InvalidValueHandling::RejectFile))]
#[case(VALID_TEMPLATE, true, Some("us_state"), Some(InvalidValueHandling::RejectRow))]
fn valid_templates(
    #[case] yaml: &str,
    #[case] validate: bool,
    #[case] rule: Option<&str>,
    #[case] reject_invalid: Option<InvalidValueHandling>
) { }


#[template]
#[rstest]
#[case("rule: 10", "Invalid value for `rule`")]
#[case("reject_invalid: maybe", "Invalid value for `reject_invalid`")]
#[case("reject_invalid: 200", "Invalid value for `reject_invalid`")]
#[case("reject_invalid: :fiels: true", "mapping values are not allowed")]
fn invalid_templates(#[case] yaml: &str, #[case] error_message: &str) {
}


mod deserialize {
    use super::*;


    #[apply(valid_templates)]
    fn it_deserializes_validation_template_from_yaml(
        #[case] yaml: &str,
        #[case] validate: bool,
        #[case] rule: Option<&str>,
        #[case] reject_invalid: Option<InvalidValueHandling>
    ) {
        let validation = serde_yaml::from_str::<ColumnValidationTemplate>(yaml).unwrap();
        expect(validation.validate()).to_equal(validate);
        expect(validation.rule()).to_equal(rule);
        expect(validation.reject_invalid()).to_equal(reject_invalid);
    }


    #[apply(invalid_templates)]
    fn it_returns_error_for_invalid_validation_template(#[case] yaml: &str, #[case] error_message: &str) {
        let validation = serde_yaml::from_str::<ColumnValidationTemplate>(yaml);
        expect(validation.as_ref()).to_be_err();
        let err = validation.unwrap_err().to_string();
        expect(err).to_contain(error_message);
    }
}
