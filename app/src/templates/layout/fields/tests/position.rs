use rstest::*;
use rstest_reuse::{self, *};
use rxpect::expect;
use rxpect::expectations::{EqualityExpectations, ResultExpectations};
use crate::templates::FieldPosition;


#[template]
#[rstest]
#[case("1..3", 1, 3)]
#[case("0", 0, 0)]
#[case("0..0", 0, 0)]
#[case(" 1..8 ", 1, 8)]
#[case(" 2 ..  4 ", 2, 4)]
#[case("17", 17, 17)]
#[case("  64  \n\r", 64, 64)]
fn valid_string_positions(#[case] str_value: &str, #[case] start: usize, #[case] end: usize) { }


#[template]
#[rstest]
#[case(0)]
#[case(1)]
#[case(1024)]
fn valid_int_positions(#[case] value: u64) { }


#[template]
#[rstest]
#[case("some invalid position")]
#[case("")]
#[case("      ")]
#[case("/n")]
#[case("start: 1")]
#[case("1.2")]
#[case("1...2")]
#[case("1 2")]
#[case("-2")]
#[case("a..b")]
#[case("a: b")]
#[case("a-: +++b")]
#[case("true")]
#[case("key: value: another")]
fn invalid_positions(#[case] str_value: &str) { }


mod new {
    use super::*;

    #[rstest]
    #[case(1, 20)]
    #[case(75, 76)]
    #[case(4, 4)]
    #[case(0, 0)]
    fn it_creates_a_new_instance_based_on_start_and_end_position(#[case] start: usize, #[case] end: usize) {
        let pos = FieldPosition::new(start, end);
        expect(pos.start()).to_equal(start);
        expect(pos.end()).to_equal(end);
    }
}


mod single {
    use super::*;

    #[rstest]
    fn it_creates_a_new_instance_for_single_position(#[values(0, 34, 3, 1234556)] pos: usize) {
        let position = FieldPosition::single(pos);
        expect(position.start()).to_equal(pos);
        expect(position.end()).to_equal(pos);
        expect(position.len()).to_equal(1);
    }
}


mod deserialize {
    use super::*;


    #[apply(valid_string_positions)]
    fn it_deserializes_from_yaml_string_value(#[case] yaml: &str, #[case] start: usize, #[case] end: usize) {
        let position = serde_yaml::from_str::<FieldPosition>(yaml).unwrap();
        expect(position.start()).to_equal(start);
        expect(position.end()).to_equal(end);
    }


    #[apply(valid_int_positions)]
    fn it_deserializes_from_yaml_int_value(#[case] int_value: u64) {
        let value = serde_yaml::to_value(int_value).unwrap();
        let position = serde_yaml::from_value::<FieldPosition>(value).unwrap();
        expect(position.start()).to_equal(int_value as usize);
        expect(position.end()).to_equal(int_value as usize);
    }


    #[apply(invalid_positions)]
    fn it_returns_error_for_invalid_position(#[case] yaml: &str) {
        expect(serde_yaml::from_str::<FieldPosition>(yaml)).to_be_err();
    }
}


mod try_from_str {
    use super::*;


    #[apply(valid_string_positions)]
    fn it_converts_from_string_value(#[case] str_value: &str, #[case] start: usize, #[case] end: usize) {
        let position = FieldPosition::try_from(str_value).unwrap();
        expect(position.start()).to_equal(start);
        expect(position.end()).to_equal(end);
    }


    #[apply(invalid_positions)]
    fn it_returns_error_for_invalid_string_value(#[case] str_value: &str) {
        expect(FieldPosition::try_from(str_value)).to_be_err();
    }
}
