#[macro_use] mod macros;
pub mod osu;
pub mod collection;
pub mod scores;
pub mod query;

use std::ops::Range;
use std::cmp::{PartialEq, PartialOrd};
use std::io::{Result as IoResult, Error as IoError, ErrorKind::InvalidInput};
use std::str::FromStr;

#[derive(Clone)]
pub enum Comparison<T: Clone + PartialEq + PartialOrd> {
    Eq(T),
    Lt(T),
    Gt(T),
    LtE(T),
    GtE(T),
    IrEE(Range<T>), // in range (a, b)
    IrEI(Range<T>), // in range (a, b]
    IrIE(Range<T>), // in range [a, b)
    IrII(Range<T>) // in range [a, b]
}

use self::Comparison::*;

impl<T: Clone + PartialEq + PartialOrd + FromStr> Comparison<T> {
    fn from_str(s: &str) -> IoResult<Self> {
        // If it's just "=4" or "=9.2" or something. Not a range.
        if is_number(s) {
            Eq(s.parse::<T>().map_err(|e| {
                let msg = format!("Invalid number: {}\nParse error: {}", s, e);
                Err(IoError::new(InvalidInput, msg.as_str()))
            })?)
        } else if is_valid_range(s) {
            let (first, middle) = s.split_at(1);
            let (middle, last) = middle.split_at(s.len() - 1);
            let mut spliterator = middle.split("..");
            let (start, end) = (spliterator.next().ok_or_else(|| {
                IoError::new(InvalidInput, "Invalid range format.")
            })?, spliterator.next().ok_or_else(|| {
                IoError::new(InvalidInput, "Invalid range format.")
            })?);
            if start == "" && end == "" {
                return Err(IoError::new(InvalidInput,
                    "At least one of the range bounds must be defined."));
            }
            let start = start.parse::<T>().map_err(|e| {
                let msg = format!("Failed to parse start of range.\n{}", e);
                IoError::new(InvalidInput, msg.as_str())
            })?;
            let end = end.parse::<T>().map_err(|e| {
                let msg = format!("Failed to parse end of range.\n{}", e);
                IoError::new(InvalidInput, msg.as_str())
            })?;
            if start == "" {
                Ok(match (first, last) {
                    ("(", ")") | ("[", ")") => Lt(end),
                    ("(", "]") | ("[", "]") => LtE(end),
                    _ => unreachable!()
                })
            } else if end == "" {
                Ok(match (first, last) {
                    ("(", ")") | ("(", "]") => Gt(end),
                    ("[", ")") | ("[", "]") => GtE(end),
                    _ => unreachable!()
                })
            } else {
                Ok(match (first, last) {
                    ("(", ")") => IrEE(start..end),
                    ("(", "]") => IrEI(start..end),
                    ("[", ")") => IrIE(start..end),
                    ("[", "]") => IrII(start..end),
                    _ => unreachable!()
                })
            }
        } else {
            Err(IoError::new(InvalidInput, "Input not recognized as number or range."))
        }
    }
}

pub(crate) fn is_number(s: &str) -> bool {
    let mut period_count = 0;
    for c in s.chars() {
        if c == '.' {
            period_count += 1;
        }
        if period_count > 1 {
            return false;
        }
        if !(c.is_numeric() || c.is_ascii_hexdigit()) {
            return false;
        }
    }
    true
}

pub(crate) fn is_valid_range(s: &str) -> bool {
    if (s.starts_with('(') || s.starts_with('[')) && (s.ends_with(')') || s.ends_with(']')) {
        let (_, middle) = s.split_at(1);
        let (middle, _) = middle.split_at(middle.len() - 1);
        let mut spliterator = middle.split("..");
        let start = if let Some(s) = spliterator.next() {
            s
        } else {
            return false;
        };
        let end = if let Some(s) = spliterator.next() {
            s
        } else {
            return false;
        };
        match (start == "", end == "") {
            (true, true) => false,
            (true, false) => is_number(start),
            (false, true) => is_number(end),
            (false, false) => is_number(start) && is_number(end)
        }
    } else {
        false
    }
}

#[derive(Clone)]
pub enum AskCompareIgnore<T: Clone + PartialEq + PartialOrd> {
    Ask,
    Compare(Comparison<T>),
    Ignore
}