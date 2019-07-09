pub mod osu;
pub mod collection;
pub mod scores;
pub mod query;

use std::ops::Range;
use std::cmp::{PartialEq, PartialOrd};
use std::io::{Result as IoResult, Error as IoError, ErrorKind::InvalidInput};
use std::str::FromStr;

use clap::ArgMatches;
use chrono::NaiveDate;

use crate::databases::osu::primitives::{RankedStatus, ByteSingle, GameplayMode};

pub enum FilterResult<T> {
    Meets(Option<T>),
    Fails
}

pub trait Compare<T> {
    fn compare(&self, other: T) -> bool;
}

#[derive(Clone)]
pub enum LoadSetting<C: Compare> {
    Load,
    Filter(C),
    Ignore
}

impl<T> LoadSetting<T> {
    pub(crate) fn is_ignore(&self) -> bool {
        match self {
            LoadSetting::Ignore => true,
            _ => false
        }
    }

    pub(crate) fn is_load(&self) -> bool {
        match self {
            LoadSetting::Load => true,
            _ => false
        }
    }
}

impl<C: Compare> From<Option<C>> for LoadSetting<C> {
    fn from(other: Option<C>) -> Self {
        if let Some(c) = other {
            LoadSetting::Filter(c)
        } else {
            LoadSetting::Ignore
        }
    }
}

struct Empty {}

impl Compare<Empty> for Empty {
    fn compare(&self, other: Empty) -> bool {
        false
    }
}

#[derive(Copy, Clone)]
pub struct EqualCopy<T: Copy + Clone + PartialEq> {
    value: T
}

impl<T: Copy + Clone + PartialEq> Compare<T> for EqualCopy<T> {
    fn compare(&self, other: T) -> bool {
        *self.value == other
    }
}

impl<T: Copy + Clone + PartialEq + FromStr> EqualCopy<T> {
    pub fn from_matches(matches: &ArgMatches, field: &str)
        -> IoResult<Option<Self>> {
        if let Some(m) = matches.value_of(field) {
            Ok(Some(EqualCopy {
                value: m.parse::<T>()
                    .map_err(|e| IoError::new(InvalidInput, format!("Error parsing value: {}", e)))?
            }))
        } else {
            Ok(None)
        }
    }
}

impl EqualCopy<bool> {
    pub fn bool_from_matches(matches: &ArgMatches, field: &str) -> IoResult<Option<Self>> {
        if let Some(m) = matches.value_of(field) {
            match m.to_lowercase().as_str() {
                "t" | "true" | "y" | "yes" | "1" => {
                    Ok(Some(EqualCopy { value: true }))
                },
                "f" | "false" | "n" | "no" | "0" => {
                    Ok(Some(EqualCopy { value: false }))
                },
                _ => {
                    let msg = format!("Could not parse {} as a boolean. Valid inputs are:\n \
                         - t/true/y/yes/1\n \
                         - f/false/n/no/0");
                    Err(IoError::new(InvalidInput, msg.as_str()))
                }
            }
        } else {
            Ok(None)
        }
    }
}

#[derive(Clone)]
pub struct EqualClone<T: Clone + PartialEq> {
    value: T
}

impl<T: Clone + PartialEq> Compare<T> for EqualClone<T> {
    fn compare(&self, other: T) -> bool {
        self.value.clone() == other
    }
}

impl<T: Clone + PartialEq + From<&str>> EqualClone<T> {
    fn from_matches(matches: &ArgMatches, field: &str) -> IoResult<Option<Self>> {
        if let Some(m) = matches.value_of(field) {
            Some(EqualClone { value: m.into() })
        } else {
            None
        }
    }
}

#[derive(Copy, Clone)]
pub enum Relational<T: Copy + Clone + PartialEq + PartialOrd> {
    Eq(T),
    Lt(T),
    Gt(T),
    LtE(T),
    GtE(T),
    InEE(Range<T>), // in range (a, b)
    InEI(Range<T>), // in range (a, b]
    InIE(Range<T>), // in range [a, b)
    InII(Range<T>) // in range [a, b]
}

impl<T: Copy + Clone + PartialEq + PartialOrd> Compare<T> for Relational<T> {
    fn compare(&self, other: T) -> bool {
        match *self {
            Relational::Eq(eq) => other == eq,
            Relational::Lt(lt) => other < lt,
            Relational::Gt(gt) => other > gt,
            Relational::LtE(lte) => other <= lte,
            Relational::GtE(gte) => other >= gte,
            Relational::InEE(Range { start, end }) => other > range.start && other < range.end,
            Relational::InEI(Range { start, end }) => other > range.start && other <= range.end,
            Relational::InIE(Range { start, end }) => other >= range.start && other < range.end,
            Relational::InII(Range { start, end }) => other >= range.start && other <= range.end
        }
    }
}

use self::Relational::*;

impl<T: Copy + Clone + PartialEq + PartialOrd + FromStr> Relational<T> {
    pub fn inner_to_option(self) -> Self {
        match self {
            Eq(eq) => Eq(Some(eq)),
            Lt(lt) => Lt(Some(lt)),
            Gt(gt) => Gt(Some(gt)),
            LtE(lte) => LtE(Some(lte)),
            GtE(gte) => GtE(Some(gte)),
            InEE(Range { start, end }) => InEE(Some(start..end)),
            InEI(Range { start, end }) => InEI(Some(start..end)),
            InIE(Range { start, end }) => InIE(Some(start..end)),
            InII(Range { start, end }) => InII(Some(start..end))
        }
    }
    
    pub fn from_matches(matches: &ArgMatches, field: &str) -> IoResult<Option<Self>> {
        if let Some(m) = matches.value_of(field) {
            // If it's just "4" or "9.2" or something. Not a range.
            if is_number(m) {
                Ok(Some(Eq(m.parse::<T>().map_err(|e| {
                    let msg = format!("Invalid value: {}\nParse error: {}", m, e);
                    Err(IoError::new(InvalidInput, msg.as_str()))
                })?)))
            } else if is_valid_range(m) {
                let (first, middle) = m.split_at(1);
                let (middle, last) = middle.split_at(middle.len() - 1);
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
                Ok(Some(if start == "" {
                    match (first, last) {
                        ("(", ")") | ("[", ")") => Lt(end),
                        ("(", "]") | ("[", "]") => LtE(end),
                        _ => unreachable!()
                    }
                } else if end == "" {
                    match (first, last) {
                        ("(", ")") | ("(", "]") => Gt(end),
                        ("[", ")") | ("[", "]") => GtE(end),
                        _ => unreachable!()
                    }
                } else {
                    match (first, last) {
                        ("(", ")") => InEE(start..end),
                        ("(", "]") => InEI(start..end),
                        ("[", ")") => InIE(start..end),
                        ("[", "]") => InII(start..end),
                        _ => unreachable!()
                    }
                }))
            } else {
                Err(IoError::new(InvalidInput, "Input not recognized as value or range."))
            }
        } else {
            Ok(None)
        }
    }

    fn date_from_matches(matches: &ArgMatches, field: &str) -> IoResult<Option<Self>> {
        if let Some(m) = matches.value_of(field) {
            if (m.starts_with('(') || m.starts_with('['))
                && (m.ends_with(')') || m.ends_with(']')) {
                let (first, middle) = m.split_at(1);
                let (middle, last) = middle.split_at(middle.len() - 1);
                let mut spliterator = middle.split("..");
                let start = spliterator.next().ok_or_else(|| {
                    IoError::new(InvalidInput, "Missing start of range.")
                })?;
                let end = spliterator.next().ok_or_else(|| {
                    IoError::new(InvalidInput, "Missing end of range.")
                })?;
                if start == "" && end == "" {
                    return Err(IoError::new(InvalidInput,
                        "At least one of the range bounds must be defined."));
                }
                let start = date_from_str(start)?;
                let end = date_from_str(end)?;
                Ok(Some(if start == "" {
                    match (first, last) {
                        ("(", ")") | ("[", ")") => Lt(end),
                        ("(", "]") | ("[", "]") => LtE(end),
                        _ => unreachable!()
                    }
                } else if end == "" {
                    match (first, last) {
                        ("(", ")") | ("(", "]") => Gt(start),
                        ("[", ")") | ("[", "]") => GtE(start),
                        _ => unreachable!()
                    }
                } else {
                    match (first, last) {
                        ("(", ")") => InEE(start..end),
                        ("(", "]") => InEI(start..end),
                        ("[", ")") => InIE(start..end),
                        ("[", "]") => InII(start..end),
                        _ => unreachable!()
                    }
                }))
            } else {
                Ok(Some(Eq(date_from_str(m)?)))
            }
        } else {
            Ok(None)
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

fn date_from_str(s: &str) -> IoResult<NaiveDate> {
    NaiveDate::parse_from_str(s, "%F").map_err(|e| {
        let msg = format!("Failed to parse input ({}) as date (YYYY-MM-DD)", s);
        IoError::new(InvalidInput, msg.as_str())
    })
}