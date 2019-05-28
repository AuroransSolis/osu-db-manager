use std::fs::File;
use std::io::{Result as IoResult, Error as IoError, ErrorKind::InvalidData};
use std::time::{Duration, SystemTime};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::sync::{Arc, Mutex, atomic::{AtomicUsize, AtomicU64, Ordering}};
use std::thread::{self, JoinHandle};
use crate::deserialize_primitives::*;
use crate::databases::{load::Load, osu::GameplayMode};

pub struct Score {
    gameplay_mode: GameplayMode,
    score_version: i32,
    md5_beatmap_hash: String,
    player_name: String,
    md5_replay_hash: String,
    number_of_300s: i16,
    number_of_100s: i16, // 150s in Taiko, 100s in CTB, 200s in Mania
    number_of_50s: i16, // small fruit in CTB, 50s in Mania
    number_of_gekis: i16, // max 300s in Mania
    number_of_katus: i16, // 100s in mania
    number_of_misses: i16,
    replay_score: i32,
    max_combo: i16,
    perfect_combo: bool,
    mods_used: i32,
    empty_string: String,
    replay_timestamp: SystemTime,
    negative_one: i32,
    online_score_id: i64
}

impl Score {
    pub fn read_from_bytes<I: Iterator<Item = u8>>(i: &mut I) -> IoResult<Self> {
        let gameplay_mode = GameplayMode::read_from_bytes(i)?;
        let score_version = read_int(i)?;
        let md5_beatmap_hash = read_md5_hash(i).map_err(IoError::new(Other,
            "Error reading MD5 beatmap hash"))?;
        let player_name = fromutf8_to_ioresult(read_string_utf8(i)?, "player name")?;
        let md5_replay_hash = read_md5_hash(i).map_err(IoError::new(Other,
            "Error reading MD5 replay hash"))?;
        let number_of_300s = read_short(i)?;
        let number_of_100s = read_short(i)?;
        let number_of_50s = read_short(i)?;
        let number_of_gekis = read_short(i)?;
        let number_of_katus = read_short(i)?;
        let number_of_misses = read_short(i)?;
        let replay_score = read_int(i)?;
        let max_combo = read_short(i)?;
        let perfect_combo = read_boolean(i)?;
        let mods_used = read_int(i)?;
        let empty_string = fromutf8_to_ioresult(read_string_utf8(i)?, "empty string")?;
        let replay_timestamp = read_datetime(i)?;
        let negative_one = read_int(i)?;
        let online_score_id = read_long(i)?;
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

pub struct ScoreDbBeatmap {
    md5_beatmap_hash: String,
    number_of_scores: i32,
    scores: Vec<Score>
}

impl ScoreDbBeatmap {
    pub fn read_from_bytes<I: Iterator<Item = u8>>(i: &mut I) -> IoResult<Self> {
        let md5_beatmap_hash = fromutf8_to_ioresult(read_string_utf8(i)?, "MD5 beatmap hash")?;
        let number_of_scores = read_int(i)?;
        let mut scores = Vec::with_capacity(number_of_scores as usize);
        for _ in 0..number_of_scores {
            scores.push(Score::read_from_bytes(i)?);
        }
        Ok(ScoreDbBeatmap {
            md5_beatmap_hash,
            number_of_scores,
            scores
        })
    }
}

pub struct ScoresDb {
    version: i32,
    number_of_beatmaps: i32,
    beatmaps: Vec<ScoreDbBeatmap>
}

impl Load for ScoresDb {
    fn read_single_thread(bytes: Vec<u8>) -> IoResult<Self> {
        let mut bytes_iter = bytes.into_iter();
        let version = read_int(&mut bytes_iter)?;
        let number_of_beatmaps = read_int(&mut bytes_iter)?;
        let mut beatmaps = Vec::with_capacity(number_of_beatmaps as usize);
        for _ in 0..number_of_beatmaps {
            beatmaps.push(ScoreDbBeatmap::read_from_bytes(&mut bytes_iter)?);
        }
        Ok(ScoresDb {
            version,
            number_of_beatmaps,
            beatmaps
        })
    }

    fn read_multi_thread(bytes: Vec<u8>) -> IoResult<Self> {

    }
}

/*
pub struct ScoreDbBeatmap {
    md5_beatmap_hash: String,
    number_of_scores: i32,
    scores: Vec<Score>
}
pub struct Score {
    gameplay_mode: GameplayMode,
    score_version: i32,
    md5_beatmap_hash: String,
    player_name: String,
    md5_replay_hash: String,
    number_of_300s: i16,
    number_of_100s: i16, // 150s in Taiko, 100s in CTB, 200s in Mania
    number_of_50s: i16, // small fruit in CTB, 50s in Mania
    number_of_gekis: i16, // max 300s in Mania
    number_of_katus: i16, // 100s in mania
    number_of_misses: i16,
    replay_score: i32,
    max_combo: i16,
    perfect_combo: bool,
    mods_used: i32,
    empty_string: String,
    replay_timestamp: SystemTime,
    negative_one: i32,
    online_score_id: i64
}
*/

fn spawn_scoredbbeatmap_loader(number_of_scoredbbeatmaps: usize, counter: Arc<Mutex<usize>>,
    start_read: Arc<Mutex<usize>>, bytes_pointer: *const Vec<u8>)
    -> JoinHandle<IoResult<Vec<ScoreDbBeatmap>>> {
    thread::spawn(move || {
        let bytes = unsafe { &*bytes_pointer };
        let mut ScoreDbBeatmaps = Vec::new();
        loop {
            let (md5_beatmap_hash, number_of_scores) = {
                let ctr = counter.lock().unwrap();
                let s = start_read.lock().unwrap();
                let md5_beatmap_hash = read_md5_hash(&mut bytes[*s..*s+ 18].iter())?;
                let number_of_scores = read_int(&mut bytes[*s + 18..*s + 22].iter())?;
                for _ in 0..number_of_scores - 1 {
                    // Skips:
                    // 1 byte for gameplay_mode
                    // 4 bytes for score_version
                    // 18 bytes for MD5 beatmap hash
                    *s += 23;
                    // Assuming 32 characters max length for username, +2 for indicator and ULEB128
                    let (player_name_len, player_name) = read_player_name_with_len(
                        &mut bytes[*s..*s + 34].iter())?;
                    *s += player_name_len + 62;
                    // Skips:
                    // 18 bytes for replay MD5 hash
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
                    // Total of 62
                }
                (md5_beatmap_hash, number_of_scores)
            };
            
        }
    })
}