pub mod osu;
pub mod collection;
pub mod scores;

use std::ops::Range;
use std::cmp::{PartialEq, PartialOrd};

#[derive(Clone)]
pub enum Comparison<T: Clone + PartialEq + PartialOrd> {
    Eq(T),
    Lt(T),
    Gt(T),
    LtE(T),
    GtE(T),
    Ir(Range<T>),
    IrI(Range<T>)
}

#[derive(Clone)]
pub enum AskCompareIgnore<T: Clone + PartialEq + PartialOrd> {
    Ask,
    Compare(Comparison<T>),
    Ignore
}