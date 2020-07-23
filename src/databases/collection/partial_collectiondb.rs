use crate::databases::collection::partial_collection::PartialCollection;
use crate::deserialize_primitives::*;
use crate::load_settings::collection::{
    collection_load_settings::CollectionLoadSettings,
    collectiondb_load_settings::CollectionDbLoadSettings,
};
use crate::maybe_deserialize_primitives::*;
use crate::read_error::{DbFileParseError, ParseErrorKind, ParseFileResult};
use crate::{masks::collection_mask::CollectionDbMask, maybe_print, maybe_print_vec};
use crossbeam_utils::thread::{self, Scope, ScopedJoinHandle};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct PartialCollectionDb<'a> {
    pub version: Option<i32>,
    pub number_of_collections: Option<i32>,
    pub collections: Option<Vec<PartialCollection<'a>>>,
}

impl<'a> PartialCollectionDb<'a> {
    pub fn read_from_bytes(
        settings: CollectionDbLoadSettings,
        jobs: usize,
        bytes: &'a [u8],
    ) -> ParseFileResult<Self> {
        if jobs == 1 {
            Self::read_single_thread(settings, bytes)
        } else {
            Self::read_multi_thread(settings, jobs, bytes)
        }
    }

    pub fn read_single_thread(
        settings: CollectionDbLoadSettings,
        bytes: &'a [u8],
    ) -> ParseFileResult<Self> {
        let mut index = 0;
        let i = &mut index;
        let version = maybe_read_int_nocomp(settings.version, &mut false, &bytes, i)?;
        let number_of_collections = read_int(&bytes, i)?;
        let collections =
            if settings.collection_load_settings.ignore_all() || number_of_collections == 0 {
                None
            } else {
                let mut tmp = Vec::with_capacity(number_of_collections as usize);
                for _ in 0..number_of_collections {
                    if let Some(collection) = PartialCollection::read_from_bytes(
                        settings.collection_load_settings.clone(),
                        &bytes,
                        i,
                    )? {
                        tmp.push(collection);
                    }
                }
                Some(tmp)
            };
        let number_of_collections = if settings.number_of_collections {
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

    pub fn read_multi_thread(
        settings: CollectionDbLoadSettings,
        jobs: usize,
        bytes: &'a [u8],
    ) -> ParseFileResult<Self> {
        let mut ind = 0;
        let version = maybe_read_int_nocomp(settings.version, &mut false, &bytes, &mut ind)?;
        let number_of_collections = read_int(&bytes, &mut ind)?;
        let collections =
            if settings.collection_load_settings.ignore_all() || number_of_collections == 0 {
                None
            } else {
                let counter = Arc::new(Mutex::new(0));
                let start_read = Arc::new(Mutex::new(8));
                let mut results = thread::scope(|s| {
                    let threads = (0..jobs)
                        .map(|_| {
                            spawn_partial_collection_loader_thread(
                                s,
                                number_of_collections as usize,
                                counter.clone(),
                                start_read.clone(),
                                bytes,
                                &settings.collection_load_settings,
                            )
                        })
                        .collect::<Vec<_>>();
                    threads
                        .into_iter()
                        .map(|joinhandle| {
                            joinhandle.join().map_err(|_| {
                                DbFileParseError::new(
                                ParseErrorKind::CollectionDbError,
                                "Failed to join collection.db partial collection parsing thread.",
                            )
                            })?
                        })
                        .collect::<Vec<_>>()
                })
                .map_err(|_| {
                    DbFileParseError::new(
                        ParseErrorKind::CollectionDbError,
                        "Failed to retrieve result from collection.db partial collection parsing \
                        scope.",
                    )
                })?;
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
        let number_of_collections = if settings.number_of_collections {
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

    pub fn display(&self, show: CollectionDbMask) {
        if !show.ignore_all() {
            maybe_print!(show.version, self.version);
            maybe_print!(show.number_of_collections, self.number_of_collections);
            if self.collections.is_some() && !show.collections_mask.ignore_all() {
                for collection in self.collections.as_ref().unwrap() {
                    collection.display(show.collections_mask);
                }
            }
        }
    }
}

fn spawn_partial_collection_loader_thread<'scope, 'b: 'scope, 'a: 'b>(
    scope: &'scope Scope<'b>,
    number: usize,
    counter: Arc<Mutex<usize>>,
    start_read: Arc<Mutex<usize>>,
    bytes: &'a [u8],
    settings: &'b CollectionLoadSettings,
) -> ScopedJoinHandle<'scope, ParseFileResult<Vec<(usize, PartialCollection<'a>)>>> {
    scope.spawn(move |_| {
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
                let collection_name = maybe_read_str_utf8(
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
            let number_of_beatmaps = if settings.number_of_beatmaps.compare(&number_of_beatmaps) {
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
