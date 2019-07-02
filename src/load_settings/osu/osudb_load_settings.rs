use chrono::NaiveDate;

use crate::read_error::{ParseFileResult, DbFileParseError, ParseErrorKind::QueryError};
use crate::load_settings::{osu::beatmap_load_settings::BeatmapLoadSettings, query::QueryStruct, LoadSetting};

pub struct OsuDbLoadSettings {
    pub version: LoadSetting<i32>,
    pub folder_count: LoadSetting<i32>,
    pub account_unlocked: LoadSetting<bool>,
    pub account_unlock_date: LoadSetting<NaiveDate>,
    pub player_name: LoadSetting<Option<String>>,
    pub number_of_beatmaps: LoadSetting<i32>,
    pub beatmap_load_settings: BeatmapLoadSettings,
    pub unknown_int: LoadSetting<i32>
}

impl QueryStruct for OsuDbLoadSettings {}