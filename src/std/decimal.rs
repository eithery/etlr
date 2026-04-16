//
// (C) BRACS, 2026
// PIC Decimal (Cobol) module
// Provides utility and conversion methods to process Cobol decimal numbers (PIC format)
//
// Examples:
//
// S9(XX)V9(YY) | S9(09)V9(04)
// 9(XX)V9(YY)  | 9(05)V9(04)
// s9(XX)V99    | 9(05)V99
// 9(XX)V99     | 9(05)V99
// S9(XX)       | S9(12)             // noqa: ERA001
// 9(XX)        | 9(12)              // noqa: ERA001
//
// - Optional leading S means that the last character will be overpunched (encoded digit and sign in one char).
// - 9(XX) where XX means a number of digits of the whole part.
// - V9(YY) means that there will be decimal point (V) and YY number of digits after decimal points (fractional digits).
// - V99 means 2 digits after decimal point.
//
use lazy_regex::{lazy_regex, Lazy};
use phf::{phf_map, Map};
use regex::Regex;
use rust_decimal::{Decimal, prelude::FromPrimitive};
use crate::std::result::Result;
use crate::std::string::Blank;
use super::errors as err;


static PIC_FORMAT_PATTERNS: Lazy<Regex> = lazy_regex!(
    r"(?ix)
    ^(?P<sign>s?)
    9\((?P<whole_digits>\d{1,2})\)
    (v(9\((?P<fractional_digits>\d{1,2})\)
    |(?P<fractional_digits_short>9{1,2})))?$"
);


static SIGNED_DIGITS_MAP: Map<char, (bool, char)> = phf_map! {
    '{' => (false, '0'),
    'A' => (false, '1'),
    'B' => (false, '2'),
    'C' => (false, '3'),
    'D' => (false, '4'),
    'E' => (false, '5'),
    'F' => (false, '6'),
    'G' => (false, '7'),
    'H' => (false, '8'),
    'I' => (false, '9'),
    '}' => (true, '0'),
    'J' => (true, '1'),
    'K' => (true, '2'),
    'L' => (true, '3'),
    'M' => (true, '4'),
    'N' => (true, '5'),
    'O' => (true, '6'),
    'P' => (true, '7'),
    'Q' => (true, '8'),
    'R' => (true, '9')
};


#[allow(dead_code)]
pub(super) fn parse(str_value: &str, pic_format: &str) -> Result<Decimal> {
    if str_value.is_blank() {
        return Err(err::blank_pic_decimal_string());
    }
    parse_format(pic_format)
        .and_then(|(sign, whole, fractional)| parse_value(str_value, sign, whole, fractional))
}


pub(super) fn parse_format(pic_format: &str) -> Result<(bool, usize, usize)> {
    let caps = PIC_FORMAT_PATTERNS.captures(pic_format)
        .ok_or_else(|| err::invalid_cobol_pic_format(pic_format))?;

    let signed = caps.name("sign").map_or(false, |m| !m.as_str().is_empty());
    let whole_digits: usize = caps["whole_digits"].parse().unwrap_or(0);
    let fractional_digits: usize = if let Some(m) = caps.name("fractional_digits") {
        m.as_str().parse().unwrap_or(0)
    } else if let Some(m) = caps.name("fractional_digits_short") {
        m.as_str().len() as usize
    } else {
        0
    };
    Ok((signed, whole_digits, fractional_digits))
}


fn parse_value(str_value: &str, signed: bool, whole_digits: usize, fractional_digits: usize) -> Result<Decimal> {
    if str_value.len() > whole_digits + fractional_digits {
        return Err(err::value_exceeds_number_of_digits(str_value));
    }
    let mut decoded_value = String::with_capacity(str_value.len() + 1);
    if signed {
        let mut chars = str_value.chars();
        let overpunch_char = chars.next_back().unwrap();
        let &(negative, last_digit) = SIGNED_DIGITS_MAP.get(&overpunch_char)
            .ok_or_else(|| err::incorrect_last_char_for_signed_value(overpunch_char))?;
        if negative {
            decoded_value.push('-');
        }
        decoded_value.push_str(chars.as_str());
        decoded_value.push(last_digit);
    }
    let final_value = if signed { decoded_value.as_str() } else { str_value };
    let int_value: i128 = final_value.parse()
        .map_err(|_| err::invalid_cobol_pic_decimal_value(final_value))?;
    let mut decimal_value = Decimal::from_i128(int_value).unwrap();
    if fractional_digits > 0 {
        decimal_value.set_scale(fractional_digits as u32).unwrap();
    }
    Ok(decimal_value)
}
