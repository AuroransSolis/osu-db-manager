use crate::databases::scores::{score::Score, scoresdb_beatmap::ScoresDbBeatmap};
use crate::deserialize_primitives::*;
use crate::read_error::{DbFileParseError, ParseErrorKind, ParseFileResult};
use crossbeam_utils::thread::{self, Scope, ScopedJoinHandle};
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct ScoresDb<'a> {
    pub version: i32,
    pub number_of_beatmaps: i32,
    pub beatmaps: Vec<ScoresDbBeatmap<'a>>,
}

impl<'a> ScoresDb<'a> {
    pub fn read_from_bytes(jobs: usize, bytes: &'a [u8]) -> ParseFileResult<Self> {
        if jobs == 1 {
            Self::read_single_thread(bytes)
        } else {
            Self::read_multi_thread(jobs, bytes)
        }
    }

    pub fn read_single_thread(bytes: &'a [u8]) -> ParseFileResult<ScoresDb<'a>> {
        let mut index = 0;
        let i = &mut index;
        let version = read_int(&bytes, i)?;
        let number_of_beatmaps = read_int(&bytes, i)?;
        let mut beatmaps = Vec::with_capacity(number_of_beatmaps as usize);
        for _ in 0..number_of_beatmaps {
            beatmaps.push(ScoresDbBeatmap::read_from_bytes(&bytes, i)?);
        }
        Ok(ScoresDb {
            version,
            number_of_beatmaps,
            beatmaps,
        })
    }

    pub fn read_multi_thread(jobs: usize, bytes: &'a [u8]) -> ParseFileResult<ScoresDb<'a>> {
        let (version, number_of_beatmaps) = {
            let mut index = 0;
            (read_int(&bytes, &mut index)?, read_int(&bytes, &mut index)?)
        };
        let counter = Arc::new(Mutex::new(0));
        let start_read = Arc::new(Mutex::new(8));
        let mut results = thread::scope(|s| {
            let threads = (0..jobs)
                .map(|_| {
                    spawn_scoresdb_beatmap_loader_thread(
                        s,
                        number_of_beatmaps as usize,
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
                            ParseErrorKind::ScoresDbError,
                            "Failed to join scores.db beatmap parsing thread.",
                        )
                    })?
                })
                .collect::<Vec<_>>()
        })
        .map_err(|_| {
            DbFileParseError::new(
                ParseErrorKind::ScoresDbError,
                "Failed to retrieve result from scores.db beatmap parsing scope.",
            )
        })?;
        let mut beatmaps = results.pop().unwrap()?;
        for beatmap_result in results {
            beatmaps.append(&mut beatmap_result?);
        }
        beatmaps.sort_by(|(a, _), (b, _)| a.cmp(b));
        let beatmaps = beatmaps
            .into_iter()
            .map(|(_, beatmap)| beatmap)
            .collect::<Vec<_>>();
        Ok(ScoresDb {
            version,
            number_of_beatmaps,
            beatmaps,
        })
    }

    pub fn display(&self) {
        println!("version: {}", self.version);
        println!("number of beatmaps: {}", self.number_of_beatmaps);
        println!("beatmaps {{");
        for beatmap in &self.beatmaps {
            beatmap.display();
        }
        println!("}}");
    }
}

fn spawn_scoresdb_beatmap_loader_thread<'scope, 'b: 'scope, 'a: 'b>(
    scope: &'scope Scope<'b>,
    number_of_scores_db_beatmaps: usize,
    counter: Arc<Mutex<usize>>,
    start_read: Arc<Mutex<usize>>,
    bytes: &'a [u8],
) -> ScopedJoinHandle<'scope, ParseFileResult<Vec<(usize, ScoresDbBeatmap<'a>)>>> {
    scope.spawn(move |_| {
        let mut scores_db_beatmaps = Vec::new();
        loop {
            let (md5_beatmap_hash, number_of_scores, mut start_read, number) = {
                let mut ctr = counter.lock().unwrap();
                let number = if *ctr >= number_of_scores_db_beatmaps {
                    return Ok(scores_db_beatmaps);
                } else {
                    *ctr += 1;
                    *ctr - 1
                };
                let mut start = start_read.lock().unwrap();
                let md5_beatmap_hash = read_md5_hash(bytes, start.deref_mut())?;
                let number_of_scores = read_int(bytes, start.deref_mut())?;
                let start_from = *start;
                for _ in 0..number_of_scores {
                    // Skips:
                    // 1 byte for gameplay_mode
                    // 4 bytes for score_version
                    // 34 bytes for MD5 beatmap hash/1 byte if indicator is 0
                    *start += 39;
                    // Assuming 32 characters max length for username, +2 for
                    // indicator and ULEB128
                    let indicator = *bytes.get(*start).ok_or_else(|| {
                        DbFileParseError::new(
                            ParseErrorKind::PrimitiveError,
                            "Failed to read \
                                                indicator for player name.",
                        )
                    })?;
                    let player_name_len = if indicator == 0x0b {
                        *bytes.get(*start + 1).ok_or_else(|| {
                            DbFileParseError::new(
                                ParseErrorKind::PrimitiveError,
                                "Failed to read player name length.",
                            )
                        })?
                    } else if indicator == 0 {
                        0
                    } else {
                        return Err(DbFileParseError::new(
                            ParseErrorKind::PrimitiveError,
                            "Read invalid indicator for score \
                                                player name.",
                        ));
                    };
                    if player_name_len & 0b10000000 == 0b10000000 {
                        return Err(DbFileParseError::new(
                            ParseErrorKind::PrimitiveError,
                            "Read invalid player name \
                                                length.",
                        ));
                    }
                    if indicator == 0 {
                        *start += 1;
                    } else {
                        *start += 2;
                    }
                    *start += player_name_len as usize + 78;
                    // Skips:
                    // 34 bytes for replay MD5 hash
                    // 2 bytes for number of 300s
                    // 2 bytes for number of 100s
                    // 2 bytes for number of 50s
                    // 2 bytes for number of gekis
                    // 2 bytes for number of katus
                    // 2 bytes for number of misses
                    // 4 bytes for score
                    // 2 bytes for max combo
                    // 1 byte for perfect combo
                    // 4 bytes for mods used
                    // 1 byte for empty string indicator
                    // 8 bytes for replay timestamp
                    // 4 bytes for 0xFFFFFFFF
                    // 8 bytes for score ID
                    // Total of 78
                }
                (md5_beatmap_hash, number_of_scores, start_from, number)
            };
            let scores = if number_of_scores == 0 {
                None
            } else {
                let mut scores = Vec::with_capacity(number_of_scores as usize);
                let i = &mut start_read;
                for _ in 0..number_of_scores {
                    scores.push(Score::read_from_bytes(bytes, i)?);
                }
                Some(scores)
            };
            scores_db_beatmaps.push((
                number,
                ScoresDbBeatmap {
                    md5_beatmap_hash,
                    number_of_scores,
                    scores,
                },
            ));
        }
    })
}
