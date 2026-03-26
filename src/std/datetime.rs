use chrono::{NaiveDateTime, NaiveDate, Local};
use phf::{phf_map, Map};
use crate::std::result::Result;
use crate::std::string::Normalize;
use super::date::DATE_FORMATS;
use super::errors as err;


#[allow(dead_code)]
pub(crate) const DEFAULT_TIMESTAMP_FORMAT: &str = "CCYYMMDDTHHMMSS";


static DATETIME_FORMATS: Map<&'static str, (&'static str, &'static str)> = phf_map! {
    "yyyymmddthhmmss" => ("%Y%m%dT%H%M%S", "%Y%m%d"),
    "ccyymmddthhmmss" => ("%Y%m%dT%H%M%S", "%Y%m%d"),
    "yymmddthhmmss" => ("%y%m%dT%H%M%S", "%y%m%d"),
    "yyyymmddthhmmssff" => ("%Y%m%dT%H%M%S%f", "%Y%m%d"),
    "ccyymmddthhmmssff" => ("%Y%m%dT%H%M%S%f", "%Y%m%d"),
    "yymmddthhmmssff" => ("%y%m%dT%H%M%S%f", "%y%m%d"),

    "ccyy-mm-ddthh:mm:ss" => ("%Y-%m-%dT%H:%M:%S", "%Y-%m-%d"),
    "yyyy-mm-ddthh:mm:ss" => ("%Y-%m-%dT%H:%M:%S", "%Y-%m-%d"),
    "yy-mm-ddthh:mm:ss" => ("%y-%m-%dT%H:%M:%S", "%y-%m-%d"),
    "ccyy-mm-ddthh:mm:ss.ff" => ("%Y-%m-%dT%H:%M:%S.%f", "%Y-%m-%d"),
    "yyyy-mm-ddthh:mm:ss.ff" => ("%Y-%m-%dT%H:%M:%S.%f", "%Y-%m-%d"),
    "yy-mm-ddthh:mm:ss.ff" => ("%y-%m-%dT%H:%M:%S.%f", "%y-%m-%d"),
    "ccyy-mm-dd-hh.mm.ss" => ("%Y-%m-%d-%H.%M.%S", "%Y-%m-%d"),
    "yyyy-mm-dd-hh.mm.ss" => ("%Y-%m-%d-%H.%M.%S", "%Y-%m-%d"),
    "yy-mm-dd-hh.mm.ss" => ("%y-%m-%d-%H.%M.%S", "%y-%m-%d"),
    "ccyy-mm-ddt00:00:00" => ("%Y-%m-%dT%H:%M:%S", "%Y-%m-%d"),
    "yyyy-mm-ddt00:00:00" => ("%Y-%m-%dT%H:%M:%S", "%Y-%m-%d"),
    "yy-mm-ddt00:00:00" => ("%y-%m-%dT%H:%M:%S", "%y-%m-%d"),
    "ccyy-mm-ddt00:00:00.000" => ("%Y-%m-%dT%H:%M:%S.%f", "%Y-%m-%d"),
    "yyyy-mm-ddt00:00:00.000" => ("%Y-%m-%dT%H:%M:%S.%f", "%Y-%m-%d"),
    "yy-mm-ddt00:00:00.000" => ("%y-%m-%dT%H:%M:%S.%f", "%y-%m-%d"),

    "ccyy-mm-dd hh:mm:ss" => ("%Y-%m-%d %H:%M:%S", "%Y-%m-%d"),
    "yyyy-mm-dd hh:mm:ss" => ("%Y-%m-%d %H:%M:%S", "%Y-%m-%d"),
    "yy-mm-dd hh:mm:ss" => ("%y-%m-%d %H:%M:%S", "%y-%m-%d"),
    "ccyy-mm-dd hh:mm:ss.ff" => ("%Y-%m-%d %H:%M:%S.%f", "%Y-%m-%d"),
    "yyyy-mm-dd hh:mm:ss.ff" => ("%Y-%m-%d %H:%M:%S.%f", "%Y-%m-%d"),
    "yy-mm-dd hh:mm:ss.ff" => ("%y-%m-%d %H:%M:%S.%f", "%y-%m-%d"),
    "ccyy-mm-dd 00:00:00" => ("%Y-%m-%d %H:%M:%S", "%Y-%m-%d"),
    "yyyy-mm-dd 00:00:00" => ("%Y-%m-%d %H:%M:%S", "%Y-%m-%d"),
    "yy-mm-dd 00:00:00" => ("%y-%m-%d %H:%M:%S", "%y-%m-%d"),
    "ccyy-mm-dd 00:00:00.000" => ("%Y-%m-%d %H:%M:%S.%f", "%Y-%m-%d"),
    "yyyy-mm-dd 00:00:00.000" => ("%Y-%m-%d %H:%M:%S.%f", "%Y-%m-%d"),
    "yy-mm-dd 00:00:00.000" => ("%y-%m-%d %H:%M:%S.%f", "%y-%m-%d"),
};


pub(crate) trait DateTime {
    #[allow(dead_code)]
    fn current_timestamp() -> String;

    #[allow(dead_code)]
    fn format_timestamp(date_format: &str) -> Result<String>;

    fn format_timestamp_with(date_format: &str, include_time: bool) -> Result<String>;

    #[allow(dead_code)]
    fn parse(date_str: &str, date_format: &str) -> Result<Self>
        where Self: Sized;

    #[allow(dead_code)]
    fn format_with(&self, date_format: &str) -> Result<String>;
}


impl DateTime for NaiveDateTime {
    fn current_timestamp() -> String {
        format_with(&Local::now().naive_local(), DEFAULT_TIMESTAMP_FORMAT, None).unwrap()
    }


    fn format_timestamp(date_format: &str) -> Result<String> {
        format_with(&Local::now().naive_local(), date_format, None)
    }


    fn format_timestamp_with(date_format: &str, include_time: bool) -> Result<String> {
        format_with(&Local::now().naive_local(), date_format, Some(include_time))
    }


    fn parse(date_str: &str, date_format: &str) -> Result<Self> {
        let (date_pattern, with_time) = lookup_date_format(date_format, None)?;
        let date_value = if with_time {
            NaiveDateTime::parse_from_str(date_str, date_pattern)?
        } else {
            NaiveDate::parse_from_str(date_str, date_pattern)?
                .and_hms_opt(0, 0, 0)
                .unwrap()
        };
        Ok(date_value)
    }


    fn format_with(&self, date_format: &str) -> Result<String> {
        format_with(self, date_format, None)
    }
}


fn format_with(date_value: &NaiveDateTime, date_format: &str, include_time: Option<bool>) -> Result<String> {
    let (format_pattern, _) = lookup_date_format(date_format, include_time)?;
    Ok(date_value.format(format_pattern).to_string())
}


fn lookup_date_format(date_format: &str, include_time: Option<bool>) -> Result<(&str, bool)> {
    let format_key = date_format.normalize();
    DATE_FORMATS.get(&format_key)
        .map(|&(short, long)| if include_time.unwrap_or(false) { (long, true) } else { (short, false) })
        .or_else(|| {
            DATETIME_FORMATS.get(&format_key)
                .map(|&(long, short)| if include_time.unwrap_or(true) { (long, true) } else { (short, false) })
        })
        .ok_or_else(|| err::invalid_date_format(date_format))
}
