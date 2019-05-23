use byteorder::{ReadBytesExt, LittleEndian};
use std::iter::Iterator;
use std::fs::File;
use std::io::{Result as IoResult, Error as IoError};
use std::io::ErrorKind::{Other, InvalidData};
use std::mem::size_of;
use std::string::FromUtf8Error;
use std::time::{Duration, SystemTime};
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

const BYTE_ERR: IoError = IoError::new(Other, "Failed to read byte.");
const SHORT_ERR: IoError = IoError::new(Other, "Failed to read byte for short.");
const INT_ERR: IoError = IoError::new(Other, "Failed to read byte for int.");
const LONG_ERR: IoError = IoError::new(Other, "Failed to read byte for long.");
const ULEB128_ERR: IoError =IoError::new(Other, "Failed to read byte for ULEB128.");
const SINGLE_ERR: IoError = IoError::new(Other, "Failed to read byte for single.");
const DOUBLE_ERR: IoError = IoError::new(Other, "Failed to read byte for double.");
const BOOLEAN_ERR: IoError = IoError::new(Other, "Failed to read byte for boolean.");
const STRING_ERR: IoError = IoError::new(Other, "Failed to read indicator for string.");
const DATETIME_ERR: IoError = IoError::new(Other, "Failed to read long for datetime.");

pub fn read_byte<I: Iterator<Item= u8>>(i: &mut I) -> IoResult<u8> {
    Ok(i.next().ok_or( BYTE_ERR)?)
}

pub fn read_short<I: Iterator<Item = u8>>(i: &mut I) -> IoResult<i16> {
    Ok(i.next().ok_or( SHORT_ERR)? as i16 + ((i.next().ok_or( SHORT_ERR)? as i16) << 8))
}

pub fn read_int<I: Iterator<Item = u8>>(i: &mut I) -> IoResult<i32> {
    Ok(i.next().ok_or( INT_ERR)? as i32 + ((i.next().ok_or( INT_ERR)? as i32) << 8)
        + ((i.next().ok_or( INT_ERR)? as i32) << 16) + ((i.next().ok_or( INT_ERR)? as i32) << 24))
}

pub fn read_long<I: Iterator<Item = u8>>(i: &mut I) -> IoResult<i64> {
    Ok(i.next().ok_or(LONG_ERR)? as i64 + ((i.next().ok_or(LONG_ERR)? as i64) << 8)
        + ((i.next().ok_or(LONG_ERR)? as i64) << 16) + ((i.next().ok_or(LONG_ERR)? as i64) << 24)
        + ((i.next().ok_or(LONG_ERR)? as i64) << 32) + ((i.next().ok_or(LONG_ERR)? as i64) << 40)
        + ((i.next().ok_or(LONG_ERR)? as i64) << 48) + ((i.next().ok_or(LONG_ERR)? as i64) << 56))
}

pub fn read_uleb128<I: Iterator<Item = u8>>(i: &mut I) -> IoResult<usize> {
    let mut out = 0;
    let mut found_end = false;
    let mut shift = 0;
    while shift < size_of::<usize>() * 8 {
        let b = i.next().ok_or(ULEB128_ERR)?;
        if shift + 8 >= size_of::<usize>() * 8 {
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
        if !(b | 0b00000000) & 0b10000000 == 0b10000000 {
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

pub fn read_uleb128_with_len<I: Iterator<Item = u8>>(i: &mut I) -> IoResult<(usize, usize)> {
    let mut out = 0;
    let mut found_end = false;
    let mut shift = 0;
    let mut len = 0;
    while shift < size_of::<usize>() * 8 {
        let b = i.next().ok_or(ULEB128_ERR)?;
        if shift + 8 >= size_of::<usize>() * 8 {
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
        if !(b | 0b00000000) & 0b10000000 == 0b10000000 {
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

pub fn read_single<I: Iterator<Item = u8>>(i: &mut I) -> IoResult<f32> {
    Ok(f32::from_bits(read_int(i).map_err(|_| SINGLE_ERR)? as u32))
}

pub fn read_double<I: Iterator<Item = u8>>(i: &mut I) -> IoResult<f64> {
    Ok(f64::from_bits(read_long(i).map_err(|_| DOUBLE_ERR)? as u64))
}

pub fn read_boolean<I: Iterator<Item = u8>>(i: &mut I) -> IoResult<bool> {
    Ok(i.next().ok_or(BOOLEAN_ERR)? == 0)
}

pub fn read_string_utf8<I: Iterator<Item = u8>>(i: &mut I)
    -> IoResult<Result<String, FromUtf8Error>> {
    let indicator = i.next().ok_or(STRING_ERR)?;
    if indicator == 0 {
        Ok(Ok(String::new()))
    } else if indicator == 0x0b {
        let length = read_uleb128(i)?;
        Ok(String::from_utf8(i.take(length).collect::<Vec<_>>()))
    } else {
        let err_msg = format!("Found invalid indicator for string ({})", indicator);
        Err(IoError::new(InvalidData, err_msg.as_str()))
    }
}

pub fn fromutf8_to_ioresult(r: Result<String, FromUtf8Error>, field: &str) -> IoResult<String> {
    match r {
        Ok(string) => Ok(string),
        Err(e) => {
            let err_msg = format!("Error reading {} ({})", field, e);
            Err(IoError::new(InvalidData, err_msg.as_str()))
        }
    }
}

pub fn read_datetime<I: Iterator<Item = u8>>(i: &mut I) -> IoResult<SystemTime> {
    let ticks = read_long(i).map_err(|_| DATETIME_ERR)?;
    let duration_since_epoch = Duration::from_micros(ticks as u64 / 10);
    Ok(SystemTime::UNIX_EPOCH + duration_since_epoch)
}