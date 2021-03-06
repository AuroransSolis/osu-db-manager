use crate::deserialize_primitives::*;
use crate::load_settings::EqualCopy;
use crate::read_error::{DbFileParseError, ParseErrorKind::*, ParseFileResult};
use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::fmt::{self, Display};
use std::io::{Error as IoError, ErrorKind::InvalidInput, Result as IoResult};
use std::str::FromStr;

// Deserializing osu!.db-specific data types
const RANKED_STATUS_ERR: &str = "Failed to read byte for ranked status.";
const GAMEPLAY_MODE_ERR: &str = "Failed to read byte for gameplay mode specifier.";

macro_rules! primitive {
    ($msg:ident) => {{
        DbFileParseError::new(PrimitiveError, $msg)
    }};
}

/// Read an int-double pair from a slice. The integer represents the mods used and the double
/// represents the star rating.
#[inline]
pub fn read_int_double_pair(bytes: &[u8], i: &mut usize) -> ParseFileResult<(i32, f64)> {
    let int = read_int(&bytes[*i + 1..*i + 5], &mut 0)?;
    let double = read_double(&bytes[*i + 6..*i + 14], &mut 0)?;
    *i += 14;
    Ok((int, double))
}

/// Conditionally read an int-double pair from a slice. The integer represents the mods used and the
/// double represents the star rating.
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

/// `TimingPoint`s indicate the BPM of a beatmap at and after a certain offset from the start.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct TimingPoint {
    bpm: f64,
    offset: f64,
    inherited: bool,
}

impl TimingPoint {
    /// Parse a `TimingPoint` from a slice of bytes.
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
}

impl Display for TimingPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "BPM: {} | Offset from start: {} | Inherited: {}",
            self.bpm, self.offset, self.inherited
        )
    }
}

/// Shows the ranking status of a particular beatmap. A beatmap can have a status of any of the
/// following:
/// - Unknown
/// - Unsubmitted
/// - Pending/WIP/Graveyard
/// - Ranked
/// - Approved
/// - Qualified
/// - Loved
/// The enum itself has a variant named `Unused`, though this, as the name suggests, should remain
/// unused, and its appearance would indicate either improper parsing or corruption of the database.
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
        setting: EqualCopy<RankedStatus>,
        skip: &mut bool,
        bytes: &[u8],
        i: &mut usize,
    ) -> ParseFileResult<Option<Self>> {
        if *i < bytes.len() {
            if setting.is_ignore() || *skip {
                *i += 1;
                Ok(None)
            } else {
                let byte = bytes[*i];
                *i += 1;
                let ranked_status = match byte {
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
                }?;
                if setting.compare(ranked_status) {
                    Ok(Some(ranked_status))
                } else {
                    *skip = true;
                    Ok(None)
                }
            }
        } else {
            Err(DbFileParseError::new(
                PrimitiveError,
                "Insufficient bytes to read ranking status.",
            ))
        }
    }
}

/// Some database fields will be a byte or a single depending on the version. Since I don't want to
/// have different structs for each database version, I instead use the `ByteSingle` enum.
#[derive(Copy, Clone, Debug)]
pub enum ByteSingle {
    Byte(u8),
    Single(f32),
}

use self::ByteSingle::*;

impl Display for ByteSingle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
        match (*self, *other) {
            (Byte(b0), Byte(b1)) => b0 == b1,
            (Byte(b), Single(s)) => s.floor() as u64 == b as u64 || s.ceil() as u64 == b as u64,
            (Single(s), Byte(b)) => s.floor() as u64 == b as u64 || s.ceil() as u64 == b as u64,
            (Single(s0), Single(s1)) => s0 == s1,
        }
    }
}

impl PartialOrd for ByteSingle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (&Byte(b0), Byte(b1)) => Some(b0.cmp(b1)),
            (&Byte(b), Single(s)) => (b as f32).partial_cmp(s),
            (Single(s), &Byte(b)) => (b as f32).partial_cmp(s),
            (&Single(s0), Single(s1)) => s0.partial_cmp(s1),
        }
    }
}

impl FromStr for ByteSingle {
    type Err = IoError;

    fn from_str(s: &str) -> IoResult<Self> {
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

/// Fairly self-explanatory - indicates which gameplay mode each beatmap is for.
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
            "osu!ctb" | "ctb" | "catch-the-beat" => Ok(Ctb),
            "osu!mania" | "mania" => Ok(Mania),
            _ => {
                let msg = format!("Unrecognized gameplay mode: {}", s);
                Err(IoError::new(InvalidInput, msg.as_str()))
            }
        }
    }
}

impl GameplayMode {
    /// Parse a `GameplayMode` from a slice of bytes.
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

    /// Conditionally parse a `GameplayMode` from a slice of bytes.
    #[inline]
    pub fn maybe_read_from_bytes(
        setting: EqualCopy<GameplayMode>,
        skip: &mut bool,
        bytes: &[u8],
        i: &mut usize,
    ) -> ParseFileResult<Option<Self>> {
        if *i < bytes.len() {
            if setting.is_ignore() || *skip {
                *i += 1;
                Ok(None)
            } else {
                let byte = bytes[*i];
                *i += 1;
                let gameplay_mode = match byte {
                    0 => Ok(Standard),
                    1 => Ok(Taiko),
                    2 => Ok(Ctb),
                    3 => Ok(Mania),
                    _ => {
                        let err_msg = format!("Read invalid gamemode specifier ({})", byte);
                        Err(DbFileParseError::new(PrimitiveError, err_msg.as_str()))
                    }
                }?;
                if let EqualCopy::Eq(cmp) = setting {
                    if cmp == gameplay_mode {
                        Ok(Some(gameplay_mode))
                    } else {
                        *skip = true;
                        Ok(None)
                    }
                } else {
                    Ok(Some(gameplay_mode))
                }
            }
        } else {
            Err(DbFileParseError::new(
                PrimitiveError,
                "Insufficient bytes to read gameplay mode.",
            ))
        }
    }
}

impl Display for GameplayMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UserPermissions {
    None,
    Normal,
    Moderator,
    Supporter,
    Friend,
    peppy,
    WorldCupStaff,
    Invalid,
}

impl UserPermissions {
    pub(crate) fn new(n: i32) -> Self {
        match n {
            0 => UserPermissions::None,
            1 => UserPermissions::Normal,
            2 => UserPermissions::Moderator,
            4 => UserPermissions::Supporter,
            8 => UserPermissions::Friend,
            16 => UserPermissions::peppy,
            32 => UserPermissions::WorldCupStaff,
            _ => UserPermissions::Invalid,
        }
    }

    #[inline]
    pub fn read_from_bytes(bytes: &[u8], i: &mut usize) -> ParseFileResult<Self> {
        let int = read_int(bytes, i)?;
        Ok(UserPermissions::new(int))
    }
}

impl Display for UserPermissions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                UserPermissions::None => "None",
                UserPermissions::Normal => "Normal",
                UserPermissions::Moderator => "Moderator",
                UserPermissions::Supporter => "Supporter",
                UserPermissions::Friend => "Friend",
                UserPermissions::peppy => "peppy",
                UserPermissions::WorldCupStaff => "World Cup staff",
                UserPermissions::Invalid => "Invalid",
            }
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UnknownShortOrUserPermissions {
    UnknownShort(i16),
    UserPermissions(UserPermissions),
}

impl Display for UnknownShortOrUserPermissions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UnknownShortOrUserPermissions::UnknownShort(n) => write!(f, "{}", n),
            UnknownShortOrUserPermissions::UserPermissions(perms) => write!(f, "{}", perms),
        }
    }
}
