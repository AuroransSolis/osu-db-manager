use crate::databases::osu::primitives::GameplayMode;
use crate::load_settings::{EqualClone, EqualCopy, Relational};
use crate::masks::scores_mask::ScoreMask;
use chrono::NaiveDate;
use clap::ArgMatches;
use std::io::Result as IoResult;

#[derive(Clone)]
pub struct ScoreLoadSettings {
    pub gameplay_mode: EqualCopy<GameplayMode>,
    pub score_version: Relational<i32>,
    pub md5_beatmap_hash: EqualClone<String>,
    pub player_name: EqualClone<String>,
    pub md5_replay_hash: EqualClone<String>,
    pub number_of_300s: Relational<i16>,
    pub number_of_100s: Relational<i16>,
    pub number_of_50s: Relational<i16>,
    pub number_of_gekis: Relational<i16>,
    pub number_of_katus: Relational<i16>,
    pub number_of_misses: Relational<i16>,
    pub replay_score: Relational<i32>,
    pub max_combo: Relational<i16>,
    pub perfect_combo: EqualCopy<bool>,
    pub mods_used: Relational<i32>,
    pub empty_string: bool,
    pub replay_timestamp: Relational<NaiveDate>,
    pub negative_one: bool,
    pub online_score_id: Relational<i64>,
}

impl ScoreLoadSettings {
    pub fn load_all(&self) -> bool {
        self.gameplay_mode.is_load()
            && self.score_version.is_load()
            && self.md5_beatmap_hash.is_load()
            && self.player_name.is_load()
            && self.md5_replay_hash.is_load()
            && self.number_of_300s.is_load()
            && self.number_of_100s.is_load()
            && self.number_of_50s.is_load()
            && self.number_of_gekis.is_load()
            && self.number_of_katus.is_load()
            && self.number_of_misses.is_load()
            && self.replay_score.is_load()
            && self.max_combo.is_load()
            && self.perfect_combo.is_load()
            && self.mods_used.is_load()
            && self.empty_string
            && self.replay_timestamp.is_load()
            && self.negative_one
            && self.online_score_id.is_load()
    }

    pub fn ignore_all(&self) -> bool {
        self.gameplay_mode.is_ignore()
            && self.score_version.is_ignore()
            && self.md5_beatmap_hash.is_ignore()
            && self.player_name.is_ignore()
            && self.md5_replay_hash.is_ignore()
            && self.number_of_300s.is_ignore()
            && self.number_of_100s.is_ignore()
            && self.number_of_50s.is_ignore()
            && self.number_of_gekis.is_ignore()
            && self.number_of_katus.is_ignore()
            && self.number_of_misses.is_ignore()
            && self.replay_score.is_ignore()
            && self.max_combo.is_ignore()
            && self.perfect_combo.is_ignore()
            && self.mods_used.is_ignore()
            && !self.empty_string
            && self.replay_timestamp.is_ignore()
            && !self.negative_one
            && self.online_score_id.is_ignore()
    }

    pub fn is_partial(&self) -> bool {
        self.gameplay_mode.is_ignore()
            || self.score_version.is_ignore()
            || self.md5_beatmap_hash.is_ignore()
            || self.player_name.is_ignore()
            || self.md5_replay_hash.is_ignore()
            || self.number_of_300s.is_ignore()
            || self.number_of_100s.is_ignore()
            || self.number_of_50s.is_ignore()
            || self.number_of_gekis.is_ignore()
            || self.number_of_katus.is_ignore()
            || self.number_of_misses.is_ignore()
            || self.replay_score.is_ignore()
            || self.max_combo.is_ignore()
            || self.perfect_combo.is_ignore()
            || self.mods_used.is_ignore()
            || !self.empty_string
            || self.replay_timestamp.is_ignore()
            || !self.negative_one
            || self.online_score_id.is_ignore()
    }

    pub fn set_from_query(&mut self, matches: &ArgMatches) -> IoResult<()> {
        self.gameplay_mode = EqualCopy::from_matches(matches, "Gameplay mode")?;
        self.score_version = Relational::from_matches(matches, "Score version")?;
        self.md5_beatmap_hash = EqualClone::from_matches(matches, "MD5 beatmap hash")?;
        self.player_name = EqualClone::from_matches(matches, "Player name")?;
        self.md5_replay_hash = EqualClone::from_matches(matches, "MD5 replay hash")?;
        self.number_of_300s = Relational::from_matches(matches, "Number of 300s")?;
        self.number_of_100s = Relational::from_matches(
            matches,
            "Number of 100s (standard/CTB)/150s (taiko)/200s (mania)",
        )?;
        self.number_of_50s =
            Relational::from_matches(matches, "Number of 50s (standard/mania)/small fruit (CTB)")?;
        self.number_of_gekis =
            Relational::from_matches(matches, "Number of gekis/max 300s (mania)")?;
        self.number_of_katus = Relational::from_matches(matches, "Number of katus/100s (mania)")?;
        self.number_of_misses = Relational::from_matches(matches, "Number of misses")?;
        self.replay_score = Relational::from_matches(matches, "Replay score")?;
        self.max_combo = Relational::from_matches(matches, "Max combo")?;
        self.perfect_combo = EqualCopy::bool_from_matches(matches, "Perfect combo")?;
        self.mods_used = Relational::from_matches(matches, "Mods used")?;
        self.replay_timestamp = Relational::date_from_matches(matches, "Replay timestamp")?;
        self.online_score_id = Relational::from_matches(matches, "Online score ID")?;
        Ok(())
    }

    pub fn set_from_mask(&mut self, mask: &ScoreMask) {
        self.gameplay_mode.apply_mask(mask.gameplay_mode);
        self.score_version.apply_mask(mask.score_version);
        self.md5_beatmap_hash.apply_mask(mask.md5_beatmap_hash);
        self.player_name.apply_mask(mask.player_name);
        self.md5_replay_hash.apply_mask(mask.md5_replay_hash);
        self.number_of_300s.apply_mask(mask.number_of_300s);
        self.number_of_100s.apply_mask(mask.number_of_100s);
        self.number_of_50s.apply_mask(mask.number_of_50s);
        self.number_of_gekis.apply_mask(mask.number_of_gekis);
        self.number_of_katus.apply_mask(mask.number_of_katus);
        self.number_of_misses.apply_mask(mask.number_of_misses);
        self.replay_score.apply_mask(mask.replay_score);
        self.max_combo.apply_mask(mask.max_combo);
        self.perfect_combo.apply_mask(mask.perfect_combo);
        self.mods_used.apply_mask(mask.mods_used);
        self.empty_string |= mask.empty_string;
        self.replay_timestamp.apply_mask(mask.replay_timestamp);
        self.negative_one |= mask.negative_one;
        self.online_score_id.apply_mask(mask.online_score_id);
    }
}
