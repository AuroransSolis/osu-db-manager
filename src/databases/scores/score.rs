use chrono::NaiveDate;

use crate::databases::osu::primitives::GameplayMode;
use crate::deserialize_primitives::*;
use crate::read_error::ParseFileResult;

#[derive(Debug, Clone)]
pub struct Score<'a> {
    pub gameplay_mode: GameplayMode,
    pub score_version: i32,
    pub md5_beatmap_hash: &'a str,
    pub player_name: Option<&'a str>,
    pub md5_replay_hash: &'a str,
    pub number_of_300s: i16,
    pub number_of_100s: i16,  // 150s in Taiko, 100s in CTB, 200s in Mania
    pub number_of_50s: i16,   // small fruit in CTB, 50s in Mania
    pub number_of_gekis: i16, // max 300s in Mania
    pub number_of_katus: i16, // 100s in mania
    pub number_of_misses: i16,
    pub replay_score: i32,
    pub max_combo: i16,
    pub perfect_combo: bool,
    pub mods_used: i32,
    pub empty_string: Option<&'a str>,
    pub replay_timestamp: NaiveDate,
    pub negative_one: i32,
    pub online_score_id: i64,
}

impl Score {
    pub fn read_from_bytes(bytes: &[u8], i: &mut usize) -> ParseFileResult<Self> {
        let gameplay_mode = GameplayMode::read_from_bytes(bytes, i)?;
        let score_version = read_int(bytes, i)?;
        let md5_beatmap_hash = read_md5_hash(bytes, i)?;
        let player_name = read_str_utf8(bytes, i, "player name")?;
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
        let empty_string = read_str_utf8(bytes, i, "empty string")?;
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
            online_score_id,
        })
    }
}
