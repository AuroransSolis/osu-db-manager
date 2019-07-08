use std::io::Result as IoResult;

use chrono::NaiveDate;
use clap::{Arg, App, SubCommand, AppSettings, ArgMatches};

use crate::load_settings::{
    LoadSetting,
    SpecialArgType,
    parse_from_arg,
    parse_from_arg_special,
    query::QueryStruct
};
use crate::databases::osu::primitives::GameplayMode;
use crate::masks::scores_mask::ScoreMask;

pub struct ScoreLoadSettings {
    pub gameplay_mode: LoadSetting<GameplayMode>,
    pub score_version: LoadSetting<i32>,
    pub md5_beatmap_hash: LoadSetting<String>,
    pub player_name: LoadSetting<Option<String>>,
    pub md5_replay_hash: LoadSetting<String>,
    pub number_of_300s: LoadSetting<i16>,
    pub number_of_100s: LoadSetting<i16>,
    pub number_of_50s: LoadSetting<i16>,
    pub number_of_gekis: LoadSetting<i16>,
    pub number_of_katus: LoadSetting<i16>,
    pub number_of_misses: LoadSetting<i16>,
    pub replay_score: LoadSetting<i32>,
    pub max_combo: LoadSetting<i16>,
    pub perfect_combo: LoadSetting<bool>,
    pub mods_used: LoadSetting<i32>,
    pub empty_string: LoadSetting<Option<String>>,
    pub replay_timestamp: LoadSetting<NaiveDate>,
    pub negative_one: LoadSetting<i32>,
    pub online_score_id: LoadSetting<i64>
}

impl ScoreLoadSettings {
    pub fn load_all(&self) -> bool {
        !(self.gameplay_mode.is_ignore() || self.score_version.is_ignore()
            || self.md5_beatmap_hash.is_ignore() || self.player_name.is_ignore()
            || self.md5_replay_hash.is_ignore() || self.number_of_300s.is_ignore()
            || self.number_of_100s.is_ignore() || self.number_of_50s.is_ignore()
            || self.number_of_gekis.is_ignore() || self.number_of_katus.is_ignore()
            || self.number_of_misses.is_ignore() || self.replay_score.is_ignore()
            || self.max_combo.is_ignore() || self.perfect_combo.is_ignore()
            || self.mods_used.is_ignore() || self.empty_string.is_ignore()
            || self.replay_timestamp.is_ignore() || self.negative_one.is_ignore()
            || self.online_score_id.is_ignore())
    }

    pub fn set_from_query(&mut self, matches: &ArgMatches) -> IoResult<()> {
        self.gameplay_mode = parse_from_arg::<GameplayMode>(matches, "Gameplay mode")?;
        self.score_version = parse_from_arg::<i32>(matches, "Score version")?;
        self.md5_beatmap_hash = parse_from_arg_special::<String>(matches, "MD5 beatmap hash",
            SpecialArgType::String)?;
        self.player_name = parse_from_arg_special::<Option<String>>(matches, "Player name",
            SpecialArgType::OptionString)?;
        self.md5_replay_hash = parse_from_arg_special::<String>(matches, "MD5 replay hash",
            SpecialArgType::String)?;
        self.number_of_300s = parse_from_arg::<i16>(matches, "Number of 300s")?;
        self.number_of_100s = parse_from_arg::<i16>(matches,
            "Number of 100s (standard/CTB)/150s (taiko)/200s (mania)")?;
        self.number_of_50s = parse_from_arg::<i16>(matches,
            "Number of 50s (standard/mania)/small fruit (CTB)")?;
        self.number_of_gekis = parse_from_arg::<i16>(matches, "Number of gekis/max 300s (mania)")?;
        self.number_of_katus = parse_from_arg::<i16>(matches, "Number of katus/100s (mania)")?;
        self.number_of_misses = parse_from_arg::<i16>(matches, "Number of misses")?;
        self.replay_score = parse_from_arg::<i32>(matches, "Replay score")?;
        self.max_combo = parse_from_arg::<i16>(matches, "Max combo")?;
        self.perfect_combo = parse_from_arg_special::<bool>(matches, "Perfect combo",
            SpecialArgType::bool)?;
        self.mods_used = parse_from_arg::<i32>(matches, "Mods used")?;
        self.replay_timestamp = parse_from_arg_special::<NaiveDate>(matches, "Replay timestamp",
            SpecialArgType::NaiveDate)?;
        self.online_score_id = parse_from_arg::<i64>(matches, "Online score ID")?;
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