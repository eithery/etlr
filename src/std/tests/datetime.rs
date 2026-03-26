use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use rstest::*;
use rxpect::expect;
use rxpect::expectations::{EqualityExpectations, ResultExpectations, ProjectedResultExpectations};
use crate::std::datetime::{DateTime, DEFAULT_TIMESTAMP_FORMAT};


mod format_with {
    use super::*;


    #[rstest]
    #[case(":default", "20260314")]
    #[case(DEFAULT_TIMESTAMP_FORMAT, "20260314T125306")]
    #[case(":iso", "2026-03-14")]
    #[case("YYYY-MM-DDTHH:MM:SS", "2026-03-14T12:53:06")]
    #[case(":iso_space", "2026-03-14")]
    #[case("YYYY-mm-dd HH:MM:SS", "2026-03-14 12:53:06")]
    #[case(":us", "03/14/2026")]
    #[case("MM/DD/YYYY HH:MM:SS", "03/14/2026 12:53:06")]
    #[case(":eu", "14/03/2026")]
    #[case("DD/MM/CCYY HH:MM:SS", "14/03/2026 12:53:06")]
    #[case("CCYYMMDD", "20260314")]
    #[case("CCYYMMDDTHHMMSS", "20260314T125306")]
    #[case("YYYY-MM-DD", "2026-03-14")]
    #[case("YYYY-MM-DDTHH:MM:SS", "2026-03-14T12:53:06")]
    fn it_crates_timestamp_with_date_format(
        #[case] date_format: &str,
        #[case] expected: &str,
    ) {
        let date = NaiveDate::from_ymd_opt(2026, 03, 14).unwrap();
        let time = NaiveTime::from_hms_opt(12, 53, 06).unwrap();
        let datetime_value = NaiveDateTime::new(date, time);
        expect(datetime_value.format_with(date_format)).to_be_ok_and().to_equal(expected);
    }


    #[rstest]
    fn it_returns_errors_for_invalid_format() {
        let date = NaiveDate::from_ymd_opt(2026, 03, 14).unwrap();
        let time = NaiveTime::from_hms_opt(12, 53, 06).unwrap();
        let datetime_value = NaiveDateTime::new(date, time);
        expect(datetime_value.format_with("Invalid format")).to_be_err();
    }
}
