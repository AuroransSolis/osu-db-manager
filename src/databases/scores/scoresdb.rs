use crate::databases::{
    load::Load,
    scores::{score::Score, scoresdb_beatmap::ScoreDbBeatmap},
};
use crate::deserialize_primitives::*;
use crate::read_error::{DbFileParseError, ParseErrorKind::*, ParseFileResult};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

#[derive(Debug, Clone)]
pub struct ScoresDb {
    pub version: i32,
    pub number_of_beatmaps: i32,
    pub beatmaps: Vec<ScoreDbBeatmap>,
}

impl Load for ScoresDb {
    fn read_single_thread(bytes: Vec<u8>) -> ParseFileResult<Self> {
        let mut index = 0;
        let i = &mut index;
        let version = read_int(&bytes, i)?;
        let number_of_beatmaps = read_int(&bytes, i)?;
        let mut beatmaps = Vec::with_capacity(number_of_beatmaps as usize);
        for _ in 0..number_of_beatmaps {
            beatmaps.push(ScoreDbBeatmap::read_from_bytes(&bytes, i)?);
        }
        Ok(ScoresDb {
            version,
            number_of_beatmaps,
            beatmaps,
        })
    }

    fn read_multi_thread(jobs: usize, bytes: Vec<u8>) -> ParseFileResult<Self> {
        let (version, number_of_beatmaps) = {
            let mut index = 0;
            (read_int(&bytes, &mut index)?, read_int(&bytes, &mut index)?)
        };
        let counter = Arc::new(Mutex::new(0));
        let start_read = Arc::new(Mutex::new(8));
        let threads = (0..jobs)
            .map(|_| {
                spawn_scoredbbeatmap_loader(
                    number_of_beatmaps as usize,
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
        let mut scoredbbeatmaps = results.pop().unwrap()?;
        for scoredbbeatmap_result in results {
            scoredbbeatmaps.append(&mut scoredbbeatmap_result?);
        }
        scoredbbeatmaps.sort_by(|(a, _), (b, _)| a.cmp(b));
        let beatmaps = scoredbbeatmaps
            .into_iter()
            .map(|(_, scoredbbeatmap)| scoredbbeatmap)
            .collect::<Vec<ScoreDbBeatmap>>();
        Ok(ScoresDb {
            version,
            number_of_beatmaps,
            beatmaps,
        })
    }
}

fn spawn_scoredbbeatmap_loader(
    number_of_scoredbbeatmaps: usize,
    counter: Arc<Mutex<usize>>,
    start_read: Arc<Mutex<usize>>,
    bytes_pointer: *const Vec<u8>,
) -> JoinHandle<ParseFileResult<Vec<(usize, ScoreDbBeatmap)>>> {
    let tmp = bytes_pointer as usize;
    thread::spawn(move || {
        let bytes = unsafe { &*(tmp as *const Vec<u8>) };
        let mut score_db_beatmaps = Vec::new();
        loop {
            let (md5_beatmap_hash, number_of_scores, mut start_read, number) = {
                let mut ctr = counter.lock().unwrap();
                let number = if *ctr >= number_of_scoredbbeatmaps {
                    return Ok(score_db_beatmaps);
                } else {
                    *ctr += 1;
                    *ctr - 1
                };
                let mut s = start_read.lock().unwrap();
                let md5_beatmap_hash = read_md5_hash(bytes, &mut *s)?;
                let number_of_scores = read_int(bytes, &mut *s)?;
                let start_from = *s;
                for _ in 0..number_of_scores {
                    // Skips:
                    // 1 byte for gameplay_mode
                    // 4 bytes for score_version
                    // 34 bytes for MD5 beatmap hash/1 byte if indicator is 0
                    *s += 39;
                    // Assuming 32 characters max length for username, +2 for indicator and ULEB128
                    let indicator = *bytes.get(*s).ok_or_else(|| {
                        DbFileParseError::new(
                            PrimitiveError,
                            "Failed to read \
                             indicator for player name.",
                        )
                    })?;
                    let player_name_len = if indicator == 0x0b {
                        *bytes.get(*s + 1).ok_or_else(|| {
                            DbFileParseError::new(
                                PrimitiveError,
                                "Failed to read player name length.",
                            )
                        })?
                    } else if indicator == 0 {
                        0
                    } else {
                        return Err(DbFileParseError::new(
                            PrimitiveError,
                            "Read invalid indicator for score \
                             player name.",
                        ));
                    };
                    if player_name_len & 0b10000000 == 0b10000000 {
                        return Err(DbFileParseError::new(
                            PrimitiveError,
                            "Read invalid player name \
                             length.",
                        ));
                    }
                    if indicator == 0 {
                        *s += 1;
                    } else {
                        *s += 2;
                    }
                    *s += player_name_len as usize + 78;
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
            score_db_beatmaps.push((
                number,
                ScoreDbBeatmap {
                    md5_beatmap_hash,
                    number_of_scores,
                    scores,
                },
            ));
        }
    })
}
