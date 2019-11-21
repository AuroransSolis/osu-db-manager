use std::io::Result as IoResult;

use chrono::NaiveDate;
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

use crate::databases::osu::primitives::GameplayMode;
use crate::load_settings::{query::QueryStruct, EqualClone, EqualCopy, LoadSetting, Relational};
use crate::masks::scores_mask::ScoreMask;

pub struct ScoreLoadSettings {
    pub gameplay_mode: LoadSetting<EqualCopy<GameplayMode>>,
    pub score_version: LoadSetting<Relational<i32>>,
    pub md5_beatmap_hash: LoadSetting<EqualClone<String>>,
    pub player_name: LoadSetting<EqualClone<String>>,
    pub md5_replay_hash: LoadSetting<EqualClone<String>>,
    pub number_of_300s: LoadSetting<Relational<i16>>,
    pub number_of_100s: LoadSetting<Relational<i16>>,
    pub number_of_50s: LoadSetting<Relational<i16>>,
    pub number_of_gekis: LoadSetting<Relational<i16>>,
    pub number_of_katus: LoadSetting<Relational<i16>>,
    pub number_of_misses: LoadSetting<Relational<i16>>,
    pub replay_score: LoadSetting<Relational<i32>>,
    pub max_combo: LoadSetting<Relational<i16>>,
    pub perfect_combo: LoadSetting<EqualCopy<bool>>,
    pub mods_used: LoadSetting<Relational<i32>>,
    pub empty_string: LoadSetting<()>,
    pub replay_timestamp: LoadSetting<Relational<NaiveDate>>,
    pub negative_one: LoadSetting<()>,
    pub online_score_id: LoadSetting<Relational<i64>>,
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
            && self.empty_string.is_load()
            && self.replay_timestamp.is_load()
            && self.negative_one.is_load()
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
            && self.empty_string.is_ignore()
            && self.replay_timestamp.is_ignore()
            && self.negative_one.is_ignore()
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
            || self.empty_string.is_ignore()
            || self.replay_timestamp.is_ignore()
            || self.negative_one.is_ignore()
            || self.online_score_id.is_ignore()
    }

    pub fn set_from_query(&mut self, matches: &ArgMatches) -> IoResult<()> {
        self.gameplay_mode = EqualCopy::from_matches(matches, "Gameplay mode")?.into();
        self.score_version = Relational::from_matches(matches, "Score version")?.into();
        self.md5_beatmap_hash = EqualClone::from_matches(matches, "MD5 beatmap hash")?.into();
        self.player_name = EqualClone::from_matches(matches, "Player name")?.into();
        self.md5_replay_hash = EqualClone::from_matches(matches, "MD5 replay hash")?.into();
        self.number_of_300s = Relational::from_matches(matches, "Number of 300s")?;
        self.number_of_100s = Relational::from_matches(
            matches,
            "Number of 100s (standard/CTB)/150s (taiko)/200s (mania)",
        )?
        .into();
        self.number_of_50s =
            Relational::from_matches(matches, "Number of 50s (standard/mania)/small fruit (CTB)")?
                .into();
        self.number_of_gekis =
            Relational::from_matches(matches, "Number of gekis/max 300s (mania)")?.into();
        self.number_of_katus =
            Relational::from_matches(matches, "Number of katus/100s (mania)")?.into();
        self.number_of_misses = Relational::from_matches(matches, "Number of misses")?.into();
        self.replay_score = Relational::from_matches(matches, "Replay score")?.into();
        self.max_combo = Relational::from_matches(matches, "Max combo")?.into();
        self.perfect_combo = EqualCopy::bool_from_matches(matches, "Perfect combo")?.into();
        self.mods_used = Relational::from_matches(matches, "Mods used")?.into();
        self.replay_timestamp = Relational::date_from_matches(matches, "Replay timestamp")?.into();
        self.online_score_id = Relational::from_matches(matches, "Online score ID")?.into();
        Ok(())
    }

    pub fn set_from_mask(&mut self, mask: ScoreMask) {
        if self.gameplay_mode.is_ignore() && mask.gameplay_mode {
            self.gameplay_mode = LoadSetting::Load;
        }
        if self.score_version.is_ignore() && mask.score_version {
            self.score_version = LoadSetting::Load;
        }
        if self.md5_beatmap_hash.is_ignore() && mask.md5_beatmap_hash {
            self.md5_beatmap_hash = LoadSetting::Load;
        }
        if self.player_name.is_ignore() && mask.player_name {
            self.player_name = LoadSetting::Load;
        }
        if self.md5_replay_hash.is_ignore() && mask.md5_replay_hash {
            self.md5_replay_hash = LoadSetting::Load;
        }
        if self.number_of_300s.is_ignore() && mask.number_of_300s {
            self.number_of_300s = LoadSetting::Load;
        }
        if self.number_of_100s.is_ignore() && mask.number_of_100s {
            self.number_of_100s = LoadSetting::Load;
        }
        if self.number_of_50s.is_ignore() && mask.number_of_50s {
            self.number_of_50s = LoadSetting::Load;
        }
        if self.number_of_gekis.is_ignore() && mask.number_of_gekis {
            self.number_of_gekis = LoadSetting::Load;
        }
        if self.number_of_katus.is_ignore() && mask.number_of_katus {
            self.number_of_katus = LoadSetting::Load;
        }
        if self.number_of_misses.is_ignore() && mask.number_of_misses {
            self.number_of_misses = LoadSetting::Load;
        }
        if self.replay_score.is_ignore() && mask.replay_score {
            self.replay_score = LoadSetting::Load;
        }
        if self.max_combo.is_ignore() && mask.max_combo {
            self.max_combo = LoadSetting::Load;
        }
        if self.perfect_combo.is_ignore() && mask.perfect_combo {
            self.perfect_combo = LoadSetting::Load;
        }
        if self.mods_used.is_ignore() && mask.mods_used {
            self.mods_used = LoadSetting::Load;
        }
        if self.empty_string.is_ignore() && mask.empty_string {
            self.empty_string = LoadSetting::Load;
        }
        if self.replay_timestamp.is_ignore() && mask.replay_timestamp {
            self.replay_timestamp = LoadSetting::Load;
        }
        if self.negative_one.is_ignore() && mask.negative_one {
            self.negative_one = LoadSetting::Load;
        }
        if self.online_score_id.is_ignore() && mask.online_score_id {
            self.online_score_id = LoadSetting::Load;
        }
    }
}
