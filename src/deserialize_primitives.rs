use byteorder::{ReadBytesExt, LittleEndian};
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

pub fn read_short<R: ReadBytesExt>(stream: &mut R) -> IoResult<i16> {
    stream.read_i16::<LittleEndian>()
}

pub fn read_int<R: ReadBytesExt>(stream: &mut R) -> IoResult<i32> {
    stream.read_i32::<LittleEndian>()
}

pub fn read_long<R: ReadBytesExt>(stream: &mut R) -> IoResult<i64> {
    stream.read_i64::<LittleEndian>()
}

pub fn read_uleb128<R: ReadBytesExt>(stream: &mut R) -> IoResult<usize> {
    let mut out = 0;
    let mut found_end = false;
    let mut shift = 0;
    while shift < size_of::<usize>() * 8 {
        let b = stream.read_u8()?;
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
    }
    if found_end {
        Ok(out)
    } else {
        let err_msg = format!("While the ULEB128 integer format supports integers \
            of arbitrary lengths, this program will only handle ULEB128 integers up to and \
            including {} bits in length.", size_of::<usize>() * 8);
        Err(IoError::new(InvalidData, err_msg.as_str()))
    }
}

pub fn read_single<R: ReadBytesExt>(stream: &mut R) -> IoResult<f32> {
    stream.read_f32::<LittleEndian>()
}

pub fn read_double<R: ReadBytesExt>(stream: &mut R) -> IoResult<f64> {
    stream.read_f64::<LittleEndian>()
}

pub fn read_boolean<R: ReadBytesExt>(stream: &mut R) -> IoResult<bool> {
    Ok(stream.read_u8()? != 0)
}

pub fn read_string_utf8<R: ReadBytesExt>(stream: &mut R) -> IoResult<Result<String, FromUtf8Error>> {
    let indicator = stream.read_u8()?;
    if indicator == 0 {
        Ok(Ok(String::new()))
    } else if indicator == 0x0b {
        let length = read_uleb128(stream)?;
        let mut bytes = Vec::with_capacity(length);
        for _ in 0..length {
            bytes.push(stream.read_u8()?);
        }
        Ok(String::from_utf8(bytes))
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

pub fn read_datetime<R: ReadBytesExt>(stream: &mut R) -> IoResult<SystemTime> {
    let ticks = read_long(stream)?;
    let duration_since_epoch = Duration::from_micros(ticks as u64 / 10);
    Ok(SystemTime::UNIX_EPOCH + duration_since_epoch)
}