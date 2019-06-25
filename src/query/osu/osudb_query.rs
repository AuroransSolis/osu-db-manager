use std::io::{Result as IoResult, Error as IoError, ErrorKind::InvalidInput};

use clap::{Arg, App, SubCommand, AppSettings, ArgGroup};

use crate::read_error::{ParseFileResult, DbFileParseError, ParseErrorKind::QueryError};
use crate::query::{AskCompareIgnore, Comparison, osu::beatmap_query::BeatmapQuery};
use crate::masks::osu_mask::OsuDbMask;

pub struct OsuDbQuery {
    pub version: bool,
    pub folder_count: bool,
    pub account_unlocked: bool,
    pub account_unlock_date: bool,
    pub player_name: bool,
    pub number_of_beatmaps: bool,
    pub beatmap_query: Option<BeatmapQuery>,
    pub unknown_int: bool
}

impl OsuDbQuery {
    fn from_arg(s: &str) -> IoResult<Self> {
        let mut args = Vec::new();
        for arg in s.split_whitespace() {
            let mut tmp = if arg.starts_with('-') {
                '-'.to_string()
            } else {
                String::new()
            };
            tmp += arg;
            args.push(tmp);
        }
        let matches = App::new("osu!.db query parser")
            .arg(Arg::with_name("Version")
                .long("VERSION")
                .required(false)
                .takes_value(false))
            .arg(Arg::with_name("Folder count")
                .long("FOLDER-COUNT")
                .required(false)
                .takes_value(false))
            .arg(Arg::with_name("Account unlocked")
                .long("ACCOUNT-UNLOCKED")
                .required(false)
                .takes_value(false))
            .arg(Arg::with_name("Account unlock date")
                .long("ACCOUNT-UNLOCK-DATE")
                .required(false)
                .takes_value(false))
            .arg(Arg::with_name("Player name")
                .long("PLAYER-NAME")
                .required(false)
                .takes_value(false))
            .arg(Arg::with_name("Number of beatmaps")
                .long("NUMBER-OF-BEATMAPS")
                .required(false)
                .takes_value(false))
            .arg(Arg::with_name("Beatmap query/filter")
                .long("BEATMAP-QUERY")
                .required(false)
                .require_equals(true)
                .takes_value(true)
                .multiple(false)
                .value_name("BEATMAP QUERY"))
            .arg(Arg::with_name("Unknown int")
                .long("UNKNOWN-INT")
                .required(false)
                .takes_value(false))
            .get_matches_from(args.into_iter());
        let [mut version, mut folder_count, mut account_unlocked, mut account_unlock_date,
            mut player_name, mut number_of_beatmaps, mut unknown_int] = [false; 7];
        let beatmap_query = if let Some(arg) = matches.value_of("Beatmap query/filter") {
            Some(BeatmapQuery::from_arg(arg)?)
        } else {
            None
        };
        if matches.is_present("Version") {
            version = true;
        }
        if matches.is_present("Folder count") {
            folder_count = true;
        }
        if matches.is_present("Account unlocked") {
            account_unlocked = true;
        }
        if matches.is_present("Account unlock date") {
            account_unlock_date = true;
        }
        if matches.is_present("Player name") {
            player_name = true;
        }
        if matches.is_present("Number of beatmaps") {
            number_of_beatmaps = true;
        }
        if matches.is_present("Unknown int") {
            unknown_int = true;
        }
        Ok(OsuDbQuery {
            version,
            folder_count,
            account_unlocked,
            account_unlock_date,
            player_name,
            number_of_beatmaps,
            beatmap_query,
            unknown_int
        })
    }
}