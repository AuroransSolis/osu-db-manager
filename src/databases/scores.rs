use std::fs::File;
use std::io::{Result as IoResult, Error as IoError, ErrorKind::InvalidData};
use std::time::{Duration, SystemTime};
use std::fmt::{Display, Formatter, Result as FmtResult};
use crate::deserialize_primitives::*;
use crate::databases::osu::{GameplayMode, Beatmap};

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
    pub fn read_from_file(file: &mut File) -> IoResult<Self> {
        let gameplay_mode = GameplayMode::read_from_file(file)?;
        let score_version = read_int(file)?;
        let md5_beatmap_hash = fromutf8_to_ioresult(read_string_utf8(file)?, "MD5 beatmap hash")?;
        let player_name = fromutf8_to_ioresult(read_string_utf8(file)?, "player name")?;
        let md5_replay_hash = fromutf8_to_ioresult(read_string_utf8(file)?, "MD5 replay hash")?;
        let number_of_300s = read_short(file)?;
        let number_of_100s = read_short(file)?;
        let number_of_50s = read_short(file)?;
        let number_of_gekis = read_short(file)?;
        let number_of_katus = read_short(file)?;
        let number_of_misses = read_short(file)?;
        let replay_score = read_int(file)?;
        let max_combo = read_short(file)?;
        let perfect_combo = read_boolean(file)?;
        let mods_used = read_int(file)?;
        let empty_string = fromutf8_to_ioresult(read_string_utf8(file)?, "empty string")?;
        let replay_timestamp = read_datetime(file)?;
        let negative_one = read_int(file)?;
        let online_score_id = read_long(file)?;
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
    pub fn read_from_file(file: &mut File) -> IoResult<Self> {
        let md5_beatmap_hash = fromutf8_to_ioresult(read_string_utf8(file)?, "MD5 beatmap hash")?;
        let number_of_scores = read_int(file)?;
        let mut scores = Vec::with_capacity(number_of_scores as usize);
        for _ in 0..number_of_scores {
            scores.push(Score::read_from_file(file)?);
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

impl ScoresDb {
    pub fn read_from_file(file: &mut File) -> IoResult<Self> {
        let version = read_int(file)?;
        let number_of_beatmaps = read_int(file)?;
        let mut beatmaps = Vec::with_capacity(number_of_beatmaps as usize);
        for _ in 0..number_of_beatmaps {
            beatmaps.push(ScoreDbBeatmap::read_from_file(file)?);
        }
        Ok(ScoresDb {
            version,
            number_of_beatmaps,
            beatmaps
        })
    }
}