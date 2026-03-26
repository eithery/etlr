use chrono::{Local, NaiveDate};
use phf::{phf_map, Map};
use crate::std::result::Result;
use crate::std::string::Normalize;
use super::errors as err;


#[allow(dead_code)]
pub(crate) const DEFAULT_DATE_FORMAT: &str = "CCYYMMDD";


pub(super) static DATE_FORMATS: Map<&'static str, (&'static str, &'static str)> = phf_map! {
    ":default" => ("%Y%m%d", "%Y%m%dT%H%M%S"),
    "yyyymmdd" => ("%Y%m%d", "%Y%m%dT%H%M%S"),
    "ccyymmdd" => ("%Y%m%d", "%Y%m%dT%H%M%S"),
    "yymmdd" => ("%y%m%d", "%y%m%dT%H%M%S"),
    "ccyymm" => ("%Y%m", "%Y%m"),
    "yyyymm" => ("%Y%m", "%Y%m"),
    "ccyy" => ("%Y", "%Y"),
    "yyyy" => ("%Y", "%Y"),
    "mmdd" => ("%m%d", "%m%dT%H%M%S"),

    ":iso" => ("%Y-%m-%d", "%Y-%m-%dT%H:%M:%S"),
    ":iso_space" => ("%Y-%m-%d", "%Y-%m-%d %H:%M:%S"),
    "ccyy-mm-dd" => ("%Y-%m-%d", "%Y-%m-%dT%H:%M:%S"),
    "yyyy-mm-dd" => ("%Y-%m-%d", "%Y-%m-%dT%H:%M:%S"),
    "yy-mm-dd" => ("%y-%m-%d", "%y-%m-%dT%H:%M:%S"),

    ":us" => ("%m/%d/%Y", "%m/%d/%Y %H:%M:%S"),
    "mm/dd/ccyy" => ("%m/%d/%Y", "%m/%d/%Y %H:%M:%S"),
    "mm/dd/yyyy" => ("%m/%d/%Y", "%m/%d/%Y %H:%M:%S"),
    "mm/dd/yy" => ("%m/%d/%y", "%m/%d/%y %H:%M:%S"),
    "mmddccyy" => ("%m%d%Y", "%m%d%YT%H%M%S"),
    "mmddyyyy" => ("%m%d%Y", "%m%d%YT%H%M%S"),
    "mmddyy" => ("%m%d%y", "%m%d%yT%H%M%S"),
    "mm-dd-ccyy" => ("%m-%d-%Y", "%m-%d-%YT%H:%M:%S"),
    "mm-dd-yyyy" => ("%m-%d-%Y", "%m-%d-%YT%H:%M:%S"),
    "mm-dd-yy" => ("%m-%d-%y", "%m-%d-%yT%H:%M:%S"),

    ":eu" => ("%d/%m/%Y", "%d/%m/%Y %H:%M:%S"),
    "dd/mm/ccyy" => ("%d/%m/%Y", "%d/%m/%Y %H:%M:%S"),
    "dd/mm/yyyy" => ("%d/%m/%Y", "%d/%m/%Y %H:%M:%S"),
    "dd/mm/yy" => ("%d/%m/%y", "%d/%m/%y %H:%M:%S"),

    "dd.mm.ccyy" => ("%d.%m.%Y", "%d.%m.%Y %H:%M:%S"),
    "dd.mm.yyyy" => ("%d.%m.%Y", "%d.%m.%Y %H:%M:%S"),
    "dd.mm.yy" => ("%d.%m.%y", "%d.%m.%y %H:%M:%S")
};


#[allow(dead_code)]
pub(crate) trait Date {
    fn current_date() -> String;

    fn format_current_date(date_format: &str) -> Result<String>;

    fn parse(date_str: &str, date_format: &str) -> Result<Self>
        where Self: Sized;

    fn format_with(&self, date_format: &str) -> Result<String>;
}


impl Date for NaiveDate {
    fn current_date() -> String {
        format_with(&Local::now().date_naive(), DEFAULT_DATE_FORMAT).unwrap()
    }


    fn format_current_date(date_format: &str) -> Result<String> {
        format_with(&Local::now().date_naive(), date_format)
    }


    fn parse(date_str: &str, date_format: &str) -> Result<Self>
        where Self: Sized {
        let date_pattern = lookup_date_format(date_format)?;
        Ok(NaiveDate::parse_from_str(date_str, date_pattern)?)
    }


    fn format_with(&self, date_format: &str) -> Result<String> {
        format_with(self, date_format)
    }
}


#[allow(dead_code)]
fn format_with(date_value: &NaiveDate, date_format: &str) -> Result<String> {
    let format_pattern = lookup_date_format(date_format)?;
    Ok(date_value.format(format_pattern).to_string())
}


#[allow(dead_code)]
fn lookup_date_format(date_format: &str) -> Result<&str> {
    let format_key = date_format.normalize();
    DATE_FORMATS.get(&format_key)
        .map(|&(short, _)| short)
        .ok_or_else(|| err::invalid_date_format(date_format))
}
