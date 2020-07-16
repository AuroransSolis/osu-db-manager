use chrono::NaiveDate;

use crate::databases::osu::primitives::GameplayMode;
use crate::load_settings::scores::score_load_settings::ScoreLoadSettings;
use crate::maybe_deserialize_primitives::*;
use crate::read_error::ParseFileResult;

#[derive(Debug, Clone)]
pub struct PartialScore<'a> {
    pub gameplay_mode: Option<GameplayMode>,
    pub score_version: Option<i32>,
    pub md5_beatmap_hash: Option<&'a str>,
    pub player_name: Option<&'a str>,
    pub md5_replay_hash: Option<&'a str>,
    pub number_of_300s: Option<i16>,
    pub number_of_100s: Option<i16>, // 150s in Taiko, 100s in CTB, 200s in Mania
    pub number_of_50s: Option<i16>,  // small fruit in CTB, 50s in Mania
    pub number_of_gekis: Option<i16>, // max 300s in Mania
    pub number_of_katus: Option<i16>, // 100s in mania
    pub number_of_misses: Option<i16>,
    pub replay_score: Option<i32>,
    pub max_combo: Option<i16>,
    pub perfect_combo: Option<bool>,
    pub mods_used: Option<i32>,
    pub empty_string: Option<&'a str>,
    pub replay_timestamp: Option<NaiveDate>,
    pub negative_one: Option<i32>,
    pub online_score_id: Option<i64>,
}

impl<'a> PartialScore<'a> {
    pub fn read_from_bytes(
        settings: &ScoreLoadSettings,
        bytes: &'a [u8],
        i: &mut usize,
    ) -> ParseFileResult<Option<Self>> {
        let mut skip = false;
        let s = &mut skip;
        let gameplay_mode =
            GameplayMode::maybe_read_from_bytes(settings.gameplay_mode, s, bytes, i)?;
        let score_version = maybe_read_int(settings.score_version, s, bytes, i)?;
        let md5_beatmap_hash = maybe_read_md5_hash(&settings.md5_beatmap_hash, s, bytes, i)?;
        let player_name = maybe_read_str_utf8(&settings.player_name, s, bytes, i, "player name")?;
        let md5_replay_hash = maybe_read_md5_hash(&settings.md5_replay_hash, s, bytes, i)?;
        let number_of_300s = maybe_read_short(settings.number_of_300s, s, bytes, i)?;
        let number_of_100s = maybe_read_short(settings.number_of_100s, s, bytes, i)?;
        let number_of_50s = maybe_read_short(settings.number_of_50s, s, bytes, i)?;
        let number_of_gekis = maybe_read_short(settings.number_of_gekis, s, bytes, i)?;
        let number_of_katus = maybe_read_short(settings.number_of_katus, s, bytes, i)?;
        let number_of_misses = maybe_read_short(settings.number_of_misses, s, bytes, i)?;
        let replay_score = maybe_read_int(settings.replay_score, s, bytes, i)?;
        let max_combo = maybe_read_short(settings.max_combo, s, bytes, i)?;
        let perfect_combo = maybe_read_boolean(settings.perfect_combo, s, bytes, i)?;
        let mods_used = maybe_read_int(settings.mods_used, s, bytes, i)?;
        let empty_string =
            maybe_read_str_utf8_nocomp(settings.empty_string, s, bytes, i, "empty string")?;
        let replay_timestamp = maybe_read_datetime(settings.replay_timestamp, s, bytes, i)?;
        let negative_one = maybe_read_int_nocomp(settings.negative_one, s, bytes, i)?;
        let online_score_id = maybe_read_long(settings.online_score_id, s, bytes, i)?;
        if *s {
            Ok(None)
        } else {
            Ok(Some(PartialScore {
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
            }))
        }
    }
}
