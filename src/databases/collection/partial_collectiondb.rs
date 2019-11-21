use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

use crate::databases::{collection::partial_collection::PartialCollection, load::PartialLoad};
use crate::deserialize_primitives::*;
use crate::load_settings::collection::{
    collection_load_settings::CollectionLoadSettings,
    collectiondb_load_settings::CollectionDbLoadSettings,
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

impl PartialLoad<CollectionDbMask, CollectionDbLoadSettings> for PartialCollectionDb {
    fn read_single_thread(
        settings: CollectionDbLoadSettings,
        bytes: Vec<u8>,
    ) -> ParseFileResult<Self> {
        let mut skip = false;
        let mut index = 0;
        let i = &mut index;
        let version = maybe_read_int_nocomp(settings.version, &mut false, &bytes, i)?;
        let number_of_collections = read_int(&bytes, i)?;
        let collections = if settings.collections_query.ignore_all() || number_of_collections == 0 {
            None
        } else {
            let mut tmp = Vec::with_capacity(number_of_collections as usize);
            for _ in 0..number_of_collections {
                if let Some(collection) =
                    PartialCollection::read_from_bytes(settings.collections_query, &bytes, i)?
                {
                    tmp.push(collection);
                }
            }
            Some(tmp)
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
        settings: CollectionDbLoadSettings,
        jobs: usize,
        bytes: Vec<u8>,
    ) -> ParseFileResult<Self> {
        let mut ind = 0;
        let version = maybe_read_int_nocomp(settings.version, &mut false, &bytes, &mut ind)?;
        let number_of_collections = read_int(&bytes, &mut ind)?;
        let collections = if settings.collections_query.ignore_all() || number_of_collections == 0 {
            None
        } else {
            let counter = Arc::new(Mutex::new(0));
            let start_read = Arc::new(Mutex::new(8));
            let threads = (0..jobs)
                .map(|_| {
                    spawn_partial_collection_loader_thread(
                        &settings.collections_query,
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
    settings: *const CollectionLoadSettings,
    number: usize,
    counter: Arc<Mutex<usize>>,
    start_read: Arc<Mutex<usize>>,
    bytes_pointer: *const Vec<u8>,
) -> JoinHandle<ParseFileResult<Vec<(usize, PartialCollection)>>> {
    let tmp_b = bytes_pointer as usize;
    let tmp_s = settings as usize;
    thread::spawn(move || {
        let (bytes, settings) = unsafe {
            (
                &*(tmp_b as *const Vec<u8>),
                &*(tmp as *const CollectionLoadSettings),
            )
        };
        let mut collections = Vec::new();
        loop {
            let mut skip = false;
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
                    &settings.collection_name,
                    &mut skip,
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
            let md5_beatmap_hashes =
                if settings.md5_beatmap_hash.is_ignore() || number_of_beatmaps == 0 {
                    None
                } else {
                    let mut tmp = Vec::with_capacity(number_of_beatmaps as usize);
                    for _ in 0..number_of_beatmaps {
                        if let Some(hash) =
                            maybe_read_md5_hash(&settings.md5_beatmap_hash, &mut false, bytes, i)?
                        {
                            tmp.push(hash);
                        }
                    }
                    Some(tmp)
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
