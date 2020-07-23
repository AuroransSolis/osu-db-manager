use crate::databases::osu::primitives::GameplayMode;
use crate::load_settings::{EqualClone, EqualCopy, Relational};
use crate::masks::scores_mask::ScoreMask;
use chrono::NaiveDate;
use std::default::Default;
use structopt::StructOpt;

#[derive(Clone, StructOpt)]
pub struct ScoreLoadSettings {
    #[structopt(
        name = "gameplay mode",
        long = "gameplay-mode",
        value_name = "EQ-GAMEPLAY-MODE",
        default_value,
        parse(try_from_str)
    )]
    pub gameplay_mode: EqualCopy<GameplayMode>,
    #[structopt(
        name = "score version",
        long = "score-version",
        value_name = "RELATIONAL",
        default_value,
        parse(try_from_str)
    )]
    pub score_version: Relational<i32>,
    #[structopt(
        name = "score md5 beatmap hash",
        long = "score-md5-beatmap-hash",
        value_name = "EQ",
        default_value,
        parse(try_from_str)
    )]
    pub md5_beatmap_hash: EqualClone<String>,
    #[structopt(
        name = "player name",
        long = "player-name",
        value_name = "EQ",
        default_value,
        parse(try_from_str)
    )]
    pub player_name: EqualClone<String>,
    #[structopt(
        name = "md5 replay hash",
        long = "md5-replay-hash",
        value_name = "EQ",
        default_value,
        parse(try_from_str)
    )]
    pub md5_replay_hash: EqualClone<String>,
    #[structopt(
        name = "number of 300s",
        long = "number-of-300s",
        value_name = "RELATIONAL",
        default_value,
        parse(try_from_str)
    )]
    pub number_of_300s: Relational<i16>,
    #[structopt(
        name = "number of 100s",
        long = "number-of-100s",
        value_name = "RELATIONAL",
        default_value,
        parse(try_from_str)
    )]
    pub number_of_100s: Relational<i16>,
    #[structopt(
        name = "number of 50s",
        long = "number-of-50s",
        value_name = "RELATIONAL",
        default_value,
        parse(try_from_str)
    )]
    pub number_of_50s: Relational<i16>,
    #[structopt(
        name = "number of gekis",
        long = "number-of-gekis",
        value_name = "RELATIONAL",
        default_value,
        parse(try_from_str)
    )]
    pub number_of_gekis: Relational<i16>,
    #[structopt(
        name = "number of katus",
        long = "number-of-katus",
        value_name = "RELATIONAL",
        default_value,
        parse(try_from_str)
    )]
    pub number_of_katus: Relational<i16>,
    #[structopt(
        name = "number of misses",
        long = "number-of-misses",
        value_name = "RELATIONAL",
        default_value,
        parse(try_from_str)
    )]
    pub number_of_misses: Relational<i16>,
    #[structopt(
        name = "replay score",
        long = "replay-score",
        value_name = "RELATIONAL",
        default_value,
        parse(try_from_str)
    )]
    pub replay_score: Relational<i32>,
    #[structopt(
        name = "max combo",
        long = "max-combo",
        value_name = "RELATIONAL",
        default_value,
        parse(try_from_str)
    )]
    pub max_combo: Relational<i16>,
    #[structopt(
        name = "perfect combo",
        long = "perfect-combo",
        value_name = "EQ-BOOL",
        possible_values(&["t", "true", "y", "yes", "1", "f", "false", "n", "no", "0"]),
        default_value,
        parse(try_from_str)
    )]
    pub perfect_combo: EqualCopy<bool>,
    #[structopt(
        name = "mods used",
        long = "mods-used",
        value_name = "RELATIONAL",
        default_value,
        parse(try_from_str)
    )]
    pub mods_used: Relational<i32>,
    #[structopt(skip)]
    pub empty_string: bool,
    #[structopt(
        name = "",
        long = "",
        value_name = "RELATIONAL-DATE",
        default_value,
        parse(try_from_str)
    )]
    pub replay_timestamp: Relational<NaiveDate>,
    #[structopt(skip)]
    pub negative_one: bool,
    #[structopt(
        name = "",
        long = "",
        value_name = "RELATIONAL",
        default_value,
        parse(try_from_str)
    )]
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

    pub fn set_from_mask(&mut self, mask: &ScoreMask) {
        self.gameplay_mode.apply_mask(mask.gameplay_mode);
        self.score_version.apply_mask(mask.score_version);
        self.md5_beatmap_hash
            .apply_mask(mask.score_md5_beatmap_hash);
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

impl Default for ScoreLoadSettings {
    fn default() -> Self {
        ScoreLoadSettings {
            gameplay_mode: EqualCopy::default(),
            score_version: Relational::default(),
            md5_beatmap_hash: EqualClone::default(),
            player_name: EqualClone::default(),
            md5_replay_hash: EqualClone::default(),
            number_of_300s: Relational::default(),
            number_of_100s: Relational::default(),
            number_of_50s: Relational::default(),
            number_of_gekis: Relational::default(),
            number_of_katus: Relational::default(),
            number_of_misses: Relational::default(),
            replay_score: Relational::default(),
            max_combo: Relational::default(),
            perfect_combo: EqualCopy::default(),
            mods_used: Relational::default(),
            empty_string: bool::default(),
            replay_timestamp: Relational::default(),
            negative_one: bool::default(),
            online_score_id: Relational::default(),
        }
    }
}
