use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

use crate::databases::{
    load::PartialLoad,
    scores::{partial_score::PartialScore, partial_scoresdb_beatmap::PartialScoresDbBeatmap},
};
use crate::deserialize_primitives::*;
use crate::load_settings::scores::scoresdb_load_settings::ScoresDbLoadSettings;
use crate::masks::scores_mask::{ScoresDbBeatmapMask, ScoresDbMask};
use crate::read_error::{DbFileParseError, ParseErrorKind::PrimitiveError, ParseFileResult};

#[derive(Debug, Clone)]
pub struct PartialScoresDb {
    pub version: Option<i32>,
    pub number_of_beatmaps: i32,
    pub beatmaps: Option<Vec<PartialScoresDbBeatmap>>,
}

impl PartialLoad<ScoresDbMask, ScoresDbLoadSettings> for PartialScoresDb {
    fn read_single_thread(mask: ScoresDbMask, bytes: Vec<u8>) -> ParseFileResult<Self> {
        let mut index = 0;
        let i = &mut index;
        let version = if mask.version {
            Some(read_int(&bytes, i)?)
        } else {
            *i += 4;
            None
        };
        let number_of_beatmaps = read_int(&bytes, i)?;
        let beatmaps = if let Some(m) = mask.beatmaps_mask {
            if number_of_beatmaps == 0 {
                None
            } else {
                let mut tmp = Vec::with_capacity(number_of_beatmaps as usize);
                for _ in 0..number_of_beatmaps {
                    tmp.push(PartialScoresDbBeatmap::read_from_bytes(m, &bytes, i)?);
                }
                Some(tmp)
            }
        } else {
            None
        };
        Ok(PartialScoresDb {
            version,
            number_of_beatmaps,
            beatmaps,
        })
    }

    fn read_multi_thread(mask: ScoresDbMask, jobs: usize, bytes: Vec<u8>) -> ParseFileResult<Self> {
        let (version, number_of_beatmaps) = {
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
        let beatmaps = if let Some(m) = mask.beatmaps_mask {
            let counter = Arc::new(Mutex::new(0));
            let start_read = Arc::new(Mutex::new(8));
            let threads = (0..jobs)
                .map(|i| {
                    spawn_partial_scoredb_beatmap_loader(
                        m,
                        number_of_beatmaps as usize,
                        counter.clone(),
                        start_read.clone(),
                        &bytes,
                        i,
                    )
                })
                .collect::<Vec<_>>();
            let mut results = threads
                .into_iter()
                .map(|joinhandle| joinhandle.join().unwrap())
                .collect::<Vec<_>>();
            let mut partial_scoredb_beatmaps = results.pop().unwrap()?;
            for partial_scoredb_beatmap_result in results {
                partial_scoredb_beatmaps.append(&mut partial_scoredb_beatmap_result?);
            }
            partial_scoredb_beatmaps.sort_by(|(a, _), (b, _)| a.cmp(b));
            Some(
                partial_scoredb_beatmaps
                    .into_iter()
                    .map(|(_, scoredbbeatmap)| scoredbbeatmap)
                    .collect::<Vec<PartialScoresDbBeatmap>>(),
            )
        } else {
            None
        };
        Ok(PartialScoresDb {
            version,
            number_of_beatmaps,
            beatmaps,
        })
    }
}

fn spawn_partial_scoredb_beatmap_loader(
    mask: ScoresDbBeatmapMask,
    number_of_scoredb_beatmaps: usize,
    counter: Arc<Mutex<usize>>,
    start_read: Arc<Mutex<usize>>,
    bytes_pointer: *const Vec<u8>,
    thread_no: usize,
) -> JoinHandle<ParseFileResult<Vec<(usize, PartialScoresDbBeatmap)>>> {
    let tmp = bytes_pointer as usize;
    thread::spawn(move || {
        let bytes = unsafe { &*(tmp as *const Vec<u8>) };
        let mut partial_scoresdb_beatmaps = Vec::new();
        loop {
            let (md5_beatmap_hash, number_of_scores, mut start_read, end, number) = {
                let mut ctr = counter.lock().unwrap();
                let number = if *ctr >= number_of_scoredb_beatmaps {
                    return Ok(partial_scoresdb_beatmaps);
                } else {
                    *ctr += 1;
                    *ctr - 1
                };
                let mut s = start_read.lock().unwrap();
                let md5_beatmap_hash = if mask.md5_beatmap_hash {
                    Some(read_md5_hash(bytes, &mut *s)?)
                } else {
                    *s += 34;
                    None
                };
                let number_of_scores = read_int(bytes, &mut *s)?;
                let start_from = *s;
                for i in 0..number_of_scores {
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
                (md5_beatmap_hash, number_of_scores, start_from, *s, number)
            };
            let scores = if let Some(m) = mask.scores_mask {
                if number_of_scores == 0 {
                    None
                } else {
                    let mut tmp = Vec::with_capacity(number_of_scores as usize);
                    let i = &mut start_read;
                    for _ in 0..number_of_scores {
                        tmp.push(PartialScore::read_from_bytes(m, bytes, i)?);
                    }
                    Some(tmp)
                }
            } else {
                None
            };
            partial_scoresdb_beatmaps.push((
                number,
                PartialScoresDbBeatmap {
                    md5_beatmap_hash,
                    number_of_scores,
                    scores,
                },
            ));
        }
    })
}
