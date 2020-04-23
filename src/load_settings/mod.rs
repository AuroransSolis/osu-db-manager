pub mod collection;
pub mod osu;
pub mod query;
pub mod scores;

use std::cmp::{PartialEq, PartialOrd};
use std::fmt::Display;
use std::io::{Error as IoError, ErrorKind::InvalidInput, Result as IoResult};
use std::ops::Range;
use std::str::FromStr;

use chrono::NaiveDate;
use clap::ArgMatches;

use crate::databases::osu::primitives::{ByteSingle, GameplayMode, RankedStatus};

// Equality checking struct for `Copy` types
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum EqualCopy<T: Copy + Clone + PartialEq> {
    Eq(T),
    Ignore,
    Load,
}

impl<T: Copy + Clone + PartialEq + FromStr> EqualCopy<T> {
    pub fn from_matches(matches: &ArgMatches, field: &str) -> IoResult<Self>
    where
        <T as FromStr>::Err: Display,
    {
        if let Some(m) = matches.value_of(field) {
            Ok(EqualCopy {
                value: m.parse::<T>().map_err(|e| {
                    IoError::new(InvalidInput, format!("Error parsing value: {}", e))
                })?,
            })
        } else {
            Ok(EqualCopy::Ignore)
        }
    }
}

impl EqualCopy<bool> {
    pub fn bool_from_matches(matches: &ArgMatches, field: &str) -> IoResult<Self> {
        if let Some(m) = matches.value_of(field) {
            match m.to_lowercase().as_str() {
                "t" | "true" | "y" | "yes" | "1" => Ok(EqualCopy::Eq(true)),
                "f" | "false" | "n" | "no" | "0" => Ok(EqualCopy::Eq(false)),
                _ => {
                    let msg = format!(
                        "Could not parse '{}' as a boolean. Valid inputs are:\n \
                         - t/true/y/yes/1\n \
                         - f/false/n/no/0",
                        m
                    );
                    Err(IoError::new(InvalidInput, msg.as_str()))
                }
            }
        } else {
            Ok(EqualCopy::Ignore)
        }
    }
}

impl<T: Copy + Clone + PartialEq> EqualCopy<T> {
    pub fn is_ignore(&self) -> bool {
        match self {
            EqualCopy::Ignore => true,
            _ => false,
        }
    }

    pub fn is_load(&self) -> bool {
        match self {
            EqualCopy::Ignore => false,
            _ => true,
        }
    }

    pub fn compare(&self, other: T) -> bool {
        match *self {
            EqualCopy::Eq(value) => value == other,
            EqualCopy::Ignore => false,
            EqualCopy::Load => true,
        }
    }

    pub fn apply_mask(&mut self, mask: bool) {
        if self.is_ignore() && mask {
            *self = EqualCopy::Load;
        }
    }
}

// Equality checking struct for `Clone` types
#[derive(Clone, Eq, PartialEq)]
pub enum EqualClone<T: Clone + PartialEq> {
    Eq(T),
    Ignore,
    Load,
}

impl EqualClone<String> {
    fn from_matches(matches: &ArgMatches, field: &str) -> IoResult<Self> {
        Ok(if let Some(m) = matches.value_of(field) {
            EqualClone::Eq(m.into())
        } else {
            EqualClone::Ignore
        })
    }
}

impl<T: Clone + PartialEq> EqualClone<T> {
    pub fn is_ignore(&self) -> bool {
        match self {
            EqualClone::Ignore => true,
            _ => false,
        }
    }

    pub fn is_load(&self) -> bool {
        match self {
            EqualClone::Ignore => false,
            _ => true,
        }
    }

    pub fn compare(&self, other: T) -> bool {
        match self {
            EqualClone::Eq(value) => value.clone() == other,
            EqualClone::Ignore => false,
            EqualClone::Load => true,
        }
    }

    pub fn apply_mask(&mut self, mask: bool) {
        if self.is_ignore() && mask {
            *self = EqualClone::Load;
        }
    }
}

impl EqualClone<String> {
    pub fn compare_str(&self, other: &str) -> bool {
        match self {
            EqualClone::Eq(value) => value.as_str() == other,
            EqualClone::Ignore => false,
            EqualClone::Load => true,
        }
    }
}

// Ordered comparisons
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Relational<T: Copy + Clone + PartialEq + PartialOrd> {
    Eq(T),
    Lt(T),
    Gt(T),
    LtE(T),
    GtE(T),
    InEE(T, T), // in range (a, b)
    InEI(T, T), // in range (a, b]
    InIE(T, T), // in range [a, b)
    InII(T, T), // in range [a, b]
    Ignore,
    Load,
}

use self::Relational::*;

impl<T: Copy + Clone + PartialEq + PartialOrd + FromStr> Relational<T>
where
    <T as FromStr>::Err: Display,
{
    pub fn is_ignore(&self) -> bool {
        match self {
            Ignore => true,
            _ => false,
        }
    }

    pub fn is_load(&self) -> bool {
        match self {
            Ignore => false,
            _ => true,
        }
    }

    pub fn compare(&self, other: &T) -> bool {
        match *self {
            Eq(eq) => *other == eq,
            Lt(lt) => *other < lt,
            Gt(gt) => *other > gt,
            LtE(lte) => *other <= lte,
            GtE(gte) => *other >= gte,
            InEE(start, end) => *other > start && *other < end,
            InEI(start, end) => *other > start && *other <= end,
            InIE(start, end) => *other >= start && *other < end,
            InII(start, end) => *other >= start && *other <= end,
            Ignore => false,
            Load => true,
        }
    }

    pub fn apply_mask(&mut self, mask: bool) {
        if self.is_ignore() && mask {
            *self = Load;
        }
    }

    pub fn from_matches(matches: &ArgMatches, field: &str) -> IoResult<Self> {
        if let Some(m) = matches.value_of(field) {
            // If it's just "4" or "9.2" or something. Not a range.
            if is_number(m) {
                Ok(Eq(m.parse::<T>().map_err(|e| {
                    let msg = format!("Invalid value: {}\nParse error: {}", m, e);
                    IoError::new(InvalidInput, msg.as_str())
                })?))
            } else if is_valid_range(m) {
                let (first, middle) = m.split_at(1);
                let (middle, last) = middle.split_at(middle.len() - 1);
                let mut spliterator = middle.split("..");
                let start_str = spliterator
                    .next()
                    .ok_or_else(|| IoError::new(InvalidInput, "Missing start of range."))?;
                let end_str = spliterator
                    .next()
                    .ok_or_else(|| IoError::new(InvalidInput, "Missing end of range."))?;
                if start_str == "" && end_str == "" {
                    return Err(IoError::new(
                        InvalidInput,
                        "At least one of the range bounds must be defined.",
                    ));
                }
                let start = start_str.parse::<T>().map_err(|e| {
                    let msg = format!("Failed to parse start of range.\n{}", e);
                    IoError::new(InvalidInput, msg.as_str())
                })?;
                let end = end_str.parse::<T>().map_err(|e| {
                    let msg = format!("Failed to parse end of range.\n{}", e);
                    IoError::new(InvalidInput, msg.as_str())
                })?;
                Ok(if start_str == "" {
                    match (first, last) {
                        ("(", ")") | ("[", ")") => Lt(end),
                        ("(", "]") | ("[", "]") => LtE(end),
                        _ => unreachable!(),
                    }
                } else if end_str == "" {
                    match (first, last) {
                        ("(", ")") | ("(", "]") => Gt(end),
                        ("[", ")") | ("[", "]") => GtE(end),
                        _ => unreachable!(),
                    }
                } else {
                    match (first, last) {
                        ("(", ")") => InEE(start, end),
                        ("(", "]") => InEI(start, end),
                        ("[", ")") => InIE(start, end),
                        ("[", "]") => InII(start, end),
                        _ => unreachable!(),
                    }
                })
            } else {
                Err(IoError::new(
                    InvalidInput,
                    "Input not recognized as value or range.",
                ))
            }
        } else {
            Ok(Ignore)
        }
    }
}

impl Relational<NaiveDate> {
    fn date_from_matches(matches: &ArgMatches, field: &str) -> IoResult<Self> {
        if let Some(m) = matches.value_of(field) {
            if (m.starts_with('(') || m.starts_with('[')) && (m.ends_with(')') || m.ends_with(']'))
            {
                let (first, middle) = m.split_at(1);
                let (middle, last) = middle.split_at(middle.len() - 1);
                let mut spliterator = middle.split("..");
                let start_str = spliterator
                    .next()
                    .ok_or_else(|| IoError::new(InvalidInput, "Missing start of range."))?;
                let end_str = spliterator
                    .next()
                    .ok_or_else(|| IoError::new(InvalidInput, "Missing end of range."))?;
                if start_str == "" && end_str == "" {
                    return Err(IoError::new(
                        InvalidInput,
                        "At least one of the range bounds must be defined.",
                    ));
                }
                let start = date_from_str(start_str)?;
                let end = date_from_str(end_str)?;
                Ok(if start_str == "" {
                    match (first, last) {
                        ("(", ")") | ("[", ")") => Lt(end),
                        ("(", "]") | ("[", "]") => LtE(end),
                        _ => unreachable!(),
                    }
                } else if end_str == "" {
                    match (first, last) {
                        ("(", ")") | ("(", "]") => Gt(start),
                        ("[", ")") | ("[", "]") => GtE(start),
                        _ => unreachable!(),
                    }
                } else {
                    match (first, last) {
                        ("(", ")") => InEE(start, end),
                        ("(", "]") => InEI(start, end),
                        ("[", ")") => InIE(start, end),
                        ("[", "]") => InII(start, end),
                        _ => unreachable!(),
                    }
                })
            } else {
                Ok(Eq(date_from_str(m)?))
            }
        } else {
            Ok(Ignore)
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
            (false, false) => is_number(start) && is_number(end),
        }
    } else {
        false
    }
}

fn date_from_str(s: &str) -> IoResult<NaiveDate> {
    NaiveDate::parse_from_str(s, "%F").map_err(|e| {
        let msg = format!("Failed to parse input ({}) as date (YYYY-MM-DD)", s);
        IoError::new(InvalidInput, msg.as_str())
    })
}
