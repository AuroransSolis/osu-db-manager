use std::cmp::PartialEq;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::{Error as IoError, ErrorKind::InvalidInput, Result as IoResult};
use std::ops::Range;
use std::str::FromStr;

use crate::deserialize_primitives::*;
use crate::read_error::{DbFileParseError, ParseErrorKind::*, ParseFileResult};

// Deserializing osu!.db-specific data types
const RANKED_STATUS_ERR: &str = "Failed to read byte for ranked status.";
const GAMEPLAY_MODE_ERR: &str = "Failed to read byte for gameplay mode specifier.";

macro_rules! primitive {
    ($msg:ident) => {{
        DbFileParseError::new(PrimitiveError, $msg)
    }};
}

#[inline]
pub fn read_int_double_pair(bytes: &[u8], i: &mut usize) -> ParseFileResult<(i32, f64)> {
    let int = read_int(&bytes[*i + 1..*i + 5], &mut 0)?;
    let double = read_double(&bytes[*i + 6..*i + 14], &mut 0)?;
    *i += 14;
    Ok((int, double))
}

pub fn maybe_read_int_double_pair(
    c: bool,
    bytes: &[u8],
    i: &mut usize,
) -> ParseFileResult<Option<(i32, f64)>> {
    if c {
        let int = read_int(&bytes[*i + 1..*i + 5], &mut 0)?;
        let double = read_double(&bytes[*i + 6..*i + 14], &mut 0)?;
        *i += 14;
        Ok(Some((int, double)))
    } else {
        *i += 14;
        Ok(None)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct TimingPoint {
    bpm: f64,
    offset: f64,
    inherited: bool,
}

impl TimingPoint {
    #[inline]
    pub fn read_from_bytes(bytes: &[u8], i: &mut usize) -> ParseFileResult<Self> {
        if *i + 17 < bytes.len() {
            let mut double_buf = [0; 8];
            double_buf.copy_from_slice(&bytes[*i..*i + 8]);
            let bpm = f64::from_bits(u64::from_le_bytes(double_buf));
            double_buf.copy_from_slice(&bytes[*i + 8..*i + 16]);
            let offset = f64::from_bits(u64::from_le_bytes(double_buf));
            let inherited = bytes[*i + 16] != 0;
            *i += 17;
            Ok(TimingPoint {
                bpm,
                offset,
                inherited,
            })
        } else {
            Err(DbFileParseError::new(
                PrimitiveError,
                "Insufficient bytes to read timing point.",
            ))
        }
    }

    pub fn maybe_read_from_bytes(
        c: bool,
        bytes: &[u8],
        i: &mut usize,
    ) -> ParseFileResult<Option<Self>> {
        if c {
            if *i + 17 < bytes.len() {
                let mut double_buf = [0; 8];
                double_buf.copy_from_slice(&bytes[*i..*i + 8]);
                let bpm = f64::from_bits(u64::from_le_bytes(double_buf));
                double_buf.copy_from_slice(&bytes[*i + 8..*i + 16]);
                let offset = f64::from_bits(u64::from_le_bytes(double_buf));
                let inherited = bytes[*i + 16] != 0;
                *i += 17;
                Ok(Some(TimingPoint {
                    bpm,
                    offset,
                    inherited,
                }))
            } else {
                Err(DbFileParseError::new(
                    PrimitiveError,
                    "Insufficient bytes to read timing point.",
                ))
            }
        } else {
            *i += 17;
            Ok(None)
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum RankedStatus {
    Unknown,
    Unsubmitted,
    PendingWIPGraveyard,
    Unused,
    Ranked,
    Approved,
    Qualified,
    Loved,
}

use self::RankedStatus::*;

impl Display for RankedStatus {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(
            f,
            "{}",
            match self {
                Unknown => "Unknown",
                Unsubmitted => "Unsubmitted",
                PendingWIPGraveyard => "Pending/WIP/Graveyard",
                Unused => "Unused",
                Ranked => "Ranked",
                Approved => "Approved",
                Qualified => "Qualified",
                Loved => "Loved",
            }
        )
    }
}

impl FromStr for RankedStatus {
    type Err = IoError;

    fn from_str(s: &str) -> IoResult<Self> {
        match s.to_lowercase().as_str() {
            "unknown" => Ok(Unknown),
            "unsubmitted" => Ok(Unsubmitted),
            "pending" | "wip" | "graveyard" => Ok(PendingWIPGraveyard),
            "unused" => Ok(Unused),
            "ranked" => Ok(Ranked),
            "approved" => Ok(Approved),
            "qualified" => Ok(Qualified),
            "loved" => Ok(Loved),
            _ => {
                let msg = format!(
                    "Invalid ranked status: {}\n\
                     Valid status types:\n \
                     - Unknown\n \
                     - Unsubmitted\n \
                     - Pending\n \
                     - WIP\n \
                     - Graveyard\n \
                     - Ranked\n \
                     - Approved\n \
                     - Qualified\n \
                     - Loved",
                    s
                );
                Err(IoError::new(InvalidInput, msg.as_str()))
            }
        }
    }
}

impl RankedStatus {
    #[inline]
    pub fn read_from_bytes(bytes: &[u8], i: &mut usize) -> ParseFileResult<Self> {
        match read_byte(bytes, i).map_err(|_| primitive!(RANKED_STATUS_ERR))? {
            0 => Ok(Unknown),
            1 => Ok(Unsubmitted),
            2 => Ok(PendingWIPGraveyard),
            3 => Ok(Unused),
            4 => Ok(Ranked),
            5 => Ok(Approved),
            6 => Ok(Qualified),
            7 => Ok(Loved),
            b @ _ => {
                let err_msg = format!("Found invalid ranked status value ({})", b);
                Err(DbFileParseError::new(PrimitiveError, err_msg.as_str()))
            }
        }
    }

    #[inline]
    pub fn maybe_read_from_bytes(
        c: bool,
        bytes: &[u8],
        i: &mut usize,
    ) -> ParseFileResult<Option<Self>> {
        if c {
            match read_byte(bytes, i).map_err(|_| primitive!(RANKED_STATUS_ERR))? {
                0 => Ok(Some(Unknown)),
                1 => Ok(Some(Unsubmitted)),
                2 => Ok(Some(PendingWIPGraveyard)),
                3 => Ok(Some(Unused)),
                4 => Ok(Some(Ranked)),
                5 => Ok(Some(Approved)),
                6 => Ok(Some(Qualified)),
                7 => Ok(Some(Loved)),
                b @ _ => {
                    let err_msg = format!("Found invalid ranked status value ({})", b);
                    Err(DbFileParseError::new(PrimitiveError, err_msg.as_str()))
                }
            }
        } else {
            Ok(None)
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum ByteSingle {
    Byte(u8),
    Single(f32),
}

use self::ByteSingle::*;

impl Display for ByteSingle {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(
            f,
            "{}",
            match self {
                Byte(b) => format!("{}", b),
                Single(s) => format!("{}", s),
            }
        )
    }
}

impl PartialEq for ByteSingle {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Byte(b0), Byte(b1)) => b0 == b1,
            (Byte(b), Single(s)) => s.floor() as u64 == b as u64 || s.ceil() as u64 == b as u64,
            (Single(s), Byte(b)) => s.floor() as u64 == b as u64 || s.ceil() as u64 == b as u64,
            (Single(s0), Single(s1)) => s0 == s1,
        }
    }
}

impl FromStr for ByteSingle {
    type Err = IoError;

    fn from_str(s: &str) -> IoResult<Self> {
        if is_number(s) {
            if s.contains('.') {
                Ok(Single(s.parse::<f32>().map_err(|e| {
                    let msg = format!("Failed to parse input: {}\n{}", s, e);
                    IoError::new(InvalidInput, msg.as_str())
                })?))
            } else {
                Ok(Byte(s.parse::<u8>().map_err(|e| {
                    let msg = format!("Failed to parse input: {}\n{}", s, e);
                    IoError::new(InvalidInput, msg.as_str())
                })?))
            }
        } else {
            let msg = format!("Input is not a number ({})", s);
            Err(IoError::new(InvalidInput, msg.as_str()))
        }
    }
}

impl ByteSingle {
    pub fn is_in_range(&self, range: Range<ByteSingle>) -> bool {
        let Range { start, end } = range;
        match (*self, start, end) {
            (Byte(n), Byte(s), Byte(e)) => n >= s && n < e,
            (Byte(n), Byte(s), Single(e)) => n >= s && (n as f32) < e,
            (Byte(n), Single(s), Byte(e)) => n as f32 >= s && n < e,
            (Byte(n), Single(s), Single(e)) => n as f32 >= s && (n as f32) < e,
            (Single(n), Byte(s), Byte(e)) => n >= s as f32 && n < e as f32,
            (Single(n), Byte(s), Single(e)) => n >= s as f32 && n < e,
            (Single(n), Single(s), Byte(e)) => n >= s && n < e as f32,
            (Single(n), Single(s), Single(e)) => n >= s && n < e,
        }
    }

    fn is_in_range_inclusive(&self, range: Range<ByteSingle>) -> bool {
        let Range { start, end } = range;
        match (*self, start, end) {
            (Byte(n), Byte(s), Byte(e)) => n >= s && n <= e,
            (Byte(n), Byte(s), Single(e)) => n >= s && (n as f32) <= e,
            (Byte(n), Single(s), Byte(e)) => n as f32 >= s && n <= e,
            (Byte(n), Single(s), Single(e)) => n as f32 >= s && (n as f32) <= e,
            (Single(n), Byte(s), Byte(e)) => n >= s as f32 && n <= e as f32,
            (Single(n), Byte(s), Single(e)) => n >= s as f32 && n <= e,
            (Single(n), Single(s), Byte(e)) => n >= s && n <= e as f32,
            (Single(n), Single(s), Single(e)) => n >= s && n <= e,
        }
    }
}

impl From<ByteSingle> for u8 {
    fn from(other: ByteSingle) -> Self {
        match other {
            Byte(byte) => byte,
            Single(single) => single as u8,
        }
    }
}

impl From<ByteSingle> for f32 {
    fn from(other: ByteSingle) -> Self {
        match other {
            Byte(byte) => byte as f32,
            Single(single) => single,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum GameplayMode {
    Standard,
    Taiko,
    Ctb,
    Mania,
}

use self::GameplayMode::*;

impl FromStr for GameplayMode {
    type Err = IoError;

    fn from_str(s: &str) -> IoResult<Self> {
        match s.to_lowercase().as_str() {
            "osu!" | "osu" | "osu!standard" | "standard" => Ok(Standard),
            "osu!taiko" | "taiko" => Ok(Taiko),
            "osu!ctb" | "ctb" | "catch the beat" => Ok(Ctb),
            "osu!mania" | "mania" => Ok(Mania),
            _ => {
                let msg = format!("Unrecognized gameplay mode: {}", s);
                Err(IoError::new(InvalidInput, msg.as_str()))
            }
        }
    }
}

impl GameplayMode {
    #[inline]
    pub fn read_from_bytes(bytes: &[u8], i: &mut usize) -> ParseFileResult<Self> {
        let b = read_byte(bytes, i).map_err(|_| primitive!(GAMEPLAY_MODE_ERR))?;
        match b {
            0 => Ok(Standard),
            1 => Ok(Taiko),
            2 => Ok(Ctb),
            3 => Ok(Mania),
            _ => {
                let err_msg = format!("Read invalid gamemode specifier ({})", b);
                Err(DbFileParseError::new(PrimitiveError, err_msg.as_str()))
            }
        }
    }

    #[inline]
    pub fn maybe_read_from_bytes(
        c: bool,
        bytes: &[u8],
        i: &mut usize,
    ) -> ParseFileResult<Option<Self>> {
        if c {
            let b = read_byte(bytes, i).map_err(|_| primitive!(GAMEPLAY_MODE_ERR))?;
            match b {
                0 => Ok(Some(Standard)),
                1 => Ok(Some(Taiko)),
                2 => Ok(Some(Ctb)),
                3 => Ok(Some(Mania)),
                _ => {
                    let err_msg = format!("Read invalid gamemode specifier ({})", b);
                    Err(DbFileParseError::new(PrimitiveError, err_msg.as_str()))
                }
            }
        } else {
            Ok(None)
        }
    }
}

impl Display for GameplayMode {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(
            f,
            "{}",
            match self {
                Standard => "osu!standard",
                Taiko => "Taiko",
                Ctb => "CTB",
                Mania => "osu!mania",
            }
        )
    }
}
