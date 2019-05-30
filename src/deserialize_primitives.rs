use byteorder::{ReadBytesExt, LittleEndian};
use std::iter::Iterator;
use std::fs::File;
use std::io::{Result as IoResult, Error as IoError};
use std::io::ErrorKind::{Other, InvalidData};
use std::mem::size_of;
use std::string::FromUtf8Error;
use std::time::{Duration, SystemTime};
use std::iter::FromIterator;
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

macro_rules! other {
    ($msg:ident) => {
        IoError::new(Other, $msg)
    }
}

macro_rules! invalid {
    ($msg:expr) => {
        IoError::new(InvalidData, $msg)
    }
}

#[inline]
pub fn read_byte<I: Iterator<Item= u8>>(i: &mut I) -> IoResult<u8> {
    Ok(i.next().ok_or(other!(BYTE_ERR))?)
}

#[inline]
pub fn read_short<I: Iterator<Item = u8>>(i: &mut I) -> IoResult<i16> {
    Ok(i.next().ok_or(other!(SHORT_ERR))? as i16 + ((i.next().ok_or(other!(SHORT_ERR))? as i16) << 8))
}

#[inline]
pub fn read_int<I: Iterator<Item = u8>>(i: &mut I) -> IoResult<i32> {
    Ok(i.next().ok_or(other!(INT_ERR))? as i32 + ((i.next().ok_or(other!(INT_ERR))? as i32) << 8)
        + ((i.next().ok_or(other!(INT_ERR))? as i32) << 16) + ((i.next().ok_or(other!(INT_ERR))? as i32) << 24))
}

#[inline]
pub fn read_long<I: Iterator<Item = u8>>(i: &mut I) -> IoResult<i64> {
    Ok(i.next().ok_or(other!(LONG_ERR))? as i64 + ((i.next().ok_or(other!(LONG_ERR))? as i64) << 8)
        + ((i.next().ok_or(other!(LONG_ERR))? as i64) << 16) + ((i.next().ok_or(other!(LONG_ERR))? as i64) << 24)
        + ((i.next().ok_or(other!(LONG_ERR))? as i64) << 32) + ((i.next().ok_or(other!(LONG_ERR))? as i64) << 40)
        + ((i.next().ok_or(other!(LONG_ERR))? as i64) << 48) + ((i.next().ok_or(other!(LONG_ERR))? as i64) << 56))
}

#[inline]
pub fn read_uleb128<I: Iterator<Item = u8>>(i: &mut I) -> IoResult<usize> {
    let mut out = 0;
    let mut found_end = false;
    let mut shift = 0;
    while shift < size_of::<usize>() * 8 {
        let b = i.next().ok_or(other!(ULEB128_ERR))?;
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
                return Err(IoError::new(InvalidData, err_msg.as_str()));
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
        Err(IoError::new(InvalidData, err_msg.as_str()))
    }
}

#[inline]
pub fn read_uleb128_with_len<I: Iterator<Item = u8>>(i: &mut I) -> IoResult<(usize, usize)> {
    let mut out = 0;
    let mut found_end = false;
    let mut shift = 0;
    let mut len = 0;
    while shift < size_of::<usize>() * 8 {
        let b = i.next().ok_or(other!(ULEB128_ERR))?;
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
                return Err(IoError::new(InvalidData, err_msg.as_str()));
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
        Err(IoError::new(InvalidData, err_msg.as_str()))
    }
}

#[inline]
pub fn read_single<I: Iterator<Item = u8>>(i: &mut I) -> IoResult<f32> {
    Ok(f32::from_bits(read_int(i).map_err(|_| other!(SINGLE_ERR))? as u32))
}

#[inline]
pub fn read_double<I: Iterator<Item = u8>>(i: &mut I) -> IoResult<f64> {
    Ok(f64::from_bits(read_long(i).map_err(|_| other!(DOUBLE_ERR))? as u64))
}

#[inline]
pub fn read_string_utf8<I: Iterator<Item = u8>>(i: &mut I)
    -> IoResult<Result<String, FromUtf8Error>> {
    let indicator = i.next().ok_or(other!(STRING_ERR))?;
    if indicator == 0 {
        Ok(Ok(String::new()))
    } else if indicator == 0x0b {
        let length = read_uleb128(i)?;
        let mut chars = Vec::with_capacity(length);
        for _ in 0..length {
            chars.push(read_byte(i)? as char);
        }
        Ok(Ok(String::from_iter(chars.into_iter())))
    } else {
        let err_msg = format!("Found invalid indicator for string ({})", indicator);
        Err(IoError::new(InvalidData, err_msg.as_str()))
    }
}

#[inline]
pub fn read_boolean<I: Iterator<Item = u8>>(i: &mut I) -> IoResult<bool> {
    Ok(i.next().ok_or(other!(BOOLEAN_ERR))? == 0)
}

#[inline]
pub fn fromutf8_to_ioresult(r: Result<String, FromUtf8Error>, field: &str) -> IoResult<String> {
    match r {
        Ok(string) => Ok(string),
        Err(e) => {
            let err_msg = format!("Error reading {} ({})", field, e);
            Err(IoError::new(InvalidData, err_msg.as_str()))
        }
    }
}

#[inline]
pub fn read_datetime<I: Iterator<Item = u8>>(i: &mut I) -> IoResult<SystemTime> {
    let ticks = read_long(i).map_err(|_| other!(DATETIME_ERR))?;
    let duration_since_epoch = Duration::from_micros(ticks as u64 / 10);
    Ok(SystemTime::UNIX_EPOCH + duration_since_epoch)
}

#[inline]
pub fn read_md5_hash<I: Iterator<Item = u8>>(i: &mut I) -> IoResult<String> {
    let indicator = read_byte(i)?;
    if indicator == 0 {
        return Ok(String::new());
    } else if indicator != 0x0b {
        let msg = format!("{}: {}", HASH_ERR, indicator);
        return Err(invalid!(msg.as_str()));
    }
    let _ = read_byte(i)?; // ULEB128 encoding for 32 uses 1 byte
    let hash_bytes = [read_byte(i)?, read_byte(i)?, read_byte(i)?, read_byte(i)?, read_byte(i)?,
        read_byte(i)?, read_byte(i)?, read_byte(i)?, read_byte(i)?, read_byte(i)?, read_byte(i)?,
        read_byte(i)?, read_byte(i)?, read_byte(i)?, read_byte(i)?, read_byte(i)?, read_byte(i)?,
        read_byte(i)?, read_byte(i)?, read_byte(i)?, read_byte(i)?, read_byte(i)?, read_byte(i)?,
        read_byte(i)?, read_byte(i)?, read_byte(i)?, read_byte(i)?, read_byte(i)?, read_byte(i)?,
        read_byte(i)?, read_byte(i)?, read_byte(i)?].to_vec();
    Ok(String::from_iter(hash_bytes.iter().map(|&byte| byte as char)))
}

#[inline]
pub fn read_player_name<I: Iterator<Item = u8>>(i: &mut I) -> IoResult<String> {
    let indicator = read_byte(i)?;
    if indicator != 0x0b {
        let msg = format!("{}: {}", USERNAME_ERR, indicator);
        return Err(invalid!(msg.as_str()));
    }
    // Usernames are ASCII (1 byte in Unicode too), and so should never need more than a byte for
    // the player name string length
    let len = read_byte(i)?;
    let mut string_bytes = Vec::with_capacity(len as usize);
    for _ in 0..len {
        string_bytes.push(read_byte(i)?);
    }
    fromutf8_to_ioresult(String::from_utf8(string_bytes), "player name")
}

#[inline]
pub fn read_player_name_with_len<I: Iterator<Item = u8>>(i: &mut I) -> IoResult<(usize, String)> {
    let indicator = read_byte(i)?;
    if indicator != 0x0b {
        let msg = format!("{}: {}", USERNAME_ERR, indicator);
        return Err(invalid!(msg.as_str()));
    }
    // Usernames are ASCII (1 byte in Unicode too), and so should never need more than a byte for
    // the player name string length
    let len = read_byte(i)?;
    let mut string_bytes = Vec::with_capacity(len as usize);
    for _ in 0..len {
        string_bytes.push(read_byte(i)?);
    }
    // The +2 is to account for the indicator and ULEB128 integer
    Ok((len as usize + 2, fromutf8_to_ioresult(String::from_utf8(string_bytes), "player_name")?))
}