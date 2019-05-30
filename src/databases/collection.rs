use std::io::{Result as IoResult, Error as IoError, ErrorKind};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::iter::FromIterator;
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
            md5_beatmap_hashes.push(read_md5_hash(i)?);
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
        let version = read_int(&mut byte_iter)?;
        let number_of_collections = read_int(&mut byte_iter)?;
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
        let (version, number_of_collections) = {
            let mut bytes_iter = bytes.iter().cloned();
            (read_int(&mut bytes_iter)?, read_int(&mut bytes_iter)?)
        };
        let counter = Arc::new(Mutex::new(0));
        let start_read = Arc::new(Mutex::new(8));
        let threads = (0..jobs)
            .map(|_| spawn_collection_loader_thread(number_of_collections as usize, counter.clone(),
                start_read.clone(), &bytes)).collect::<Vec<_>>();
        let mut results = threads.into_iter().map(|joinhandle| joinhandle.join().unwrap())
            .collect::<Vec<_>>();
        let mut collections = results.pop().unwrap()?;
        for collection_result in results {
            collections.append(&mut collection_result?);
        }
        collections.sort_by(|(a, _), (b, _)| a.cmp(b));
        let collections = collections.into_iter().map(|(_, collection)| collection)
            .collect::<Vec<Collection>>();
        Ok(CollectionDb {
            version,
            number_of_collections,
            collections
        })
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

fn spawn_collection_loader_thread(number: usize, counter: Arc<Mutex<usize>>,
    start_read: Arc<Mutex<usize>>, byte_pointer: *const Vec<u8>)
    -> JoinHandle<IoResult<Vec<(usize, Collection)>>> {
    let tmp = byte_pointer as usize;
    thread::spawn(move || {
        let bytes = unsafe { &*(tmp as *const Vec<u8>) };
        let mut collections = Vec::new();
        loop {
            let (collection_name, number_of_beatmaps, num, start) = {
                let mut ctr = counter.lock().unwrap();
                if *ctr >= number {
                    return Ok(collections);
                } else {
                    *ctr += 1;
                }
                let mut start = start_read.lock().unwrap();
                let indicator = bytes[*start];
                if indicator != 0x0b {
                    return Err(IoError::new(ErrorKind::InvalidData, "Read invalid indicator for \
                        collection name String."));
                }
                *start += 1;
                let (collection_name_len, bytes_used) = read_uleb128_with_len(
                    &mut (&bytes[*start..*start + 1]).iter().cloned())?;
                let collection_name = String::from_iter(
                    bytes[*start + bytes_used..*start + bytes_used + collection_name_len].iter()
                        .map(|&byte| byte as char));
                let number_of_beatmaps = read_int(&mut (&bytes[*start + bytes_used
                    + collection_name_len..*start + bytes_used + collection_name_len + 4]).iter().cloned())?;
                let num = *ctr - 1;
                *ctr += 1;
                *start += bytes_used + collection_name_len + 4;
                let s = *start;
                // Accounts for: 1 indicator byte, 1 length byte, and 16 bytes for MD5 hash.
                *start += number_of_beatmaps as usize * 18;
                (collection_name, number_of_beatmaps, num, s)
            };
            let mut bytes_iter = bytes[start..start + number_of_beatmaps as usize * 18].iter().cloned();
            let mut md5_beatmap_hashes = Vec::with_capacity(number_of_beatmaps as usize);
            for _ in 0..number_of_beatmaps {
                md5_beatmap_hashes.push(read_md5_hash(&mut bytes_iter)?);
            }
            collections.push((num, Collection {
                collection_name,
                number_of_beatmaps,
                md5_beatmap_hashes
            }));
        }
    })
}
