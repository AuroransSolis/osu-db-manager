pub mod osu;
pub mod collection;
pub mod scores;

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

impl<T: Clone + PartialEq + PartialOrd + FromStr> Comparison<T> {
    fn from_str(s: &str) -> IoResult<Self> {
        if !s.contains('(') {
            Eq(s.parse::<T>().map_err(|e| {
                let msg = format!("Invalid number: {}\nParse error: {}", s, e);
                Err(IoError::new(InvalidInput, msg.as_str()))
            })?)
        } else {
            let
        }
    }
}

#[derive(Clone)]
pub enum AskCompareIgnore<T: Clone + PartialEq + PartialOrd> {
    Ask,
    Compare(Comparison<T>),
    Ignore
}