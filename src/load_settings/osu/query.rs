use std::io::{Result as IoResult, Error as IoError, ErrorKind::InvalidInput};

use clap::{Arg, App, SubCommand, AppSettings, ArgGroup};
use chrono::NaiveDate;

use crate::read_error::{ParseFileResult, DbFileParseError, ParseErrorKind::QueryError};
use crate::load_settings::{
    LoadSetting,
    Comparison,
    SpecialArgType,
    parse_from_arg,
    parse_from_arg_special,
    osu::{osudb_load_settings::OsuDbLoadSettings, beatmap_load_settings::BeatmapLoadSettings},
    query::QueryStruct
};
use crate::masks::osu_mask::{OsuDbMask, BeatmapMask};

impl QueryStruct<OsuDbMask> for OsuDbLoadSettings {
    fn load_all(&self) -> bool {
        !(self.beatmap_load_settings.load_all() || self.version.is_ignore()
            || self.folder_count.is_ignore() || self.account_unlocked.is_ignore()
            || self.account_unlock_date.is_ignore() || self.player_name.is_ignore()
            || self.unknown_int.is_ignore())
    }

    fn set_from_query(&mut self, args: Vec<&str>) -> IoResult<()> {
        self.beatmap_load_settings.set_from_args(args)?;
        Ok(())
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