pub mod collection;
pub mod osu;
pub mod query;
pub mod scores;

use std::cmp::{PartialEq, PartialOrd};
use std::io::{Error as IoError, ErrorKind::InvalidInput, Result as IoResult};
use std::ops::Range;
use std::str::FromStr;

use chrono::NaiveDate;
use clap::ArgMatches;

use crate::databases::osu::primitives::{ByteSingle, GameplayMode, RankedStatus};

// Comparison trait for use in partial loading.
pub trait Compare<T> {
    fn compare(&self, other: T) -> bool;
}

// Generic struct that will be used to create the query structs for the databases
#[derive(Clone)]
pub enum LoadSetting<C: Compare<C>> {
    Load,
    Filter(C),
    Ignore,
}

impl<C: Compare<C> + Copy> Copy for LoadSetting<C> {}

impl<C: Compare<C>> LoadSetting<C> {
    pub(crate) fn is_ignore(&self) -> bool {
        match self {
            LoadSetting::Ignore => true,
            _ => false,
        }
    }

    pub(crate) fn is_load(&self) -> bool {
        match self {
            LoadSetting::Load => true,
            _ => false,
        }
    }
}

impl<T: Copy + Clone> LoadSetting<EqualCopy<T>> {
    pub(crate) fn apply(&self, other: T) -> Option<T> {
        match self {
            LoadSetting::Load => Some(other),
            LoadSetting::Filter(cmp) => {
                if cmp.compare(other) {
                    Some(other)
                } else {
                    None
                }
            }
            LoadSetting::Ignore => None,
        }
    }
}

impl<T: Copy + Clone> LoadSetting<Relational<T>> {
    pub(crate) fn apply(&self, other: T) -> Option<T> {
        match self {
            LoadSetting::Load => Some(other),
            LoadSetting::Filter(cmp) => {
                if cmp.compare(other) {
                    Some(other)
                } else {
                    None
                }
            }
            LoadSetting::Ignore => None,
        }
    }
}

impl<T: Clone> LoadSetting<EqualClone<T>> {
    pub(crate) fn apply(&self, other: T) -> Option<T> {
        match self {
            LoadSetting::Load => Some(other),
            LoadSetting::Filter(cmp) => {
                if cmp.compare(other.clone()) {
                    Some(other)
                } else {
                    None
                }
            }
            LoadSetting::Ignore => None,
        }
    }
}

impl<C: Compare<C>> From<Option<C>> for LoadSetting<C> {
    fn from(other: Option<C>) -> Self {
        if let Some(c) = other {
            LoadSetting::Filter(c)
        } else {
            LoadSetting::Ignore
        }
    }
}

impl Compare<()> for () {
    // Never load things marked with `Empty` - they aren't necessary, so don't waste time on them
    fn compare(&self, other: ()) -> bool {
        false
    }
}

// Equality checking struct for `Copy` types
#[derive(Copy, Clone)]
pub struct EqualCopy<T: Copy + Clone + PartialEq> {
    value: T,
}

impl<T: Copy + Clone + PartialEq> Compare<T> for EqualCopy<T> {
    fn compare(&self, other: T) -> bool {
        *self.value == other
    }
}

// Also allow comparing `T` with `Option<T>`
impl<T: Copy + Clone + PartialEq> Compare<Option<T>> for EqualCopy<T> {
    fn compare(&self, other: Option<T>) -> bool {
        if let Some(value) = other {
            *self.value == other
        } else {
            false
        }
    }
}

impl<T: Copy + Clone + PartialEq + FromStr> EqualCopy<T> {
    pub fn from_matches(matches: &ArgMatches, field: &str) -> IoResult<Option<Self>> {
        if let Some(m) = matches.value_of(field) {
            Ok(Some(EqualCopy {
                value: m.parse::<T>().map_err(|e| {
                    IoError::new(InvalidInput, format!("Error parsing value: {}", e))
                })?,
            }))
        } else {
            Ok(None)
        }
    }
}

// Enable conversion between `EqualCopy` types, e.g. `EqualCopy<ByteSingle>` to `EqualCopy<u8>`
impl<T, U: From<T>> From<EqualCopy<T>> for EqualCopy<U> {
    fn from(other: EqualCopy<T>) -> Self {
        EqualCopy {
            value: U::from(other.value),
        }
    }
}

impl EqualCopy<bool> {
    pub fn bool_from_matches(matches: &ArgMatches, field: &str) -> IoResult<Option<Self>> {
        if let Some(m) = matches.value_of(field) {
            match m.to_lowercase().as_str() {
                "t" | "true" | "y" | "yes" | "1" => Ok(Some(EqualCopy { value: true })),
                "f" | "false" | "n" | "no" | "0" => Ok(Some(EqualCopy { value: false })),
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
            Ok(None)
        }
    }
}

// Equality checking struct for `Clone` types
#[derive(Clone)]
pub struct EqualClone<T: Clone + PartialEq> {
    value: T,
}

impl<T: Clone + PartialEq> Compare<T> for EqualClone<T> {
    fn compare(&self, other: T) -> bool {
        self.value.clone() == other
    }
}

// Allow comparing `T` with `Option<T>`
impl<T: Clone + PartialEq> Compare<Option<T>> for EqualClone<T> {
    fn compare(&self, other: Option<T>) -> bool {
        if let Some(value) = other {
            self.value.clone() == other
        } else {
            false
        }
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

// Ordered comparisons
#[derive(Copy, Clone)]
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
}

impl<T: Copy + Clone + PartialEq + PartialOrd> Compare<T> for Relational<T> {
    fn compare(&self, other: T) -> bool {
        match *self {
            Relational::Eq(eq) => other == eq,
            Relational::Lt(lt) => other < lt,
            Relational::Gt(gt) => other > gt,
            Relational::LtE(lte) => other <= lte,
            Relational::GtE(gte) => other >= gte,
            Relational::InEE(start, end) => other > start && other < end,
            Relational::InEI(start, end) => other > start && other <= end,
            Relational::InIE(start, end) => other >= start && other < end,
            Relational::InII(start, end) => other >= start && other <= end,
        }
    }
}

use self::Relational::*;

impl<T: Copy + Clone + PartialEq + PartialOrd + FromStr> Relational<T> {
    pub fn inner_to_option(self) -> Relational<Option<T>> {
        match self {
            Eq(eq) => Eq(Some(eq)),
            Lt(lt) => Lt(Some(lt)),
            Gt(gt) => Gt(Some(gt)),
            LtE(lte) => LtE(Some(lte)),
            GtE(gte) => GtE(Some(gte)),
            InEE(start, end) => InEE(Some(start), Some(end)),
            InEI(start, end) => InEI(Some(start), Some(end)),
            InIE(start, end) => InIE(Some(start), Some(end)),
            InII(start, end) => InII(Some(start), Some(end)),
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
                let (start, end) = (
                    spliterator
                        .next()
                        .ok_or_else(|| IoError::new(InvalidInput, "Invalid range format."))?,
                    spliterator
                        .next()
                        .ok_or_else(|| IoError::new(InvalidInput, "Invalid range format."))?,
                );
                if start == "" && end == "" {
                    return Err(IoError::new(
                        InvalidInput,
                        "At least one of the range bounds must be defined.",
                    ));
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
                        _ => unreachable!(),
                    }
                } else if end == "" {
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
                }))
            } else {
                Err(IoError::new(
                    InvalidInput,
                    "Input not recognized as value or range.",
                ))
            }
        } else {
            Ok(None)
        }
    }

    fn date_from_matches(matches: &ArgMatches, field: &str) -> IoResult<Option<Self>> {
        if let Some(m) = matches.value_of(field) {
            if (m.starts_with('(') || m.starts_with('[')) && (m.ends_with(')') || m.ends_with(']'))
            {
                let (first, middle) = m.split_at(1);
                let (middle, last) = middle.split_at(middle.len() - 1);
                let mut spliterator = middle.split("..");
                let start = spliterator
                    .next()
                    .ok_or_else(|| IoError::new(InvalidInput, "Missing start of range."))?;
                let end = spliterator
                    .next()
                    .ok_or_else(|| IoError::new(InvalidInput, "Missing end of range."))?;
                if start == "" && end == "" {
                    return Err(IoError::new(
                        InvalidInput,
                        "At least one of the range bounds must be defined.",
                    ));
                }
                let start = date_from_str(start)?;
                let end = date_from_str(end)?;
                Ok(Some(if start == "" {
                    match (first, last) {
                        ("(", ")") | ("[", ")") => Lt(end),
                        ("(", "]") | ("[", "]") => LtE(end),
                        _ => unreachable!(),
                    }
                } else if end == "" {
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
