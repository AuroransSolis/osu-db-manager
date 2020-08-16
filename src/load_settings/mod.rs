pub mod collection;
pub mod osu;
pub mod scores;

use crate::load_settings::{
    collection::collectiondb_load_settings::CollectionDbLoadSettings,
    osu::osudb_load_settings::OsuDbLoadSettings,
    scores::scoresdb_load_settings::ScoresDbLoadSettings,
};
use std::cmp::{PartialEq, PartialOrd};
use std::default::Default;
use std::fmt::{self, Debug, Display};
use std::str::FromStr;
use structopt::StructOpt;

#[derive(StructOpt)]
pub enum LoadSettings {
    #[structopt(name = "collection-query")]
    CollectionSettings(CollectionDbLoadSettings),
    #[structopt(name = "osu-query")]
    OsuSettings(OsuDbLoadSettings),
    #[structopt(name = "scores-query")]
    ScoresSettings(ScoresDbLoadSettings),
}

// Equality checking struct for `Copy` types
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum EqualCopy<T: Copy + Clone + PartialEq> {
    Eq(T),
    Ignore,
    Load,
}

impl<T: Copy + Clone + PartialEq> Default for EqualCopy<T> {
    fn default() -> Self {
        EqualCopy::Ignore
    }
}

impl<T: Copy + Clone + PartialEq + FromStr> FromStr for EqualCopy<T>
where
    <T as FromStr>::Err: Debug,
{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        if s == "" || s == "ignore" {
            Ok(EqualCopy::Ignore)
        } else {
            Ok(EqualCopy::Eq({
                s.parse::<T>()
                    .map_err(|e| format!("Error parsing value: {:?}", e))?
            }))
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

impl<T: Clone + Copy + Display + PartialEq> Display for EqualCopy<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EqualCopy::Eq(eq) => write!(f, "={}", eq),
            EqualCopy::Ignore => write!(f, "ignore"),
            EqualCopy::Load => write!(f, "load"),
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

impl<T: Clone + PartialEq> Default for EqualClone<T> {
    fn default() -> Self {
        EqualClone::Ignore
    }
}

impl<T: Clone + FromStr + PartialEq> FromStr for EqualClone<T>
where
    <T as FromStr>::Err: Debug,
{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        if s == "" || s == "ignore" {
            Ok(EqualClone::Ignore)
        } else {
            Ok(EqualClone::Eq(
                s.parse::<T>()
                    .map_err(|e| format!("Error parsing value: {:?}", e))?,
            ))
        }
    }
}

impl<T: Clone + PartialEq> EqualClone<T> {
    pub fn is_ignore(&self) -> bool {
        match self {
            EqualClone::Ignore => true,
            _ => false,
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

impl<T: Clone + Display + PartialEq> Display for EqualClone<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EqualClone::Eq(eq) => write!(f, "={}", eq),
            EqualClone::Ignore => write!(f, "ignore"),
            EqualClone::Load => write!(f, "load"),
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

impl<T: Copy + Clone + PartialEq + PartialOrd> Default for Relational<T> {
    fn default() -> Self {
        Relational::Ignore
    }
}

impl<T: Copy + Clone + PartialEq + PartialOrd> Relational<T> {
    pub fn is_ignore(&self) -> bool {
        match self {
            Relational::Ignore => true,
            _ => false,
        }
    }

    pub fn is_load(&self) -> bool {
        match self {
            Relational::Ignore => false,
            _ => true,
        }
    }

    pub fn compare(&self, other: &T) -> bool {
        match *self {
            Relational::Eq(eq) => *other == eq,
            Relational::Lt(lt) => *other < lt,
            Relational::Gt(gt) => *other > gt,
            Relational::LtE(lte) => *other <= lte,
            Relational::GtE(gte) => *other >= gte,
            Relational::InEE(start, end) => *other > start && *other < end,
            Relational::InEI(start, end) => *other > start && *other <= end,
            Relational::InIE(start, end) => *other >= start && *other < end,
            Relational::InII(start, end) => *other >= start && *other <= end,
            Relational::Ignore => false,
            Relational::Load => true,
        }
    }

    pub fn apply_mask(&mut self, mask: bool) {
        if self.is_ignore() && mask {
            *self = Relational::Load;
        }
    }
}

impl<T: Copy + Clone + FromStr + PartialEq + PartialOrd> FromStr for Relational<T>
where
    <T as FromStr>::Err: Debug,
{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        if s == "" || s == "ignore" {
            Ok(Relational::Ignore)
        } else {
            // If it's just "4" or "9.2" or something. Not a range.
            if is_number(s) {
                Ok(Relational::Eq(s.parse::<T>().map_err(|e| {
                    format!("Invalid value: {}\nParse error: {:?}", s, e)
                })?))
            } else if is_valid_range(s) {
                let (first, middle) = s.split_at(1);
                let (middle, last) = middle.split_at(middle.len() - 1);
                let mut spliterator = middle.split("..");
                let start_str = spliterator
                    .next()
                    .ok_or_else(|| "Missing start of range.".to_string())?;
                let end_str = spliterator
                    .next()
                    .ok_or_else(|| "Missing end of range.".to_string())?;
                if start_str == "" && end_str == "" {
                    return Err("At least one of the range bounds must be defined.".into());
                }
                let start = start_str
                    .parse::<T>()
                    .map_err(|e| format!("Failed to parse start of range.\n{:?}", e))?;
                let end = end_str
                    .parse::<T>()
                    .map_err(|e| format!("Failed to parse end of range.\n{:?}", e))?;
                Ok(if start_str == "" {
                    match (first, last) {
                        ("(", ")") | ("[", ")") => Relational::Lt(end),
                        ("(", "]") | ("[", "]") => Relational::LtE(end),
                        _ => unreachable!(),
                    }
                } else if end_str == "" {
                    match (first, last) {
                        ("(", ")") | ("(", "]") => Relational::Gt(end),
                        ("[", ")") | ("[", "]") => Relational::GtE(end),
                        _ => unreachable!(),
                    }
                } else {
                    match (first, last) {
                        ("(", ")") => Relational::InEE(start, end),
                        ("(", "]") => Relational::InEI(start, end),
                        ("[", ")") => Relational::InIE(start, end),
                        ("[", "]") => Relational::InII(start, end),
                        _ => unreachable!(),
                    }
                })
            } else {
                Err("Input not recognized as value or range.".into())
            }
        }
    }
}

impl<T: Clone + Copy + Display + PartialEq + PartialOrd> Display for Relational<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Relational::Eq(eq) => write!(f, "={}", eq),
            Relational::Lt(lt) => write!(f, "(..{})", lt),
            Relational::Gt(gt) => write!(f, "({}..)", gt),
            Relational::LtE(lte) => write!(f, "(..{}]", lte),
            Relational::GtE(gte) => write!(f, "[{}..)", gte),
            Relational::InEE(start, end) => write!(f, "({}..{})", start, end),
            Relational::InEI(start, end) => write!(f, "({}..{}]", start, end),
            Relational::InIE(start, end) => write!(f, "[{}..{})", start, end),
            Relational::InII(start, end) => write!(f, "[{}..{}]", start, end),
            Relational::Ignore => write!(f, "ignore"),
            Relational::Load => write!(f, "load"),
        }
    }
}

pub(crate) fn is_number(s: &str) -> bool {
    let mut period_count = 0;
    for c in s.chars() {
        if c == '.' {
            period_count += 1;
        } else if !(c.is_numeric() || c.is_ascii_hexdigit()) {
            return false;
        }
        if period_count > 1 {
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
