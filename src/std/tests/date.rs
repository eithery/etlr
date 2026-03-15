use chrono::{DateTime, Local, TimeZone};
use rstest::*;
use rxpect::expect;
use rxpect::expectations::{EqualityExpectations, ResultExpectations, ProjectedResultExpectations};
use crate::std::clock::Clock;
use crate::std::date;


struct TestClock;


impl Clock for TestClock {
    fn now() -> DateTime<Local> {
        Local.with_ymd_and_hms(2026, 3, 14, 12, 53, 6).unwrap()
    }
}


mod timestamp_with {
    use super::*;

    #[rstest]
    #[case(":default", "20260314", "20260314T125306")]
    #[case(":iso", "2026-03-14", "2026-03-14T12:53:06")]
    #[case(":iso_space", "2026-03-14", "2026-03-14 12:53:06")]
    #[case(":us", "03/14/2026", "03/14/2026 12:53:06")]
    #[case(":eu", "14/03/2026", "14/03/2026 12:53:06")]
    #[case("CCYYMMDD", "20260314", "20260314T125306")]
    #[case("YYYY-MM-DD", "2026-03-14", "2026-03-14T12:53:06")]
    fn it_crates_timestamp_with_date_format(
        #[case] date_format: &str,
        #[case] expected: &str,
        #[case] expected_with_time: &str
    ) {
        expect(date::timestamp_with::<TestClock>(date_format, false)).to_be_ok_and(|v| v.to_equal(expected));
        expect(date::timestamp_with::<TestClock>(date_format, true)).to_be_ok_and(|v| v.to_equal(expected_with_time));
    }


    #[rstest]
    fn it_returns_errors_for_invalid_format() {
        expect(date::timestamp_with::<TestClock>("invalid format", false)).to_be_err();
    }
}
