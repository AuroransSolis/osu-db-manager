use crate::databases::osu::primitives::ByteSingle;
use crate::deserialize_primitives::*;
use crate::load_settings::{EqualClone, EqualCopy, Relational};
use crate::read_error::{DbFileParseError, ParseErrorKind::*, ParseFileResult};
use chrono::{naive::NaiveDate, Duration as ChronoDuration};
use std::str;
use std::time::Duration;

// Primitive types we need to read from databases:
// Byte
// Short
// Int
// Long
// ULEB128
// Single
// Double
// Boolean
// String
// Datetime

const BYTE_ERR: &str = "Failed to read byte.";
const SHORT_ERR: &str = "Failed to read byte for short.";
const INT_ERR: &str = "Failed to read byte for int.";
const LONG_ERR: &str = "Failed to read byte for long.";
const SINGLE_ERR: &str = "Failed to read byte for single.";
const BOOLEAN_ERR: &str = "Failed to read byte for boolean.";
const STRING_ERR: &str = "Failed to read indicator for string.";
const DATETIME_ERR: &str = "Failed to read long for datetime.";
const HASH_ERR: &str = "Read invalid indicator byte for MD5 hash string";

macro_rules! primitive {
    ($msg:ident) => {
        DbFileParseError::new(PrimitiveError, $msg)
    };
}

#[inline]
pub fn maybe_read_byte(
    s: Relational<u8>,
    skip: &mut bool,
    bytes: &[u8],
    i: &mut usize,
) -> ParseFileResult<Option<u8>> {
    if *i < bytes.len() {
        if *skip || s.is_ignore() {
            *i += 1;
            Ok(None)
        } else {
            let tmp = bytes[*i];
            *i += 1;
            if s.compare(&tmp) {
                Ok(Some(tmp))
            } else {
                *skip = true;
                Ok(None)
            }
        }
    } else {
        Err(primitive!(BYTE_ERR))
    }
}

#[inline]
pub fn maybe_read_byte_bs(
    s: Relational<ByteSingle>,
    skip: &mut bool,
    bytes: &[u8],
    i: &mut usize,
) -> ParseFileResult<Option<u8>> {
    if *i < bytes.len() {
        if *skip || s.is_ignore() {
            *i += 1;
            Ok(None)
        } else {
            let tmp = bytes[*i];
            *i += 1;
            if s.compare(&ByteSingle::Byte(tmp)) {
                Ok(Some(tmp))
            } else {
                *skip = true;
                Ok(None)
            }
        }
    } else {
        Err(primitive!(BYTE_ERR))
    }
}

#[inline]
pub fn maybe_read_short(
    s: Relational<i16>,
    skip: &mut bool,
    bytes: &[u8],
    i: &mut usize,
) -> ParseFileResult<Option<i16>> {
    if *i + 1 < bytes.len() {
        if *skip || s.is_ignore() {
            *i += 2;
            Ok(None)
        } else {
            let mut buf = [0; 2];
            buf.copy_from_slice(&bytes[*i..*i + 2]);
            let tmp = i16::from_le_bytes(buf);
            *i += 2;
            if s.compare(&tmp) {
                Ok(Some(tmp))
            } else {
                *skip = true;
                Ok(None)
            }
        }
    } else {
        Err(primitive!(SHORT_ERR))
    }
}

#[inline]
pub fn maybe_read_short_nocomp(
    s: bool,
    skip: &mut bool,
    bytes: &[u8],
    i: &mut usize,
) -> ParseFileResult<Option<i16>> {
    if *i + 1 < bytes.len() {
        if *skip || !s {
            *i += 2;
            Ok(None)
        } else {
            let mut buf = [0; 2];
            buf.copy_from_slice(&bytes[*i..*i + 2]);
            let tmp = i16::from_le_bytes(buf);
            *i += 2;
            Ok(Some(tmp))
        }
    } else {
        Err(primitive!(SHORT_ERR))
    }
}

#[inline]
pub fn maybe_read_int(
    s: Relational<i32>,
    skip: &mut bool,
    bytes: &[u8],
    i: &mut usize,
) -> ParseFileResult<Option<i32>> {
    if *i + 3 < bytes.len() {
        if *skip || s.is_ignore() {
            *i += 4;
            Ok(None)
        } else {
            let mut buf = [0; 4];
            buf.copy_from_slice(&bytes[*i..*i + 4]);
            let tmp = i32::from_le_bytes(buf);
            *i += 4;
            if s.compare(&tmp) {
                Ok(Some(tmp))
            } else {
                *skip = true;
                Ok(None)
            }
        }
    } else {
        Err(primitive!(INT_ERR))
    }
}

#[inline]
pub fn maybe_read_int_nocomp(
    s: bool,
    skip: &mut bool,
    bytes: &[u8],
    i: &mut usize,
) -> ParseFileResult<Option<i32>> {
    if *i + 3 < bytes.len() {
        if *skip || !s {
            *i += 4;
            Ok(None)
        } else {
            let mut buf = [0; 4];
            buf.copy_from_slice(&bytes[*i..*i + 4]);
            let tmp = Ok(Some(i32::from_le_bytes(buf)));
            *i += 4;
            tmp
        }
    } else {
        Err(primitive!(INT_ERR))
    }
}

#[inline]
pub fn maybe_read_long(
    s: Relational<i64>,
    skip: &mut bool,
    bytes: &[u8],
    i: &mut usize,
) -> ParseFileResult<Option<i64>> {
    if *i + 7 < bytes.len() {
        if *skip || s.is_ignore() {
            *i += 8;
            Ok(None)
        } else {
            let mut buf = [0; 8];
            buf.copy_from_slice(&bytes[*i..*i + 8]);
            let tmp = i64::from_le_bytes(buf);
            *i += 8;
            if s.compare(&tmp) {
                Ok(Some(tmp))
            } else {
                *skip = true;
                Ok(None)
            }
        }
    } else {
        Err(primitive!(LONG_ERR))
    }
}

#[inline]
pub fn maybe_read_single(
    s: Relational<f32>,
    skip: &mut bool,
    bytes: &[u8],
    i: &mut usize,
) -> ParseFileResult<Option<f32>> {
    if *i + 4 < bytes.len() {
        if *skip || s.is_ignore() {
            *i += 4;
            Ok(None)
        } else {
            let mut buf = [0; 4];
            buf.copy_from_slice(&bytes[*i..*i + 4]);
            let tmp = u32::from_le_bytes(buf);
            let tmp = f32::from_bits(tmp);
            *i += 4;
            if s.compare(&tmp) {
                Ok(Some(tmp))
            } else {
                *skip = true;
                Ok(None)
            }
        }
    } else {
        Err(primitive!(SINGLE_ERR))
    }
}

#[inline]
pub fn maybe_read_single_bs(
    s: Relational<ByteSingle>,
    skip: &mut bool,
    bytes: &[u8],
    i: &mut usize,
) -> ParseFileResult<Option<f32>> {
    if *i + 4 < bytes.len() {
        if *skip || s.is_ignore() {
            *i += 4;
            Ok(None)
        } else {
            let mut buf = [0; 4];
            buf.copy_from_slice(&bytes[*i..*i + 4]);
            let tmp = u32::from_le_bytes(buf);
            let tmp = f32::from_bits(tmp);
            *i += 4;
            if s.compare(&ByteSingle::Single(tmp)) {
                Ok(Some(tmp))
            } else {
                *skip = true;
                Ok(None)
            }
        }
    } else {
        Err(primitive!(SINGLE_ERR))
    }
}

#[inline]
pub fn maybe_read_double(
    s: Relational<f64>,
    skip: &mut bool,
    bytes: &[u8],
    i: &mut usize,
) -> ParseFileResult<Option<f64>> {
    if *i + 8 < bytes.len() {
        if *skip || s.is_ignore() {
            *i += 8;
            Ok(None)
        } else {
            let mut buf = [0; 8];
            buf.copy_from_slice(&bytes[*i..*i + 8]);
            let tmp = u64::from_le_bytes(buf);
            let tmp = f64::from_bits(tmp);
            *i += 8;
            if s.compare(&tmp) {
                Ok(Some(tmp))
            } else {
                *skip = true;
                Ok(None)
            }
        }
    } else {
        Err(primitive!(SINGLE_ERR))
    }
}

#[inline]
pub fn maybe_read_str_utf8<'a>(
    s: &EqualClone<String>,
    skip: &mut bool,
    bytes: &'a [u8],
    i: &mut usize,
    field: &str,
) -> ParseFileResult<Option<&'a str>> {
    if *i < bytes.len() {
        let indicator = bytes[*i];
        *i += 1;
        if indicator == 0x0b {
            let length = read_uleb128(bytes, i)?;
            if *i + length <= bytes.len() {
                if *skip || s.is_ignore() {
                    *i += length;
                    Ok(None)
                } else {
                    let tmp = str::from_utf8(&bytes[*i..*i + length]).map_err(|e| {
                        let err_msg = format!("Error reading string for {} ({})", field, e);
                        DbFileParseError::new(PrimitiveError, err_msg.as_str())
                    })?;
                    *i += length;
                    if s.compare_str(tmp) {
                        Ok(Some(tmp))
                    } else {
                        *skip = true;
                        Ok(None)
                    }
                }
            } else {
                Err(DbFileParseError::new(
                    PrimitiveError,
                    "String length goes past end of file.",
                ))
            }
        } else if indicator == 0 {
            Ok(None)
        } else {
            let err_msg = format!(
                "Read invalid string indicator ({}, index: {}).",
                indicator, i
            );
            Err(DbFileParseError::new(PrimitiveError, err_msg.as_str()))
        }
    } else {
        Err(primitive!(STRING_ERR))
    }
}

#[inline]
pub fn maybe_read_str_utf8_nocomp<'a>(
    s: bool,
    skip: &mut bool,
    bytes: &'a [u8],
    i: &mut usize,
    field: &str,
) -> ParseFileResult<Option<&'a str>> {
    if *i < bytes.len() {
        let indicator = bytes[*i];
        *i += 1;
        if indicator == 0x0b {
            let length = read_uleb128(bytes, i)?;
            if *i + length <= bytes.len() {
                if *skip || !s {
                    *i += length;
                    Ok(None)
                } else {
                    let tmp = str::from_utf8(&bytes[*i..*i + length]).map_err(|e| {
                        let err_msg = format!("Error reading string for {} ({})", field, e);
                        DbFileParseError::new(PrimitiveError, err_msg.as_str())
                    })?;
                    *i += length;
                    Ok(Some(tmp))
                }
            } else {
                Err(DbFileParseError::new(
                    PrimitiveError,
                    "String length goes past end of file.",
                ))
            }
        } else if indicator == 0 {
            Ok(None)
        } else {
            let err_msg = format!(
                "Read invalid string indicator ({}, index: {}).",
                indicator, i
            );
            Err(DbFileParseError::new(PrimitiveError, err_msg.as_str()))
        }
    } else {
        Err(primitive!(STRING_ERR))
    }
}

#[inline]
pub fn maybe_read_boolean(
    s: EqualCopy<bool>,
    skip: &mut bool,
    bytes: &[u8],
    i: &mut usize,
) -> ParseFileResult<Option<bool>> {
    if *i < bytes.len() {
        if *skip || s.is_ignore() {
            *i += 1;
            Ok(None)
        } else {
            let tmp = bytes[*i] != 0;
            *i += 1;
            if s.compare(tmp) {
                Ok(Some(tmp))
            } else {
                *skip = true;
                Ok(None)
            }
        }
    } else {
        Err(primitive!(BOOLEAN_ERR))
    }
}

#[inline]
pub fn maybe_read_boolean_nocomp(
    s: bool,
    skip: &mut bool,
    bytes: &[u8],
    i: &mut usize,
) -> ParseFileResult<Option<bool>> {
    if *i < bytes.len() {
        if *skip || !s {
            *i += 1;
            Ok(None)
        } else {
            let tmp = Ok(Some(bytes[*i] != 0));
            *i += 1;
            tmp
        }
    } else {
        Err(primitive!(BOOLEAN_ERR))
    }
}

#[inline]
pub fn maybe_read_datetime(
    s: Relational<NaiveDate>,
    skip: &mut bool,
    bytes: &[u8],
    i: &mut usize,
) -> ParseFileResult<Option<NaiveDate>> {
    if *i + 7 < bytes.len() {
        if *skip || s.is_ignore() {
            *i += 8;
            Ok(None)
        } else {
            let mut buf = [0; 8];
            buf.copy_from_slice(&bytes[*i..*i + 8]);
            let ticks = u64::from_le_bytes(buf);
            *i += 8;
            let duration_since_epoch = Duration::from_micros(ticks / 10);
            let chrono_duration = ChronoDuration::from_std(duration_since_epoch).map_err(|e| {
                let msg = format!(
                    "Failed to convert std::time::Duration to chrono::Duration\n\
                     {}",
                    e
                );
                DbFileParseError::new(PrimitiveError, msg)
            })?;
            let naive_date = NaiveDate::from_ymd(1970, 0, 0) + chrono_duration;
            if s.compare(&naive_date) {
                Ok(Some(naive_date))
            } else {
                *skip = true;
                Ok(None)
            }
        }
    } else {
        Err(primitive!(DATETIME_ERR))
    }
}

#[inline]
pub fn maybe_read_datetime_nocomp(
    s: bool,
    skip: &mut bool,
    bytes: &[u8],
    i: &mut usize,
) -> ParseFileResult<Option<NaiveDate>> {
    if *i + 7 < bytes.len() {
        if *skip || !s {
            *i += 8;
            Ok(None)
        } else {
            let mut buf = [0; 8];
            buf.copy_from_slice(&bytes[*i..*i + 8]);
            let ticks = u64::from_le_bytes(buf);
            *i += 8;
            let duration_since_epoch = Duration::from_micros(ticks / 10);
            let chrono_duration = ChronoDuration::from_std(duration_since_epoch).map_err(|e| {
                let msg = format!(
                    "Failed to convert std::time::Duration to chrono::Duration\n\
                     {}",
                    e
                );
                DbFileParseError::new(PrimitiveError, msg)
            })?;
            let naive_date = NaiveDate::from_ymd(1970, 0, 0) + chrono_duration;
            Ok(Some(naive_date))
        }
    } else {
        Err(primitive!(DATETIME_ERR))
    }
}

#[inline]
pub fn maybe_read_md5_hash<'a>(
    s: &EqualClone<String>,
    skip: &mut bool,
    bytes: &'a [u8],
    i: &mut usize,
) -> ParseFileResult<Option<&'a str>> {
    if *i < bytes.len() {
        let indicator = bytes[*i];
        *i += 1;
        if indicator == 0 {
            if s.compare(String::with_capacity(0)) {
                Ok(Some(""))
            } else {
                Ok(None)
            }
        } else if indicator == 0x0b {
            if *i + 32 < bytes.len() {
                if *skip || s.is_ignore() {
                    *i += 33;
                    Ok(None)
                } else {
                    // first byte will be 32 every time if indicator == 0x0b
                    let tmp = str::from_utf8(&bytes[*i + 1..*i + 33]).map_err(|_| {
                        DbFileParseError::new(PrimitiveError, "Error reading MD5 hash.")
                    })?;
                    *i += 33;
                    if s.compare_str(tmp) {
                        Ok(Some(tmp))
                    } else {
                        *skip = true;
                        Ok(None)
                    }
                }
            } else {
                Err(DbFileParseError::new(
                    PrimitiveError,
                    "Not enough bytes left to read MD5 \
                     hash.",
                ))
            }
        } else {
            Err(DbFileParseError::new(
                PrimitiveError,
                format!("{}: {}", HASH_ERR, indicator),
            ))
        }
    } else {
        Err(DbFileParseError::new(
            PrimitiveError,
            "Could not read hash indicator byte.",
        ))
    }
}

#[inline]
pub fn maybe_read_player_name_nocomp<'a>(
    s: bool,
    skip: &mut bool,
    bytes: &'a [u8],
    i: &mut usize,
) -> ParseFileResult<Option<&'a str>> {
    if *i < bytes.len() {
        let indicator = bytes[*i];
        *i += 1;
        if indicator == 0 {
            Ok(None)
        } else if indicator == 0x0b {
            if *i < bytes.len() {
                // Usernames are ASCII (1 byte in Unicode too), and so should never need more
                // than a byte for the player name string length. Additionally, from talking
                // with a Tillerino maintainer, I have found that the longest usernames that
                // Tillerino has read are about 20 characters. I also limit the username length
                // to 64 characters and return an error if it's longer.
                let length = bytes[*i] as usize;
                *i += 1;
                if length & 11000000 != 0 {
                    return Err(DbFileParseError::new(
                        PrimitiveError,
                        "Read invalid player name length",
                    ));
                }
                if *i + length < bytes.len() {
                    if *skip || s {
                        *i += length;
                        Ok(None)
                    } else {
                        let tmp = str::from_utf8(&bytes[*i..*i + length]).map_err(|e| {
                            DbFileParseError::new(
                                PrimitiveError,
                                format!("Failed to parse bytes into string:\n{}", e),
                            )
                        })?;
                        *i += length;
                        Ok(Some(tmp))
                    }
                } else {
                    Err(DbFileParseError::new(
                        PrimitiveError,
                        "Not enough bytes left to read player name.",
                    ))
                }
            } else {
                Err(DbFileParseError::new(
                    PrimitiveError,
                    "Could not read player name length byte.",
                ))
            }
        } else {
            Err(DbFileParseError::new(
                PrimitiveError,
                "Read invalid indicator for player name string.",
            ))
        }
    } else {
        Err(DbFileParseError::new(
            PrimitiveError,
            "Could not read indicator for player name string.",
        ))
    }
}
