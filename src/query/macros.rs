use std::io::{Result as IoResult, Error as IoError, ErrorKind::InvalidInput};

use chrono::{NaiveDateTime, NaiveDate};

use crate::query::Comparison;

#[macro_export]
macro_rules! get_parse_and_assign {
    ($matches:ident { $($arg_name:literal, $var:ident => $t:ty);+ }) => {
        $(
            let $var = if let Some(m) = $matches.value_of($arg_name) {
                Some(Comparison::from_str(m)?)
            } else {
                None
            };
        )+
    }
}

#[macro_export]
macro_rules! get_and_assign_string {
    ($matches:ident { $($arg_name:literal, $var:ident);+ }) => {
        $(
            let $var = if let Some(m) = $matches.value_of($arg_name) {
                Some(Comparison::Eq(m.to_string()))
            } else {
                None
            };
        )+
    }
}

fn datetime_from_str(s: &str) -> IoResult<NaiveDate> {
    NaiveDate::parse_from_str(s, "%F").map_err(|e| {
        let msg = format!("Failed to parse input ({}) as date (YYYY-MM-DD)", s);
        IoError::new(InvalidInput, msg.as_str())
    })
}

#[macro_export]
macro_rules! get_and_assign_datetime {
    ($matches:ident { $($arg_name:literal, $var:ident);+ }) => {
        $(
            let $var = if let Some(m) = $matches.value_of($arg_name) {
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
                    if start == "" {
                        Some(match (first, last) {
                            ("(", ")") | ("[", ")") => Comparison::Lt(end),
                            ("(", "]") | ("[", "]") => Comparison::LtE(end),
                            _ => unreachable!()
                        })
                    } else if end == "" {
                        Some(match (first, last) {
                            ("(", ")") | ("(", "]") => Comparison::Gt(start),
                            ("[", ")") | ("[", "]") => Comparison::GtE(start),
                            _ => unreachable!()
                        })
                    } else {
                        Some(match (first, last) {
                            ("(", ")") => Comparison::IrEE(start..end),
                            ("(", "]") => Comparison::IrEI(start..end),
                            ("[", ")") => Comparison::IrIE(start..end),
                            ("[", "]") => Comparison::IrII(start..end),
                            _ => unreachable!()
                        })
                    }
                } else {
                    Some(datetime_from_str(m)?)
                }
            } else {
                None
            };
        )+
    }
}

#[macro_export]
macro_rules! get_and_assign_bool {
    ($matches:ident { $($arg_name:literal, $var:ident);+ }) => {
        $(
            let $var = if let Some(m) = $matches.value_of($arg_name) {
                Some(match m.to_lowercase().as_str() {
                    "t" | "true" | "y" | "yes" => Comparison::Eq(true),
                    "f" | "false" | "n" | "no" => Comparison::Eq(false),
                    _ => {
                        let msg = format!("Could not parse {} as a boolean. Valid inputs are:\n \
                         - t/true/y/yes\n \
                         - f/false/n/no");
                        return Err(IoError::new(InvalidInput, msg.as_str()));
                    }
                })
            } else {
                None
            };
        )+
    }
}