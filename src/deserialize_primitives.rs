use std::mem::size_of;
use std::time::Duration;

use chrono::{naive::NaiveDate, Duration as ChronoDuration};

use crate::read_error::{ParseFileResult, DbFileParseError, ParseErrorKind::*};
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
const ULEB128_ERR: &str = "Failed to read byte for ULEB128.";
const SINGLE_ERR: &str = "Failed to read byte for single.";
const DOUBLE_ERR: &str = "Failed to read byte for double.";
const BOOLEAN_ERR: &str = "Failed to read byte for boolean.";
const STRING_ERR: &str = "Failed to read indicator for string.";
const DATETIME_ERR: &str = "Failed to read long for datetime.";
const HASH_ERR: &str = "Read invalid indicator byte for MD5 hash string";
const USERNAME_ERR: &str = "Read invalid incidator byte for username string";

macro_rules! primitive {
    ($msg:ident) => {
        DbFileParseError::new(PrimitiveError, $msg)
    }
}

#[inline]
pub fn read_byte(bytes: &[u8], i: &mut usize) -> ParseFileResult<u8> {
    if *i < bytes.len() {
        let tmp = Ok(bytes[*i]);
        *i += 1;
        tmp
    } else {
        Err(primitive!(BYTE_ERR))
    }
}

#[inline]
pub fn read_short(bytes: &[u8], i: &mut usize) -> ParseFileResult<i16> {
    if *i + 1 < bytes.len() {
        let mut buf = [0; 2];
        buf.copy_from_slice(&bytes[*i..*i + 2]);
        let tmp = Ok(i16::from_le_bytes(buf));
        *i += 2;
        tmp
    } else {
        Err(primitive!(SHORT_ERR))
    }
}

#[inline]
pub fn read_int(bytes: &[u8], i: &mut usize) -> ParseFileResult<i32> {
    if *i + 3 < bytes.len() {
        let mut buf = [0; 4];
        buf.copy_from_slice(&bytes[*i..*i + 4]);
        let tmp = Ok(i32::from_le_bytes(buf));
        *i += 4;
        tmp
    } else {
        Err(primitive!(INT_ERR))
    }
}

#[inline]
pub fn read_long(bytes: &[u8], i: &mut usize) -> ParseFileResult<i64> {
    if *i + 7 < bytes.len() {
        let mut buf = [0; 8];
        buf.copy_from_slice(&bytes[*i..*i + 8]);
        let tmp = Ok(i64::from_le_bytes(buf));
        *i += 8;
        tmp
    } else {
        Err(primitive!(LONG_ERR))
    }
}

#[inline]
pub fn read_uleb128(bytes: &[u8], i: &mut usize) -> ParseFileResult<usize> {
    let mut out = 0;
    let mut found_end = false;
    let mut shift = 0;
    while shift < size_of::<usize>() * 8 {
        let b = *bytes.get(*i).ok_or_else(|| primitive!(ULEB128_ERR))?;
        *i += 1;
        // Handle case when there's less than eight bits left in the usize
        if shift + 8 >= size_of::<usize>() * 8 {
            // If the last byte has a value that fits within the remaining number of bits, add it
            // to our total and break the loop
            if 0b11111111 >> (size_of::<usize>() * 8 - shift) | b
                < (0b10000000 >> size_of::<usize>() * 8 - shift - 1) {
                out += (b as usize) << shift;
                found_end = true;
                break;
            } else {
                let err_msg = format!("While the ULEB128 integer format supports integers \
                    of arbitrary lengths, this program will only handle ULEB128 integers \
                    representing integers up to and including {} bits in length.",
                    size_of::<usize>() * 8);
                return Err(DbFileParseError::new(PrimitiveError, err_msg.as_str()));
            }
        }
        out += (b as usize & 0b01111111) << shift;
        if b & 0b10000000 == 0 {
            found_end = true;
            break;
        }
        shift += 7;
    }
    if found_end {
        Ok(out)
    } else {
        let err_msg = format!("While the ULEB128 integer format supports integers \
            of arbitrary lengths, this program will only handle ULEB128 integers representing \
            integers up to and including {} bits in length.", size_of::<usize>() * 8);
        Err(DbFileParseError::new(PrimitiveError, err_msg.as_str()))
    }
}

#[inline]
pub fn read_uleb128_with_len(bytes: &[u8], i: &mut usize) -> ParseFileResult<(usize, usize)> {
    let mut out = 0;
    let mut found_end = false;
    let mut shift = 0;
    let mut len = 0;
    while shift < size_of::<usize>() * 8 {
        let b = read_byte(bytes, i).map_err(|_| DbFileParseError::new(PrimitiveError,
            "Failed to read byte for ULEB128 integer."))?;
        // Handle case when there's less than eight bits left in the usize
        if shift + 8 >= size_of::<usize>() * 8 {
            // If the last byte has a value that fits within the remaining number of bits, add it
            // to our total and break the loop
            if 0b11111111 >> (size_of::<usize>() * 8 - shift) | b
                < (0b10000000 >> size_of::<usize>() * 8 - shift - 1) {
                out += (b as usize) << shift;
                found_end = true;
                break;
            } else {
                let err_msg = format!("While the ULEB128 integer format supports integers \
                    of arbitrary lengths, this program will only handle ULEB128 integers \
                    representing integers up to and including {} bits in length.",
                    size_of::<usize>() * 8);
                return Err(DbFileParseError::new(PrimitiveError, err_msg.as_str()));
            }
        }
        out += (b as usize & 0b01111111) << shift;
        if b & 0b10000000 == 0 {
            found_end = true;
            break;
        }
        shift += 7;
        len += 1;
    }
    if found_end {
        Ok((out, len))
    } else {
        let err_msg = format!("While the ULEB128 integer format supports integers \
            of arbitrary lengths, this program will only handle ULEB128 integers representing \
            integers up to and including {} bits in length.", size_of::<usize>() * 8);
        Err(DbFileParseError::new(PrimitiveError, err_msg.as_str()))
    }
}

#[inline]
pub fn read_single(bytes: &[u8], i: &mut usize) -> ParseFileResult<f32> {
    Ok(f32::from_bits(read_int(bytes, i).map_err(|_| primitive!(SINGLE_ERR))? as u32))
}

#[inline]
pub fn read_double(bytes: &[u8], i: &mut usize) -> ParseFileResult<f64> {
    Ok(f64::from_bits(read_long(bytes, i).map_err(|_| primitive!(DOUBLE_ERR))? as u64))
}

#[inline]
pub fn read_string_utf8(bytes: &[u8], i: &mut usize, field: &str)
    -> ParseFileResult<Option<String>> {
    if *i < bytes.len() {
        let indicator = bytes[*i];
        *i += 1;
        if indicator == 0x0b {
            let length = read_uleb128(bytes, i)?;
            if *i + length <= bytes.len() {
                let tmp = Ok(
                    Some(String::from_utf8(bytes[*i..*i + length].to_vec()).map_err(|e| {
                        let err_msg = format!("Error reading string for {} ({})", field, e);
                        DbFileParseError::new(PrimitiveError, err_msg.as_str())
                    })?)
                );
                *i += length;
                tmp
            } else {
                Err(DbFileParseError::new(PrimitiveError, "String length goes past end of file."))
            }
        } else if indicator == 0 {
            Ok(None)
        } else {
            let err_msg = format!("Read invalid string indicator ({}, index: {}).", indicator, i);
            Err(DbFileParseError::new(PrimitiveError, err_msg.as_str()))
        }
    } else {
        Err(primitive!(STRING_ERR))
    }
}

#[inline]
pub fn read_string_utf8_with_len(bytes: &[u8], i: &mut usize, field: &str)
    -> ParseFileResult<(usize, Option<String>)> {
    if *i < bytes.len() {
        let indicator = bytes[*i];
        *i += 1;
        if indicator == 0x0b {
            let (length, length_bytes) = read_uleb128_with_len(bytes, i)?;
            if *i + length <= bytes.len() {
                let tmp = Ok((
                    1 + length_bytes + length,
                    Some(String::from_utf8(bytes[*i..*i + length].to_vec()).map_err(|e| {
                        let err_msg = format!("Error reading string for {} ({})", field, e);
                        DbFileParseError::new(PrimitiveError, err_msg.as_str())
                    })?)
                ));
                *i += length;
                tmp
            } else {
                Err(DbFileParseError::new(PrimitiveError, "String length goes past end of file."))
            }
        } else if indicator == 0 {
            Ok((1, None))
        } else {
            let err_msg = format!("Read invalid string indicator ({}).", indicator);
            Err(DbFileParseError::new(PrimitiveError, err_msg.as_str()))
        }
    } else {
        Err(primitive!(STRING_ERR))
    }
}

#[inline]
pub fn read_boolean(bytes: &[u8], i: &mut usize) -> ParseFileResult<bool> {
    Ok(read_byte(bytes, i).map_err(|_| primitive!(BOOLEAN_ERR))? != 0)
}

#[inline]
pub fn read_datetime(bytes: &[u8], i: &mut usize) -> ParseFileResult<NaiveDate> {
    let ticks = read_long(bytes, i).map_err(|_| primitive!(DATETIME_ERR))?;
    let duration_since_epoch = Duration::from_micros(ticks as u64 / 10);
    let chrono_duration = ChronoDuration::from_std(duration_since_epoch).map_err(|e| {
        let msg = format!("Failed to convert std::time::Duration to chrono::Duration\n\
                {}", e);
        DbFileParseError(PrimitiveError, msg)
    })?;
    Ok(NaiveDate::from_ymd(1970, 0, 0) + chrono_duration)
}

#[inline]
pub fn read_md5_hash(bytes: &[u8], i: &mut usize) -> ParseFileResult<String> {
    let indicator = read_byte(bytes, i)?;
    if indicator == 0 {
        Err(DbFileParseError::new(PrimitiveError, "Missing hash! Indicator was 0."))
    } else if indicator == 0x0b {
        if *i + 32 < bytes.len() {
            // first byte will be 32 every time
            let hash_bytes = (bytes[*i + 1..*i + 33]).to_vec();
            *i += 33;
            Ok(String::from_utf8(hash_bytes)
                .map_err(|_| DbFileParseError::new(PrimitiveError, "Error reading MD5 hash."))?)
        } else {
            Err(DbFileParseError::new(PrimitiveError, "Not enough bytes left to read MD5 hash."))
        }
    } else {
        let msg = format!("{}: {}", HASH_ERR, indicator);
        Err(DbFileParseError::new(PrimitiveError, msg.as_str()))
    }
}

#[inline]
pub fn read_player_name(bytes: &[u8], i: &mut usize) -> ParseFileResult<Option<String>> {
    let indicator = read_byte(bytes, i)?;
    if indicator == 0 {
        Ok(None)
    } else if indicator == 0x0b {
        // Usernames are ASCII (1 byte in Unicode too), and so should never need more than a byte
        // for the player name string length. Additionally, from talking with a Tillerino
        // maintainer, I have found that the longest usernames that Tillerino has read are about 20
        // characters.
        let player_name_len = read_byte(bytes, i).map_err(|_| DbFileParseError::new(PrimitiveError,
            "Failed to read player name length."))?;
        if player_name_len & 0b10000000 == 0b10000000 {
            return Err(DbFileParseError::new(PrimitiveError, "Read invalid player name length."));
        }
        if *i + player_name_len as usize <= bytes.len() {
            let tmp = Ok(Some(
                String::from_utf8(bytes[*i..*i + player_name_len as usize].to_vec())
                    .map_err(|_| DbFileParseError::new(PrimitiveError, "Bytes made invalid UTF-8 \
                        string!"))?
            ));
            *i += player_name_len as usize;
            tmp
        } else {
            Err(DbFileParseError::new(PrimitiveError, "Not enough bytes left in buffer for \
                specified string length."))
        }
    } else {
        let msg = format!("{}: {}", USERNAME_ERR, indicator);
        return Err(primitive!(msg));
    }
}

#[inline]
pub fn read_player_name_with_len(bytes: &[u8], i: &mut usize) -> ParseFileResult<(usize, Option<String>)> {
    let indicator = read_byte(bytes, i)?;
    if indicator == 0 {
        Ok((1, None))
    } else if indicator == 0x0b {
        // Usernames are ASCII (1 byte in Unicode too), and so should never need more than a byte
        // for the player name string length. Additionally, from talking with a Tillerino
        // maintainer, I have found that the longest usernames that Tillerino has read are about 20
        // characters.
        let player_name_len = read_byte(bytes, i).map_err(|_| DbFileParseError::new(PrimitiveError,
            "Failed to read player name length."))?;
        if player_name_len & 0b10000000 == 0b10000000 {
            return Err(DbFileParseError::new(PrimitiveError, "Read invalid player name length."));
        }
        if *i + player_name_len as usize <= bytes.len() {
            let tmp = Ok((
                2 + player_name_len as usize,
                Some(
                    String::from_utf8(bytes[*i..*i + player_name_len as usize].to_vec())
                        .map_err(|_| DbFileParseError::new(PrimitiveError, "Bytes made invalid \
                            UTF-8 string!"))?
                )
            ));
            *i += 2 + player_name_len as usize;
            tmp
        } else {
            Err(DbFileParseError::new(PrimitiveError, "Not enough bytes left in buffer for \
                specified string length."))
        }
    } else {
        let msg = format!("{}: {}", USERNAME_ERR, indicator);
        return Err(primitive!(msg));
    }
}
