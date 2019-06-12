use std::time::SystemTime;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use crate::deserialize_primitives::*;
use crate::databases::{load::Load, osu::GameplayMode};
use crate::read_error::{ParseFileResult, DbFileParseError, ParseErrorKind::*};

#[derive(Debug, Clone)]
pub struct Score {
    pub gameplay_mode: GameplayMode,
    pub score_version: i32,
    pub md5_beatmap_hash: Option<String>,
    pub player_name: Option<String>,
    pub md5_replay_hash: Option<String>,
    pub number_of_300s: i16,
    pub number_of_100s: i16, // 150s in Taiko, 100s in CTB, 200s in Mania
    pub number_of_50s: i16, // small fruit in CTB, 50s in Mania
    pub number_of_gekis: i16, // max 300s in Mania
    pub number_of_katus: i16, // 100s in mania
    pub number_of_misses: i16,
    pub replay_score: i32,
    pub max_combo: i16,
    pub perfect_combo: bool,
    pub mods_used: i32,
    pub empty_string: Option<String>,
    pub replay_timestamp: SystemTime,
    pub negative_one: i32,
    pub online_score_id: i64
}

impl Score {
    pub fn read_from_bytes(bytes: &[u8], i: &mut usize) -> ParseFileResult<Self> {
        let gameplay_mode = GameplayMode::read_from_bytes(bytes, i)?;
        let score_version = read_int(bytes, i)?;
        let md5_beatmap_hash = read_md5_hash(bytes, i)?;
        let player_name = read_string_utf8(bytes, i, "player name")?;
        let md5_replay_hash = read_md5_hash(bytes, i)?;
        let number_of_300s = read_short(bytes, i)?;
        let number_of_100s = read_short(bytes, i)?;
        let number_of_50s = read_short(bytes, i)?;
        let number_of_gekis = read_short(bytes, i)?;
        let number_of_katus = read_short(bytes, i)?;
        let number_of_misses = read_short(bytes, i)?;
        let replay_score = read_int(bytes, i)?;
        let max_combo = read_short(bytes, i)?;
        let perfect_combo = read_boolean(bytes, i)?;
        let mods_used = read_int(bytes, i)?;
        let empty_string = read_string_utf8(bytes, i, "empty string")?;
        let replay_timestamp = read_datetime(bytes, i)?;
        let negative_one = read_int(bytes, i)?;
        let online_score_id = read_long(bytes, i)?;
        Ok(Score {
            gameplay_mode,
            score_version,
            md5_beatmap_hash,
            player_name,
            md5_replay_hash,
            number_of_300s,
            number_of_100s,
            number_of_50s,
            number_of_gekis,
            number_of_katus,
            number_of_misses,
            replay_score,
            max_combo,
            perfect_combo,
            mods_used,
            empty_string,
            replay_timestamp,
            negative_one,
            online_score_id
        })
    }
}

#[derive(Debug, Clone)]
pub struct ScoreDbBeatmap {
    pub md5_beatmap_hash: Option<String>,
    pub number_of_scores: i32,
    pub scores: Option<Vec<Score>>
}

impl ScoreDbBeatmap {
    pub fn read_from_bytes(bytes: &[u8], i: &mut usize) -> ParseFileResult<Self> {
        let md5_beatmap_hash = read_md5_hash(bytes, i)?;
        let number_of_scores = read_int(bytes, i)?;
        let mut scores = if number_of_scores == 0 {
            None
        } else {
            let mut scores = Vec::with_capacity(number_of_scores as usize);
            for _ in 0..number_of_scores {
                scores.push(Score::read_from_bytes(bytes, i)?);
            }
            Some(scores)
        };
        Ok(ScoreDbBeatmap {
            md5_beatmap_hash,
            number_of_scores,
            scores
        })
    }
}

#[derive(Debug, Clone)]
pub struct ScoresDb {
    pub version: i32,
    pub number_of_beatmaps: i32,
    pub beatmaps: Vec<ScoreDbBeatmap>
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
            beatmaps
        })
    }

    fn read_multi_thread(jobs: usize, bytes: Vec<u8>) -> ParseFileResult<Self> {
        let (version, number_of_beatmaps) = {
            let mut index = 0;
            (read_int(&bytes, &mut index)?, read_int(&bytes, &mut index)?)
        };
        let counter = Arc::new(Mutex::new(0));
        let start_read = Arc::new(Mutex::new(8));
        let threads = (0..jobs).map(|i| spawn_scoredbbeatmap_loader(number_of_beatmaps as usize,
            counter.clone(), start_read.clone(), &bytes, i)).collect::<Vec<_>>();
        let mut results = threads.into_iter().map(|joinhandle| joinhandle.join().unwrap())
            .collect::<Vec<_>>();
        let mut scoredbbeatmaps = results.pop().unwrap()?;
        for scoredbbeatmap_result in results {
            scoredbbeatmaps.append(&mut scoredbbeatmap_result?);
        }
        scoredbbeatmaps.sort_by(|(a, _), (b, _)| a.cmp(b));
        let beatmaps = scoredbbeatmaps.into_iter().map(|(_, scoredbbeatmap)| scoredbbeatmap)
            .collect::<Vec<ScoreDbBeatmap>>();
        Ok(ScoresDb {
            version,
            number_of_beatmaps,
            beatmaps
        })
    }
}

fn spawn_scoredbbeatmap_loader(number_of_scoredbbeatmaps: usize, counter: Arc<Mutex<usize>>,
    start_read: Arc<Mutex<usize>>, bytes_pointer: *const Vec<u8>, thread_no: usize)
    -> JoinHandle<ParseFileResult<Vec<(usize, ScoreDbBeatmap)>>> {
    let tmp = bytes_pointer as usize;
    thread::spawn(move || {
        let bytes = unsafe { &*(tmp as *const Vec<u8>) };
        let mut score_db_beatmaps = Vec::new();
        loop {
            let (md5_beatmap_hash, number_of_scores, mut start_read, end, number) = {
                let mut ctr = counter.lock().unwrap();
                let number = if *ctr >= number_of_scoredbbeatmaps {
                    return Ok(score_db_beatmaps);
                } else {
                    *ctr += 1;
                    *ctr - 1
                };
                let mut s = start_read.lock().unwrap();
                let md5_beatmap_hash = read_md5_hash(bytes, &mut *s)?;
                *s += 34;
                let number_of_scores = read_int(bytes, &mut *s)?;
                // Skips:
                // 34 bytes for beatmap MD5 hash
                // 4 bytes for number of beatmaps
                *s += 4;
                let start_from = *s;
                for i in 0..number_of_scores {
                    // Skips:
                    // 1 byte for gameplay_mode
                    // 4 bytes for score_version
                    // 34 bytes for MD5 beatmap hash/1 byte if indicator is 0
                    *s += 39;
                    // Assuming 32 characters max length for username, +2 for indicator and ULEB128
                    let indicator = *bytes.get(*s)
                        .ok_or_else(|| DbFileParseError::new(PrimitiveError, "Failed to read \
                            indicator for player name."))?;
                    let player_name_len = if indicator == 0x0b {
                        *bytes.get(*s + 1).ok_or_else(|| DbFileParseError::new(PrimitiveError,
                            "Failed to read player name length."))?
                    } else if indicator == 0 {
                        0
                    } else {
                        return Err(DbFileParseError::new(PrimitiveError, "Read invalid indicator for score \
                            player name."));
                    };
                    if player_name_len & 0b10000000 == 0b10000000 {
                        return Err(DbFileParseError::new(PrimitiveError, "Read invalid player name \
                            length."));
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
            score_db_beatmaps.push((number, ScoreDbBeatmap {
                md5_beatmap_hash,
                number_of_scores,
                scores
            }));
        }
    })
}