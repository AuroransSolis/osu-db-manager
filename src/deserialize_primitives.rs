use byteorder::{ReadBytesExt, LittleEndian};
use std::fs::File;
use std::io::{Result as IoResult, Error as IoError};
use std::io::ErrorKind::InvalidData;
use std::mem::size_of;
use std::string::FromUtf8Error;

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

pub fn read_short(file: &mut File) -> IoResult<i16> {
    file.read_i16::<LittleEndian>()
}

pub fn read_int(file: &mut File) -> IoResult<i32> {
    file.read_i32::<LittleEndian>()
}

pub fn read_long(file: &mut File) -> IoResult<i64> {
    file.read_i64::<LittleEndian>()
}

pub fn read_uleb128(file: &mut File) -> IoResult<usize> {
    let mut out = 0;
    let mut found_end = false;
    let mut shift = 0;
    while shift < size_of::<usize>() * 8 {
        let b = file.read_u8()?;
        if shift + 8 >= size_of::<usize>() * 8 {
            if 0b11111111 >> (size_of::<usize>() * 8 - shift) | b
                < (0b10000000 >> size_of::<usize>() * 8 - shift - 1) {
                out += (b as usize) << shift;
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

pub fn read_single(file: &mut File) -> IoResult<f32> {
    file.read_f32::<LittleEndian>()
}

pub fn read_double(file: &mut File) -> IoResult<f64> {
    file.read_f64::<LittleEndian>()
}

pub fn read_boolean(file: &mut File) -> IoResult<bool> {
    Ok(file.read_u8()? != 0)
}

pub fn read_string_utf8(file: &mut File) -> IoResult<Result<String, FromUtf8Error>> {
    let indicator = file.read_u8()?;
    if indicator == 0 {
        Ok(Ok(String::new()))
    } else if indicator == 0x0b {
        let length = read_uleb128(file)?;
        let mut bytes = Vec::with_capacity(length);
        for _ in 0..length {
            bytes.push(file.read_u8()?);
        }
        Ok(String::from_utf8(bytes))
    } else {
        let err_msg = format!("Found invalid indicator for string ({})", indicator);
        Err(IoError::new(InvalidData, err_msg.as_str()))
    }
}