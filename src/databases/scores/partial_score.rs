use std::time::SystemTime;

use crate::maybe_deserialize_primitives::*;
use crate::read_error::ParseFileResult;
use crate::databases::osu::primitives::GameplayMode;
use crate::masks::scores_mask::ScoreMask;

#[derive(Debug, Clone)]
pub struct PartialScore {
    pub gameplay_mode: Option<GameplayMode>,
    pub score_version: Option<i32>,
    pub md5_beatmap_hash: Option<String>,
    pub player_name: Option<String>,
    pub md5_replay_hash: Option<String>,
    pub number_of_300s: Option<i16>,
    pub number_of_100s: Option<i16>, // 150s in Taiko, 100s in CTB, 200s in Mania
    pub number_of_50s: Option<i16>, // small fruit in CTB, 50s in Mania
    pub number_of_gekis: Option<i16>, // max 300s in Mania
    pub number_of_katus: Option<i16>, // 100s in mania
    pub number_of_misses: Option<i16>,
    pub replay_score: Option<i32>,
    pub max_combo: Option<i16>,
    pub perfect_combo: Option<bool>,
    pub mods_used: Option<i32>,
    pub empty_string: Option<String>,
    pub replay_timestamp: Option<SystemTime>,
    pub negative_one: Option<i32>,
    pub online_score_id: Option<i64>
}

impl PartialScore {
    pub fn read_from_bytes(mask: ScoreMask, bytes: &[u8], i: &mut usize) -> ParseFileResult<Self> {
        let gameplay_mode = GameplayMode::maybe_read_from_bytes(mask.gameplay_mode, bytes, i)?;
        let score_version = maybe_read_int(mask.score_version, bytes, i)?;
        let md5_beatmap_hash = maybe_read_md5_hash(mask.md5_beatmap_hash, bytes, i)?;
        let player_name = maybe_read_string_utf8(mask.player_name, bytes, i, "player name")?;
        let md5_replay_hash = maybe_read_md5_hash(mask.md5_replay_hash, bytes, i)?;
        let number_of_300s = maybe_read_short(mask.number_of_300s, bytes, i)?;
        let number_of_100s = maybe_read_short(mask.number_of_100s, bytes, i)?;
        let number_of_50s = maybe_read_short(mask.number_of_50s, bytes, i)?;
        let number_of_gekis = maybe_read_short(mask.number_of_gekis, bytes, i)?;
        let number_of_katus = maybe_read_short(mask.number_of_katus, bytes, i)?;
        let number_of_misses = maybe_read_short(mask.number_of_misses, bytes, i)?;
        let replay_score = maybe_read_int(mask.replay_score, bytes, i)?;
        let max_combo = maybe_read_short(mask.max_combo, bytes, i)?;
        let perfect_combo = maybe_read_boolean(mask.perfect_combo, bytes, i)?;
        let mods_used = maybe_read_int(mask.mods_used, bytes, i)?;
        let empty_string = maybe_read_string_utf8(mask.empty_string, bytes, i, "empty string")?;
        let replay_timestamp = maybe_read_datetime(mask.replay_timestamp, bytes, i)?;
        let negative_one = maybe_read_int(mask.negative_one, bytes, i)?;
        let online_score_id = maybe_read_long(mask.online_score_id, bytes, i)?;
        Ok(PartialScore {
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