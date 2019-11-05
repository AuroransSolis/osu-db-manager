use std::io::Result as IoResult;

use chrono::NaiveDate;

use crate::load_settings::{
    osu::beatmap_load_settings::BeatmapLoadSettings, query::QueryStruct, Empty, LoadSetting,
};
use crate::masks::osu_mask::OsuDbMask;
use crate::read_error::{DbFileParseError, ParseErrorKind::QueryError, ParseFileResult};

pub struct OsuDbLoadSettings {
    pub version: LoadSetting<Empty>,
    pub folder_count: LoadSetting<Empty>,
    pub account_unlocked: LoadSetting<Empty>,
    pub account_unlock_date: LoadSetting<Empty>,
    pub player_name: LoadSetting<Empty>,
    pub number_of_beatmaps: LoadSetting<Empty>,
    pub beatmap_load_settings: BeatmapLoadSettings,
    pub unknown_int: LoadSetting<Empty>,
}

impl Default for OsuDbLoadSettings {
    fn default() -> Self {
        OsuDbLoadSettings {
            version: LoadSetting::Ignore,
            folder_count: LoadSetting::Ignore,
            account_unlocked: LoadSetting::Ignore,
            account_unlock_date: LoadSetting::Ignore,
            player_name: LoadSetting::Ignore,
            number_of_beatmaps: LoadSetting::Ignore,
            beatmap_load_settings: BeatmapLoadSettings::default(),
            unknown_int: LoadSetting::Ignore,
        }
    }
}

impl QueryStruct<OsuDbMask> for OsuDbLoadSettings {
    fn load_all(&self) -> bool {
        self.beatmap_load_settings.load_all()
            && self.version.is_load()
            && self.folder_count.is_load()
            && self.account_unlocked.is_load()
            && self.account_unlock_date.is_load()
            && self.player_name.is_load()
            && self.unknown_int.is_load()
    }

    fn ignore_all(&self) -> bool {
        self.beatmap_load_settings.ignore_all()
            && self.version.is_ignore()
            && self.folder_count.is_ignore()
            && self.account_unlocked.is_ignore()
            && self.account_unlock_date.is_ignore()
            && self.player_name.is_ignore()
            && self.unknown_int.is_ignore()
    }

    fn is_partial(&self) -> bool {
        self.beatmap_load_settings.is_partial()
            || self.version.is_ignore()
            || self.folder_count.is_ignore()
            || self.account_unlocked.is_ignore()
            || self.account_unlock_date.is_ignore()
            || self.player_name.is_ignore()
            || self.unknown_int.is_ignore()
    }

    fn set_from_query(&mut self, args: Vec<&str>) -> IoResult<()> {
        self.beatmap_load_settings.set_from_query(args)
    }

    fn set_from_mask(&mut self, mask: &OsuDbMask) {
        if self.version.is_ignore() && mask.version {
            self.version = LoadSetting::Load;
        }
        if self.folder_count.is_ignore() && mask.folder_count {
            self.folder_count = LoadSetting::Load;
        }
        if self.account_unlocked.is_ignore() && mask.account_unlocked {
            self.account_unlocked = LoadSetting::Load;
        }
        if self.account_unlock_date.is_ignore() && mask.account_unlock_date {
            self.account_unlock_date = LoadSetting::Load;
        }
        if self.player_name.is_ignore() && mask.player_name {
            self.player_name = LoadSetting::Load;
        }
        if self.number_of_beatmaps.is_ignore() && mask.number_of_beatmaps {
            self.player_name = LoadSetting::Load;
        }
        if let Some(beatmap_mask) = mask.beatmap_mask.as_ref() {
            self.beatmap_load_settings.set_from_mask(beatmap_mask);
        }
        if self.unknown_int.is_ignore() && mask.unknown_int {
            self.unknown_int = LoadSetting::Load;
        }
    }
}
