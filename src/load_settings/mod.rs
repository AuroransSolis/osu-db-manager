pub mod osu;
pub mod collection;
pub mod scores;
pub mod query;

use std::ops::Range;
use std::cmp::{PartialEq, PartialOrd};
use std::io::{Result as IoResult, Error as IoError, ErrorKind::InvalidInput};
use std::str::FromStr;

use clap::ArgMatches;

use crate::databases::osu::primitives::{RankedStatus, ByteSingle, GameplayMode};

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
use chrono::NaiveDate;
use crate::load_settings::LoadSetting::Load;
use crate::masks::mask::DbMask::CollectionMask;

impl<T: Clone + PartialEq + PartialOrd + FromStr> Comparison<T> {
    fn from_str(s: &str) -> IoResult<Self> {
        // If it's just "4" or "9.2" or something. Not a range.
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
pub enum LoadSetting<T: Clone + PartialEq + PartialOrd> {
    Load,
    Filter(Comparison<T>),
    Ignore
}

impl<T> LoadSetting<T> {
    pub(crate) fn is_ignore(&self) -> bool {
        match self {
            LoadSetting::Ignore => true,
            _ => false
        }
    }
}

pub(crate) enum SpecialArgType {
    Optioni32,
    bool,
    NaiveDate,
    String,
    OptionString,
    Optioni16
}

trait IsSpecialArgType {}

impl IsSpecialArgType for Option<i32>{}
impl IsSpecialArgType for bool {}
impl IsSpecialArgType for NaiveDate {}
impl IsSpecialArgType for String {}
impl IsSpecialArgType for Option<String> {}
impl IsSpecialArgType for Option<i16> {}

fn date_from_str(s: &str) -> IoResult<NaiveDate> {
    NaiveDate::parse_from_str(s, "%F").map_err(|e| {
        let msg = format!("Failed to parse input ({}) as date (YYYY-MM-DD)", s);
        IoError::new(InvalidInput, msg.as_str())
    })
}

fn parse_from_arg_special<'a, T: IsSpecialArgType>(matches: &ArgMatches<'a>, field: &str,
    t: SpecialArgType) -> IoResult<LoadSetting<T>> {
    match t {
        SpecialArgType::Optioni32 => Ok(Some(parse_from_arg::<i32>(matches, field)?))z,
        SpecialArgType::bool => {
            match m.to_lowercase().as_str() {
                "t" | "true" | "y" | "yes" | "1" => Ok(LoadSetting::Filter(Comparison::Eq(true))),
                "f" | "false" | "n" | "no" | "0" => Ok(LoadSetting::Filter(Comparison::Eq(false))),
                _ => {
                    let msg = format!("Could not parse {} as a boolean. Valid inputs are:\n \
                         - t/true/y/yes\n \
                         - f/false/n/no");
                    Err(IoError::new(InvalidInput, msg.as_str()))
                }
            }
        },
        SpecialArgType::String => {
            Ok(if let Some(m) = matches.value_of(field) {
                LoadSetting::Filter(Comparison::Eq(m.to_string()))
            } else {
                LoadSetting::Ignore
            })
        },
        SpecialArgType::OptionString => {
            Ok(if let Some(m) = matches.value_of(field) {
                LoadSetting::Filter(Comparison::Eq(Some(m.to_string())))
            } else {
                LoadSetting::Ignore
            })
        },
        SpecialArgType::NaiveDate => {
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
                    let start = datetime_from_str(start)?;
                    let end = datetime_from_str(end)?;
                    Ok(LoadSetting::Filter(if start == "" {
                        match (first, last) {
                            ("(", ")") | ("[", ")") => Comparison::Lt(end),
                            ("(", "]") | ("[", "]") => Comparison::LtE(end),
                            _ => unreachable!()
                        }
                    } else if end == "" {
                        match (first, last) {
                            ("(", ")") | ("(", "]") => Comparison::Gt(start),
                            ("[", ")") | ("[", "]") => Comparison::GtE(start),
                            _ => unreachable!()
                        }
                    } else {
                        match (first, last) {
                            ("(", ")") => Comparison::IrEE(start..end),
                            ("(", "]") => Comparison::IrEI(start..end),
                            ("[", ")") => Comparison::IrIE(start..end),
                            ("[", "]") => Comparison::IrII(start..end),
                            _ => unreachable!()
                        }
                    }))
                } else {
                    Ok(LoadSetting::Filter(Comparison::Eq(datetime_from_str(m)?)))
                }
            } else {
                Ok(LoadSetting::Ignore)
            }
        },
        SpecialArgType::Optioni16 => Ok(Some(parse_from_arg::<i16>(matches, field)?))
    }
}

fn parse_from_arg<'a, T: IsArgType + FromStr>(matches: &ArgMatches<'a>, field: &str)
    -> IoResult<LoadSetting<T>> {
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
            let parse_start = |s| s.parse::<T>().map_err(|e| {
                let msg = format!("Error parsing start of range.\n{}", e);
                IoError::new(InvalidInput, msg.as_str())
            });
            let parse_end = |s| s.parse::<T>().map_err(|e| {
                let msg = format!("Error parsing end of range.\n{}", e);
                IoError::new(InvalidInput, msg.as_str())
            });
            Ok(LoadSetting::Filter(if start == "" {
                let end = parse_end(end)?;
                match (first, last) {
                    ("(", ")") | ("[", ")") => Comparison::Lt(end),
                    ("(", "]") | ("[", "]") => Comparison::LtE(end),
                    _ => unreachable!()
                }
            } else if end == "" {
                let start = parse_start(start)?;
                match (first, last) {
                    ("(", ")") | ("(", "]") => Comparison::Gt(start),
                    ("[", ")") | ("[", "]") => Comparison::GtE(start),
                    _ => unreachable!()
                }
            } else {
                let start = parse_start(start)?;
                let end = parse_end(end)?;
                match (first, last) {
                    ("(", ")") => Comparison::IrEE(start..end),
                    ("(", "]") => Comparison::IrEI(start..end),
                    ("[", ")") => Comparison::IrIE(start..end),
                    ("[", "]") => Comparison::IrII(start..end),
                    _ => unreachable!()
                }
            }))
        } else {
            Ok(LoadSetting::Filter(Comparison::Eq(m.parse::<T>().map_err(|e| {
                let msg = format!("Error parsing value.\n{}", e);
                IoError::new(InvalidInput, msg.as_str())
            })?)))
        }
    } else {
        Ok(LoadSetting::Ignore)
    }
}