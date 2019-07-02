use std::io::{Result as IoResult, Error as IoError, ErrorKind::InvalidInput};

use chrono::{NaiveDateTime, NaiveDate};

use crate::load_settings::{LoadSetting, Comparison};

#[macro_export]
macro_rules! assign_if_present {
    ($matches:ident { $($arg_name:literal, $var:ident);+ }) => {
        $(
            let $var = if $matches.is_present()
        )+
    }
}

#[macro_export]
macro_rules! assign_from_value_of {
    ($matches:ident { $($arg_name:literal => $var:ident: $t:ty);+ }) => {
        $(
            assign_from_value_of!($matches | $arg_name => $var: $t);
        )+
    };
    ($matches:ident | $arg_name:literal => $var:ident: String) => {
        if let Some(m) = $matches.value_of($arg_name) {
            LoadSetting::Filter(Comparison::Eq(m.to_string()))
        } else {
            LoadSetting::Ignore
        }
    };
    ($matches:ident | $arg_name:literal => $var:ident: NaiveDate) => {

    }
}

#[macro_export]
macro_rules! get_parse_and_assign {
    ($matches:ident { $($arg_name:literal => $var:ident: $t:ty);+ }) => {
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
    ($matches:ident { $($arg_name:literal => $var:ident);+ }) => {
        $(
            let $var = if let Some(m) = $matches.value_of($arg_name) {
                Some(Comparison::Eq(m.to_string()))
            } else {
                None
            };
        )+
    }
}

#[macro_export]
macro_rules! get_and_assign_datetime {
    ($matches:ident { $($arg_name:literal, $var:ident);+ }) => {
        $(

        )+
    }
}

#[macro_export]
macro_rules! get_and_assign_bool {
    ($matches:ident { $($arg_name:literal, $var:ident);+ }) => {
        $(
            let $var = if let Some(m) = $matches.value_of($arg_name) {
                Some(
                })
            } else {
                None
            };
        )+
    }
}