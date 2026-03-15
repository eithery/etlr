use chrono::{DateTime, Local};
use phf::{phf_map, Map};
use crate::std::result::Result;
use crate::std::string::Normalize;
use super::errors as err;
use super::clock::{Clock, SystemClock};


static DATE_FORMATS: Map<&'static str, (&'static str, &'static str)> = phf_map! {
    ":default" => ("%Y%m%d", "%Y%m%dT%H%M%S"),
    "yyyymmdd" => ("%Y%m%d", "%Y%m%dT%H%M%S"),
    "ccyymmdd" => ("%Y%m%d", "%Y%m%dT%H%M%S"),
    "yyyymmddthhmmss" => ("%Y%m%d", "%Y%m%dT%H%M%S"),
    "ccyymmddthhmmss" => ("%Y%m%d", "%Y%m%dT%H%M%S"),
    "yymmddthhmmss" => ("%y%m%d", "%y%m%dT%H%M%S"),
    "yymmddthhmmssff" => ("%y%m%d", "%y%m%dT%H%M%S.%f"),
    "yymmdd" => ("%y%m%d", "%y%m%dT%H%M%S"),
    "ccyymm" => ("%Y%m", "%Y%m"),
    "yyyymm" => ("%Y%m", "%Y%m"),
    "ccyy" => ("%Y", "%Y"),
    "yyyy" => ("%Y", "%Y"),
    "mmdd" => ("%m%d", "%m%dT%H%M%S"),

    ":iso" => ("%Y-%m-%d", "%Y-%m-%dT%H:%M:%S"),
    "ccyy-mm-dd" => ("%Y-%m-%d", "%Y-%m-%dT%H:%M:%S"),
    "yyyy-mm-dd" => ("%Y-%m-%d", "%Y-%m-%dT%H:%M:%S"),
    "ccyy-mm-ddthhmmss" => ("%Y-%m-%d", "%Y-%m-%dT%H%M%S"),
    "yyyy-mm-ddthhmmss" => ("%Y-%m-%d", "%Y-%m-%dT%H%M%S"),
    "ccyy-mm-ddThh:mm:ss" => ("%Y-%m-%d", "%Y-%m-%dT%H:%M:%S"),
    "yyyy-mm-ddThh:mm:ss" => ("%Y-%m-%d", "%Y-%m-%dT%H:%M:%S"),

    ":iso_space" => ("%Y-%m-%d", "%Y-%m-%d %H:%M:%S"),
    "ccyy-mm-dd 00:00:00.000" => ("%Y-%m-%d", "%Y-%m-%d %H:%M:%S.%f"),
    "yyyy-mm-dd 00:00:00.000" => ("%Y-%m-%d", "%Y-%m-%d %H:%M:%S.%f"),
    "ccyy-mm-dd hhmmss" => ("%Y-%m-%d", "%Y-%m-%d %H%M%S"),
    "yyyy-mm-dd hhmmss" => ("%Y-%m-%d", "%Y-%m-%d %H%M%S"),

    ":us" => ("%m/%d/%Y", "%m/%d/%Y %H:%M:%S"),
    "mm/dd/ccyy" => ("%m/%d/%Y", "%m/%d/%Y %H:%M:%S"),
    "mm/dd/yyyy" => ("%m/%d/%Y", "%m/%d/%Y %H:%M:%S"),
    "mm/dd/yy" => ("%m/%d/%y", "%m/%d/%y %H:%M:%S"),

    ":eu" => ("%d/%m/%Y", "%d/%m/%Y %H:%M:%S"),
    "dd/mm/ccyy" => ("%d/%m/%Y", "%d/%m/%Y %H:%M:%S"),
    "dd/mm/yyyy" => ("%d/%m/%Y", "%d/%m/%Y %H:%M:%S"),
    "dd/mm/yy" => ("%d/%m/%y", "%d/%m/%y %H:%M:%S"),
};


pub(crate) fn timestamp(date_format: &str, include_time: bool) -> Result<String> {
    timestamp_with::<SystemClock>(date_format, include_time)
}


pub(crate) fn timestamp_with<C: Clock>(date_format: &str, include_time: bool) -> Result<String> {
    format(C::now(), date_format, include_time)
}


fn format(date_value: DateTime<Local>, date_format: &str, include_time: bool) -> Result<String> {
    let (short_format, long_format) = lookup_date_format(date_format)?;
    let pattern = if include_time { *long_format } else { *short_format };
    Ok(date_value.format(pattern).to_string())
}


fn lookup_date_format(date_format: &str) -> Result<&(&str, &str)> {
    let format_key = date_format.normalize();
    DATE_FORMATS
        .get(&format_key)
        .ok_or_else(|| err::invalid_date_format(date_format))
}
