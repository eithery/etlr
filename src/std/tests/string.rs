use rstest::*;
use rstest_reuse::{self, *};
use rxpect::expect;
use rxpect::expectations::{EqualityExpectations, BooleanExpectations, OptionExpectations};
use crate::std::string::*;


mod normalize {
    use super::*;

    #[template]
    #[rstest]
    #[case("\r\n \t  Some STRING   \n\t\r", "some string")]
    #[case("", "")]
    #[case("    ", "")]
    #[case(&"   VaLUE ".to_string(), "value")]
    #[case("  TEST StrinG ", "test string")]
    #[case("  \r 1234\n\r  \t", "1234")]
    #[case(" TEST\r\n", "test")]
    #[case(" __..A \n ", "__..a")]
    fn denormalized_strings(#[case] str_value: &str, #[case] expected: &str) {}


    #[apply(denormalized_strings)]
    fn it_trims_and_converts_a_string_to_lowercase(#[case] str_value: &str, #[case] expected: &str) {
        expect(str_value.normalize()).to_equal(expected);
    }


    #[apply(denormalized_strings)]
    fn it_is_applicable_to_optional_strings(#[case] str_value: &str, #[case] expected: &str) {
        expect(Some(str_value).normalize()).to_equal(Some(expected.to_string()));
    }


    #[rstest]
    fn it_returns_none_for_none_value() {
        expect((None as Option<String>).normalize()).to_be_none();
    }
}


#[template]
#[rstest]
#[case("")]
#[case(" ")]
#[case("\r\n")]
#[case("\t")]
#[case("    \r\n\t   ")]
fn blank_strings(#[case] str_value: &str) {}


#[template]
#[rstest]
#[case("Some string")]
#[case("Some String.  \n\r")]
#[case(".")]
#[case("___")]
fn non_blank_strings(#[case] str_value: &str) {}


mod is_blank {
    use super::*;

    #[apply(blank_strings)]
    fn it_returns_true_for_a_blank_string(#[case] str_value: &str) {
        expect(str_value.is_blank()).to_be_true();
    }


    #[apply(non_blank_strings)]
    fn it_returns_false_for_a_non_blank_string(#[case] str_value: &str) {
        expect(str_value.is_blank()).to_be_false();
    }
}


mod is_not_blank {
    use super::*;

    #[apply(blank_strings)]
    fn it_returns_false_for_a_blank_string(#[case] str_value: &str) {
        expect(str_value.is_not_blank()).to_be_false();
    }


    #[apply(non_blank_strings)]
    fn it_returns_true_for_a_non_blank_string(#[case] str_value: &str) {
        expect(str_value.is_not_blank()).to_be_true();
    }
}
