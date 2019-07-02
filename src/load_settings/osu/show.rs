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
    osu::{osudb_load_settings::OsuDbLoadSetting, beatmap_load_settings::BeatmapLoadSetting}
};
use crate::databases::osu::primitives::{RankedStatus, ByteSingle, GameplayMode};
use crate::masks::osu_mask::{OsuDbMask, BeatmapMask};

impl OsuDbMask {
    pub(crate) fn from_show_args(args: Vec<&str>) -> Self {
        if args.len() == 0 {
            return Self::default();
        }

    }
}