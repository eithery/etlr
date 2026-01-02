use rstest::*;
use rstest_reuse::{self, *};
use rxpect::expect;
use rxpect::expectations::{EqualityExpectations, BooleanExpectations, OptionExpectations};
use crate::std::string::*;


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
        expect(String::from(str_value).is_blank()).to_be_true();
        expect(Some(str_value).is_blank()).to_be_true();
    }


    #[apply(non_blank_strings)]
    fn it_returns_false_for_a_non_blank_string(#[case] str_value: &str) {
        expect(str_value.is_blank()).to_be_false();
        expect(String::from(str_value).is_blank()).to_be_false();
        expect(Some(str_value).is_blank()).to_be_false();
    }


    #[rstest]
    fn it_returns_true_for_none() {
        expect((None as Option<&str>).is_blank()).to_be_true();
    }
}


mod is_not_blank {
    use super::*;

    #[apply(blank_strings)]
    fn it_returns_false_for_a_blank_string(#[case] str_value: &str) {
        expect(str_value.is_not_blank()).to_be_false();
        expect(String::from(str_value).is_not_blank()).to_be_false();
        expect(Some(str_value).is_not_blank()).to_be_false();
    }


    #[apply(non_blank_strings)]
    fn it_returns_true_for_a_non_blank_string(#[case] str_value: &str) {
        expect(str_value.is_not_blank()).to_be_true();
        expect(String::from(str_value).is_not_blank()).to_be_true();
        expect(Some(str_value).is_not_blank()).to_be_true();
    }


    #[rstest]
    fn it_returns_false_for_none() {
        expect((None as Option<&str>).is_not_blank()).to_be_false();
    }
}


mod trim {
    use super::*;

    #[rstest]
    #[case("\r\n \t  Some STRING   \n\t\r", "Some STRING")]
    #[case("", "")]
    #[case("    ", "")]
    #[case(&"   VaLUE ".to_string(), "VaLUE")]
    #[case("  TEST StrinG ", "TEST StrinG")]
    #[case("  \r 1234\n\r  \t", "1234")]
    #[case(" TEST\r\n", "TEST")]
    #[case(" __..A \n ", "__..A")]
    #[case(&String::from("  \r\n"), &String::new())]
    fn it_trims_spaces_from_a_string(#[case] str_value: &str, #[case] expected: &str) {
        expect(Some(str_value).trim()).to_equal(Some(expected));
    }


    #[rstest]
    fn it_returns_none_for_none_value() {
        expect((None as Option<String>).trim()).to_be_none();
    }
}


mod chomp {
    use super::*;

    #[rstest]
    #[case("test string\n\r", "test string")]
    #[case(" \n\r", " ")]
    #[case("\n", "")]
    #[case("test123 \n", "test123 ")]
    #[case("test123\r", "test123")]
    fn it_removes_trailing_crlf(#[case] str_value: &str, #[case] expected: &str) {
        expect(str_value.chomp()).to_equal(expected);
        expect(String::from(str_value).chomp()).to_equal(expected);
        expect(Some(str_value).chomp()).to_equal(Some(expected));
    }


    #[rstest]
    fn it_does_not_change_a_string_without_trailing_crlf(
        #[values("", "    ", "\r\n ", "\n ", "test string", "TEST123")]
        str_value: &str
    ) {
        expect(str_value.chomp()).to_equal(str_value);
        expect(String::from(str_value).chomp()).to_equal(str_value);
        expect(Some(str_value).chomp()).to_equal(Some(str_value));
    }


    #[rstest]
    fn it_returns_none_for_none_value() {
        expect((None as Option<&str>).chomp()).to_be_none();
    }
}


mod remove {
    use super::*;

    #[rstest]
    #[case("abracadabra", "a", "brcdbr")]
    #[case("abracadabra", "abrc", "d")]
    #[case("abracadabra", "abracadabra", "")]
    #[case("123", "2", "13")]
    #[case("  .  .   . ", " ", "...")]
    #[case(" . . .", ".", "   ")]
    #[case("  \r 1234\n\r  \t test string ", "\r\n \t", "1234teststring")]
    #[case("Some string", "", "Some string")]
    fn it_removes_chars_from_a_string(#[case] str_value: &str, #[case] chars: &str, #[case] expected: &str) {
        expect(str_value.remove_chars(chars)).to_equal(expected);
        expect(String::from(str_value).remove_chars(chars)).to_equal(expected);
        expect(Some(str_value).remove_chars(chars)).to_equal(Some(expected.to_string()));
    }


    #[rstest]
    fn it_returns_none_for_none_value() {
        expect((None as Option<String>).remove_chars("ABC")).to_be_none();
    }
}


mod normalize {
    use super::*;

    #[rstest]
    #[case("\r\n \t  Some STRING   \n\t\r", "some string")]
    #[case("", "")]
    #[case("    ", "")]
    #[case(&"   VaLUE ".to_string(), "value")]
    #[case("  TEST StrinG ", "test string")]
    #[case("  \r 1234\n\r  \t", "1234")]
    #[case(" TEST\r\n", "test")]
    #[case(" __..A \n ", "__..a")]
    #[case(&String::new(), &String::new())]
    fn it_trims_and_converts_a_string_to_lowercase(#[case] str_value: &str, #[case] expected: &str) {
        expect(str_value.normalize()).to_equal(expected);
        expect(String::from(str_value).normalize()).to_equal(expected);
        expect(Some(str_value).normalize()).to_equal(Some(expected.to_string()));
    }


    #[rstest]
    fn it_returns_none_for_none_value() {
        expect((None as Option<String>).normalize()).to_be_none();
    }
}


mod try_parse {
    use super::*;

    #[rstest]
    fn it_parses_truthy_string_values(
        #[values("true", "True", "TRUE", "  true  \n\r", "1", "Yes", "Y", "y", " YES \n\r", "  1  ", "T", "  t\n\r")]
        str_value: &str
    ) {
        expect(str_value.try_parse()).to_equal(Some(true));
        expect(String::from(str_value).try_parse()).to_equal(Some(true));
        expect(Some(str_value).try_parse()).to_equal(Some(true));
    }


    #[rstest]
    fn it_parses_falsy_string_values(
        #[values("false", "False", "FALSE", "  false \n\r", "0", "No", "N", "n", "NO \n", "  0  ", "F", "  f\n")]
        str_value: &str
    ) {
        expect(str_value.try_parse()).to_equal(Some(false));
        expect(String::from(str_value).try_parse()).to_equal(Some(false));
        expect(Some(str_value).try_parse()).to_equal(Some(false));
    }


    #[rstest]
    fn it_returns_none_for_not_bool_strings(
        #[values(".", "abracadabra", "001", "true1", "false.", "N0", "yes-yes")]
        str_value: &str
    ) {
        expect(str_value.try_parse()).to_be_none();
        expect(String::from(str_value).try_parse()).to_be_none();
        expect(Some(str_value).try_parse()).to_be_none();
    }


    #[rstest]
    fn it_returns_none_for_blank_strings(
        #[values("", "   ", " \t\r\n ", "\n")]
        str_value: &str
    ) {
        expect(str_value.try_parse()).to_be_none();
        expect(String::from(str_value).try_parse()).to_be_none();
        expect(Some(str_value).try_parse()).to_be_none();
    }


    #[rstest]
    fn it_returns_none_for_none_value() {
        expect((None as Option<String>).try_parse()).to_be_none();
    }
}
