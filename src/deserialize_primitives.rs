use byteorder::{ReadBytesExt, LittleEndian};
use std::fs::File;
use std::io::{Result as IoResult, Error as IoError};
use std::io::ErrorKind::InvalidData;
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

pub fn read_short(bytes: &[u8], cursor: &mut usize) -> i16 {
    let ret = bytes[*cursor + 0] as i16 + ((bytes[*cursor + 1] as i16) << 8);
    *cursor += 2;
    ret
}

pub fn read_int(bytes: &[u8], cursor: &mut usize) -> i32 {
    let ret = bytes[*cursor + 0] as i32 + ((bytes[*cursor + 1] as i32) << 8)
        + ((bytes[*cursor + 2] as i32) << 16) + ((bytes[*cursor + 3] as i32) << 24);
    *cursor += 4;
    ret
}

pub fn read_long(bytes: &[u8], cursor: &mut usize) -> i64 {
    let ret = bytes[*cursor + 0] as i64 + ((bytes[*cursor + 1] as i64) << 8)
        + ((bytes[*cursor + 2] as i64) << 16) + ((bytes[*cursor + 3] as i64) << 24)
        + ((bytes[*cursor + 4] as i64) << 32) + ((bytes[*cursor + 5] as i64) << 40)
        + ((bytes[*cursor + 6] as i64) << 48) + ((bytes[*cursor + 7] as i64) << 56);
    *cursor += 8;
    ret
}

pub fn read_uleb128(bytes: &[u8], cursor: &mut usize) -> IoResult<usize> {
    let mut out = 0;
    let mut found_end = false;
    let mut shift = 0;
    let mut add = 0;
    while shift < size_of::<usize>() * 8 {
        let b = bytes[*cursor + add];
        if shift + 8 >= size_of::<usize>() * 8 {
            if 0b11111111 >> (size_of::<usize>() * 8 - shift) | b
                < (0b10000000 >> size_of::<usize>() * 8 - shift - 1) {
                out += (b as usize) << shift;
                found_end = true;
                break;
            } else {
                let err_msg = format!("While the ULEB128 integer format supports integers \
                    of arbitrary lengths, this program will only handle ULEB128 integers up to and \
                    including {} bits in length.", size_of::<usize>() * 8);
                return Err(IoError::new(InvalidData, err_msg.as_str()));
            }
        }
        out += (b as usize & 0b01111111) << shift;
        if !(b | 0b00000000) & 0b10000000 == 0b10000000 {
            found_end = true;
            break;
        }
        shift += 7;
        add += 1;
    }
    if found_end {
        *cursor += add;
        Ok(out)
    } else {
        let err_msg = format!("While the ULEB128 integer format supports integers \
            of arbitrary lengths, this program will only handle ULEB128 integers up to and \
            including {} bits in length.", size_of::<usize>() * 8);
        Err(IoError::new(InvalidData, err_msg.as_str()))
    }
}

pub fn read_single(bytes: &[u8], cursor: &mut usize) -> f32 {
    let necessary = [bytes[*cursor + 3], bytes[*cursor + 2], bytes[*cursor + 1], bytes[*cursor]];
    *cursor += 4;
    unsafe { std::mem::transmute::<[u8; 4], f32>(necessary) }
}

pub fn read_double(bytes: &[u8], cursor: &mut usize) -> f64 {
    let necessary = [bytes[*cursor + 7], bytes[*cursor + 6], bytes[*cursor + 5], bytes[*cursor + 4],
        bytes[*cursor + 3], bytes[*cursor + 2], bytes[*cursor + 1], bytes[*cursor]];
    *cursor += 8;
    unsafe { std::mem::transmute::<[u8; 8], f64>(necessary) }
}

pub fn read_boolean(bytes: &[u8], cursor: &mut usize) -> bool {
    let ret = bytes[cursor] == 0;
    *cursor += 1;
    ret
}

pub fn read_string_utf8(bytes: &[u8], cursor: &mut usize)
    -> IoResult<Result<String, FromUtf8Error>> {
    let indicator = bytes[*cursor];
    *cursor += 1;
    if indicator == 0 {
        Ok(Ok(String::new()))
    } else if indicator == 0x0b {
        let length = read_uleb128(bytes, cursor)?;
        let ret = Ok(String::from_utf8(bytes[*cursor..*cursor + length].into_vec()));
        *cursor += length;
        ret
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

pub fn read_datetime(bytes: &[u8], cursor: &mut usize) -> SystemTime {
    let ticks = read_long(bytes, cursor);
    let duration_since_epoch = Duration::from_micros(ticks as u64 / 10);
    SystemTime::UNIX_EPOCH + duration_since_epoch
}