use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use rstest::*;
use rxpect::expect;
use rxpect::expectations::{EqualityExpectations, ResultExpectations};
use crate::std::decimal;
use crate::std::errors as err;


mod parse_format {
    use super::*;


    #[rstest]
    #[case("s9(09)v9(04)", (true, 9, 4))]
    #[case("S9(12)V99", (true, 12, 2))]
    #[case("9(05)v9(02)", (false, 5, 2))]
    #[case("s9(4)", (true, 4, 0))]
    #[case("9(20)", (false, 20, 0))]
    fn it_decodes_pic_format_into_the_parts(#[case] pic_format: &str, #[case] expected: (bool, usize, usize)) {
        let result = decimal::parse_format(pic_format);
        expect(result.as_ref()).to_be_ok();
        expect(result.unwrap()).to_equal(expected);
    }


    #[rstest]
    #[case("9(200)")]
    #[case("8(20)")]
    fn it_returns_errors_for_invalid_pic_format(#[case] pic_format: &str) {
        let result = decimal::parse_format(pic_format);
        expect(result.as_ref()).to_be_err();
        expect(result.unwrap_err().to_string()).to_equal(err::invalid_cobol_pic_format(pic_format).to_string());
    }
}


mod parse {
    use super::*;


    #[rstest]
    #[case("4118{", "s9(09)v9(04)", dec!(4.1180))]
    #[case("00004118L", "s9(09)v9(04)", dec!(-4.1183))]
    #[case("4118L", "s9(09)", dec!(-41183))]
    #[case("123456", "9(09)v99", dec!(1234.56))]
    #[case("12345678901234567890123456", "9(20)v9(6)", dec!(12345678901234567890.123456))]
    fn it_parses_string_in_pic_format_to_decimal(#[case] str_value: &str, #[case] pic_format: &str, #[case] expected: Decimal) {
        let result = decimal::parse(str_value, pic_format);
        expect(result.as_ref()).to_be_ok();
        expect(result.unwrap()).to_equal(expected);
    }
}
