use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use crate::deserialize_primitives::*;
use crate::databases::{load::PartialLoad, collection::partial_collection::PartialCollection};
use crate::read_error::{ParseFileResult};
use crate::masks::collection_mask::CollectionDbMask;

#[derive(Debug, Clone)]
pub struct PartialCollectionDb {
    pub version: Option<i32>,
    pub number_of_collections: i32,
    pub collections: Option<Vec<PartialCollection>>
}

impl PartialLoad<CollectionDbMask> for PartialCollectionDb {
    fn read_single_thread(mask: CollectionDbMask, bytes: Vec<u8>) -> ParseFileResult<Self> {
        let mut index = 0;
        let i = &mut index;
        let version = if mask.version {
            Some(read_int(&bytes, i)?)
        } else {
            *i += 4;
            None
        };
        let number_of_collections = read_int(&bytes, i)?;
        let mut collections = if let Some(collections_mask) = mask.collections_mask {
            let mut tmp = Vec::with_capacity(number_of_collections as usize);
            for _ in 0..number_of_collections {
                tmp.push(PartialCollection::read_from_bytes(collections_mask, &bytes, i)?);
            }
            Some(tmp)
        } else {
            None
        };
        Ok(PartialCollectionDb {
            version,
            number_of_collections,
            collections
        })
    }

    fn read_multi_thread(mask: CollectionDbMask, jobs: usize, bytes: Vec<u8>)
        -> ParseFileResult<Self> {
        let (version, number_of_collections) = {
            let mut index = 0;
            (if mask.version {
                Some(read_int(&bytes, &mut index)?)
            } else {
                index += 4;
                None
            }, read_int(&bytes, &mut index)?)
        };
        let collections = if mask.collections {
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
            Some(collections.into_iter().map(|(_, collection)| collection)
                .collect::<Vec<PartialCollection>>())
        } else {
            None
        };
        Ok(PartialCollectionDb {
            version,
            number_of_collections,
            collections
        })
    }
}

fn spawn_collection_loader_thread(number: usize, counter: Arc<Mutex<usize>>,
    start_read: Arc<Mutex<usize>>, byte_pointer: *const Vec<u8>)
    -> JoinHandle<ParseFileResult<Vec<(usize, PartialCollection)>>> {
    let tmp = byte_pointer as usize;
    thread::spawn(move || {
        let bytes = unsafe { &*(tmp as *const Vec<u8>) };
        let mut collections = Vec::new();
        loop {
            let (collection_name, number_of_beatmaps, num, mut start) = {
                let mut ctr = counter.lock().unwrap();
                if *ctr >= number {
                    return Ok(collections);
                } else {
                    *ctr += 1;
                }
                let num = *ctr - 1;
                let mut start = start_read.lock().unwrap();
                let (bytes_used, collection_name) = read_string_utf8_with_len(
                    bytes, &mut *start, "collection name")?;
                let number_of_beatmaps = read_int(bytes, &mut *start)?;
                let s = *start;
                // Accounts for: 1 indicator byte, 1 length byte, and 32 bytes for MD5 hash.
                *start += number_of_beatmaps as usize * 34;
                (collection_name, number_of_beatmaps, num, s)
            };
            let mut i = &mut start;
            let mut md5_beatmap_hashes = Vec::with_capacity(number_of_beatmaps as usize);
            for _ in 0..number_of_beatmaps {
                md5_beatmap_hashes.push(read_md5_hash(bytes, i)?);
            }
            collections.push((num, PartialCollection {
                collection_name,
                number_of_beatmaps,
                md5_beatmap_hashes
            }));
        }
    })
}
