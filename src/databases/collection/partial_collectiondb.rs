use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

use crate::databases::{collection::partial_collection::PartialCollection, load::PartialLoad};
use crate::deserialize_primitives::*;
use crate::load_settings::{
    collection::collectiondb_load_settings::CollectionDbLoadSettings, FilterResult,
};
use crate::masks::collection_mask::{CollectionDbMask, CollectionMask};
use crate::maybe_deserialize_primitives::*;
use crate::read_error::ParseFileResult;

#[derive(Debug, Clone)]
pub struct PartialCollectionDb {
    pub version: Option<i32>,
    pub number_of_collections: Option<i32>,
    pub collections: Option<Vec<PartialCollection>>,
}

impl PartialLoad<CollectionDbMask> for PartialCollectionDb {
    fn read_single_thread(
        settings: CollectionDbLoadSettings,
        bytes: Vec<u8>,
    ) -> ParseFileResult<Self> {
        let mut skip = false;
        let mut index = 0;
        let i = &mut index;
        let version = read_int(&bytes, i)?;
        let number_of_collections = read_int(&bytes, i)?;
        let collections = if !settings.collections_query.ignore_all() {
            let mut tmp = Vec::with_capacity(number_of_collections as usize);
            for _ in 0..number_of_collections {
                if let Some(collection) =
                    PartialCollection::read_from_bytes(settings.collections_query, &bytes, i)?
                {
                    tmp.push(collection);
                }
            }
            Some(tmp)
        } else {
            None
        };
        let version = if settings.version.is_ignore() {
            Some(version)
        } else {
            None
        };
        let number_of_collections = if mask.number_of_collections {
            Some(number_of_collections)
        } else {
            None
        };
        Ok(PartialCollectionDb {
            version,
            number_of_collections,
            collections,
        })
    }

    fn read_multi_thread(
        mask: CollectionDbMask,
        jobs: usize,
        bytes: Vec<u8>,
    ) -> ParseFileResult<Self> {
        let (version, number_of_collections) = {
            let mut index = 0;
            (
                if mask.version {
                    Some(read_int(&bytes, &mut index)?)
                } else {
                    index += 4;
                    None
                },
                read_int(&bytes, &mut index)?,
            )
        };
        let collections = if let Some(collections_mask) = mask.collections_mask {
            if number_of_collections == 0 {
                None
            } else {
                let counter = Arc::new(Mutex::new(0));
                let start_read = Arc::new(Mutex::new(8));
                let threads = (0..jobs)
                    .map(|_| {
                        spawn_partial_collection_loader_thread(
                            collections_mask,
                            number_of_collections as usize,
                            counter.clone(),
                            start_read.clone(),
                            &bytes,
                        )
                    })
                    .collect::<Vec<_>>();
                let mut results = threads
                    .into_iter()
                    .map(|joinhandle| joinhandle.join().unwrap())
                    .collect::<Vec<_>>();
                let mut partial_collections = results.pop().unwrap()?;
                for partial_collection_result in results {
                    partial_collections.append(&mut partial_collection_result?);
                }
                partial_collections.sort_by(|(a, _), (b, _)| a.cmp(b));
                Some(
                    partial_collections
                        .into_iter()
                        .map(|(_, partial_collection)| partial_collection)
                        .collect::<Vec<PartialCollection>>(),
                )
            }
        } else {
            None
        };
        let number_of_collections = if mask.number_of_collections {
            Some(number_of_collections)
        } else {
            None
        };
        Ok(PartialCollectionDb {
            version,
            number_of_collections,
            collections,
        })
    }
}

fn spawn_partial_collection_loader_thread(
    mask: CollectionMask,
    number: usize,
    counter: Arc<Mutex<usize>>,
    start_read: Arc<Mutex<usize>>,
    bytes_pointer: *const Vec<u8>,
) -> JoinHandle<ParseFileResult<Vec<(usize, PartialCollection)>>> {
    let tmp = bytes_pointer as usize;
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
                let collection_name = maybe_read_string_utf8(
                    mask.collection_name,
                    bytes,
                    &mut *start,
                    "collection name",
                )?;
                let number_of_beatmaps = read_int(bytes, &mut *start)?;
                let s = *start;
                // Accounts for: 1 indicator byte, 1 length byte, and 32 bytes for MD5 hash.
                *start += number_of_beatmaps as usize * 34;
                (collection_name, number_of_beatmaps, num, s)
            };
            let i = &mut start;
            let md5_beatmap_hashes = if mask.md5_beatmap_hashes {
                if number_of_beatmaps == 0 {
                    None
                } else {
                    let mut tmp = Vec::with_capacity(number_of_beatmaps as usize);
                    for _ in 0..number_of_beatmaps {
                        tmp.push(read_md5_hash(bytes, i)?);
                    }
                    Some(tmp)
                }
            } else {
                None
            };
            let number_of_beatmaps = if mask.number_of_beatmaps {
                Some(number_of_beatmaps)
            } else {
                None
            };
            collections.push((
                num,
                PartialCollection {
                    collection_name,
                    number_of_beatmaps,
                    md5_beatmap_hashes,
                },
            ));
        }
    })
}
