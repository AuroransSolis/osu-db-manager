use std::fs::File;
use std::io::{Result as IoResult, Error as IoError, ErrorKind};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::env;
use std::sync::{Arc, Mutex, atomic::{AtomicUsize, AtomicU64, Ordering}};
use std::thread::{self, JoinHandle};
use crate::deserialize_primitives::*;
use crate::databases::load::Load;

pub struct Collection {
    collection_name: String,
    number_of_beatmaps: i32,
    md5_beatmap_hashes: Vec<String>
}

impl Collection {
    pub fn read_from_bytes<I: Iterator<Item = u8>>(i: &mut I) -> IoResult<Self> {
        let collection_name = fromutf8_to_ioresult(read_string_utf8(i)?, "collection name")?;
        let number_of_beatmaps = read_int(i)?;
        let mut md5_beatmap_hashes = Vec::with_capacity(number_of_beatmaps as usize);
        for _ in 0..number_of_beatmaps {
            md5_beatmap_hashes.push(fromutf8_to_ioresult(read_string_utf8(i)?,
                "MD5 beatmap hash")?);
        }
        Ok(Collection {
            collection_name,
            number_of_beatmaps,
            md5_beatmap_hashes
        })
    }
}

pub struct CollectionDb {
    version: i32,
    number_of_collections: i32,
    collections: Vec<Collection>
}

impl Load for CollectionDb {
    fn read_single_thread(bytes: Vec<u8>) -> IoResult<Self> {
        let mut byte_iter = bytes.into_iter();
        let version = read_int(byte_iter)?;
        let number_of_collections = read_int(byte_iter)?;
        let mut collections = Vec::with_capacity(number_of_collections as usize);
        for _ in 0..number_of_collections {
            collections.push(Collection::read_from_bytes(&mut byte_iter)?);
        }
        Ok(CollectionDb {
            version,
            number_of_collections,
            collections
        })
    }

    fn read_multi_thread(jobs: usize, bytes: Vec<u8>) -> IoResult<Self> {

    }
}

/*fn get_collection_slices(bytes: &Vec<u8>) -> Vec<&[u8]> {
    let number_of_collections = {
        read_int(&mut bytes.iter().skip(4))?;
    };
    let mut start = 8;
    let mut slices = Vec::with_capacity(number_of_collections as usize);
    for _ in 0..slices {
        let (collection_name_len, bytes_used) = read_uleb128_with_len(&mut bytes.iter())?;
        let
    }
}*/

fn spawn_collection_loader_thread(number: usize, counter: AtomicUsize, start_read: AtomicUsize,
    byte_pointer: *const Vec<u8>) -> JoinHandle<IoResult<Vec<(usize, Collection)>>> {
    thread::spawn(move || {
        let bytes = unsafe { &*byte_pointer };
        let mut collections = Vec::new();
        loop {
            let (collection_name, number_of_beatmaps, num, start) = {
                let ctr = counter.get_mut();
                let start = start_read.get_mut();
                if *ctr == number {
                    return Ok(collections);
                }
                let indicator = bytes[*start];
                if indicator != 0x0b {
                    return Err(IoError::new(ErrorKind::InvalidData, "Read invalid indicator for \
                        collection name String."));
                }
                *start += 1;
                let (collection_name_len, bytes_used) = read_uleb128_with_len(&mut bytes[start..start + 1].iter())?;
                let collection_name = String::from_iter(
                    bytes[start + bytes_used..start + bytes_used + collection_name_len].iter());
                let number_of_beatmaps = read_int(bytes[start + bytes_used
                    + collection_name_len..start + bytes_used + collection_name_len + 4].iter())?;
                let num = *ctr;
                *ctr += 1;
                *start += bytes_used + collection_name_len + 4;
                let s = *start;
                // Accounts for: 1 indicator byte, 1 length byte, and 16 bytes for MD5 hash.
                *start += number_of_beatmaps * 18;
                (collection_name, number_of_beatmaps, num, s);
            };
            let mut bytes_iter = bytes[start..start + number_of_beatmaps * 18].iter();
            let mut md5_beatmap_hashes = Vec::with_capacity(number_of_beatmaps as usize);
            for _ in 0..number_of_beatmaps {
                md5_beatmap_hashes.push(fromutf8_to_ioresult(
                    read_string_utf8(&mut bytes_iter.take(18))?, "MD5 beatmap hash")?);
            }
            collections.push((num, Collection {
                collection_name,
                number_of_beatmaps,
                md5_beatmap_hashes
            }));
        }
    })
}