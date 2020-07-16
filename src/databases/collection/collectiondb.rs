use crate::databases::{collection::collection::Collection, load::Load};
use crate::deserialize_primitives::*;
use crate::read_error::{DbFileParseError, ParseErrorKind, ParseFileResult};
use crossbeam_utils::thread::{self, Scope, ScopedJoinHandle};
use std::sync::{Arc, Mutex};

/// Contains collections of beatmaps defined by the user, typically in the game. This can be used
/// to, say, group together practice maps for streams or jumps, or maybe a user's favourite maps.
#[derive(Debug, Clone)]
pub struct CollectionDb<'a> {
    pub version: i32,
    pub number_of_collections: i32,
    pub collections: Vec<Collection<'a>>,
}

impl<'a> Load<'a> for CollectionDb<'a> {
    fn read_single_thread(bytes: &'a [u8]) -> ParseFileResult<Self> {
        let mut index = 0;
        let i = &mut index;
        let version = read_int(&bytes, i)?;
        let number_of_collections = read_int(&bytes, i)?;
        let mut collections = Vec::with_capacity(number_of_collections as usize);
        for _ in 0..number_of_collections {
            collections.push(Collection::read_from_bytes(&bytes, i)?);
        }
        Ok(CollectionDb {
            version,
            number_of_collections,
            collections,
        })
    }

    fn read_multi_thread(jobs: usize, bytes: &'a [u8]) -> ParseFileResult<Self> {
        let version = read_int(&bytes, &mut 0)?;
        let number_of_collections = read_int(&bytes, &mut 4)?;
        // Keeps track of how many collections we've parsed.
        let counter = Arc::new(Mutex::new(0));
        // Keeps track of the offset into the file bytes from which to start parsing a new
        // collection.
        let start_read = Arc::new(Mutex::new(8));
        let mut results = thread::scope(|s| {
            let threads = (0..jobs)
                .map(|n| {
                    spawn_collection_loader_thread(
                        s,
                        number_of_collections as usize,
                        counter.clone(),
                        start_read.clone(),
                        bytes,
                    )
                })
                .collect::<Vec<_>>();
            // Join the threads and collect from them the result of the collections they've tried to
            // parse. If a thread failed to parse one, it'll return an error, which will then be
            // handled as described in the comment after the next.
            threads
                .into_iter()
                .map(|joinhandle| {
                    joinhandle.join().map_err(|_| {
                        DbFileParseError::new(
                            ParseErrorKind::CollectionDbError,
                            "Failed to join collection.db collection parsing thread.",
                        )
                    })?
                })
                .collect::<Vec<_>>()
        })
        .map_err(|_| {
            DbFileParseError::new(
                ParseErrorKind::CollectionDbError,
                "Failed to retrieve result from collection.db collection parsing scope.",
            )
        })?;
        // Take the first collection out of the parser thread results.
        let mut collections = results.pop().unwrap()?;
        // For each result in the parser threads, ensure that it's an `Ok(collection)`. Escalate the
        // first error that's found to the caller.
        // Note that this is done in a `for` loop for the express reason of being able to escalate
        // the error out of this function. I wouldn't be able to use it in a closure, say, like:
        //     let collections = results.into_iter().map(|result| result?).collect::<Vec<_>>();
        // since I want the closure to return a `Collection`, but the use of a `?` would require
        // that it returns a `Result<Collection, E>`.
        for collection_result in results {
            collections.append(&mut collection_result?);
        }
        // Sort by collection number - ensure that what is returned is in the same order as it is in
        // the database file.
        collections.sort_by(|(a, _), (b, _)| a.cmp(b));
        let collections = collections
            .into_iter()
            .map(|(_, collection)| collection)
            .collect::<Vec<_>>();
        // Get rid of the collection numbers and keep only the collections themselves.
        Ok(CollectionDb {
            version,
            number_of_collections,
            collections,
        })
    }
}

fn spawn_collection_loader_thread<'a, 'scope>(
    scope: &'scope Scope<'a>,
    number: usize,
    counter: Arc<Mutex<usize>>,
    start_read: Arc<Mutex<usize>>,
    bytes: &'a [u8],
) -> ScopedJoinHandle<'scope, ParseFileResult<Vec<(usize, Collection<'a>)>>> {
    scope.spawn(move |_| {
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
                let collection_name = read_str_utf8(bytes, &mut *start, "collection name")?;
                let number_of_beatmaps = read_int(bytes, &mut *start)?;
                let s = *start;
                // Accounts for: 1 indicator byte, 1 length byte, and 32 bytes for MD5 hash.
                *start += number_of_beatmaps as usize * 34;
                (collection_name, number_of_beatmaps, num, s)
            };
            let i = &mut start;
            let mut md5_beatmap_hashes = Vec::with_capacity(number_of_beatmaps as usize);
            for _ in 0..number_of_beatmaps {
                md5_beatmap_hashes.push(read_md5_hash(bytes, i)?);
            }
            collections.push((
                num,
                Collection {
                    collection_name,
                    number_of_beatmaps,
                    md5_beatmap_hashes,
                },
            ));
        }
    })
}
