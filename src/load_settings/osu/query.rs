use std::io::{Result as IoResult, Error as IoError, ErrorKind::InvalidInput};

use clap::{Arg, App, SubCommand, AppSettings, ArgGroup};
use chrono::NaiveDate;

use crate::read_error::{ParseFileResult, DbFileParseError, ParseErrorKind::QueryError};
use crate::load_settings::{
    LoadSetting,
    Comparison,
    SpecialArgType,
    parse_from_arg_str,
    parse_from_arg_special,
    osu::{osudb_load_settings::OsuDbLoadSettings, beatmap_load_settings::BeatmapLoadSettings}
};
use crate::databases::osu::primitives::{RankedStatus, ByteSingle, GameplayMode};

impl OsuDbLoadSettings {
    pub fn query_args(mut self, args: Vec<&str>) -> IoResult<Self> {
        if args.len() == 0 {
            return Ok(self);
        }
        let matches = App::new("osu!.db query parser")
            .arg(Arg::with_name("Version")
                .long("VERSION")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name("Folder count")
                .long("FOLDER-COUNT")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name("Account unlocked")
                .long("ACCOUNT-UNLOCKED")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name("Account unlock date")
                .long("ACCOUNT-UNLOCK-DATE")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name("Player name")
                .long("PLAYER-NAME")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name("Number of beatmaps")
                .long("NUMBER-OF-BEATMAPS")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .subcommand(SubCommand::with_name("query")
                .arg(Arg::with_name("Entry size")
                    .long("ENTRY-SIZE")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("SIZE"))
                .arg(Arg::with_name("Artist name")
                    .long("ARTIST-NAME")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NAME"))
                .arg(Arg::with_name("Artist name unicode")
                    .long("ARTIST-NAME-UNICODE")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NAME"))
                .arg(Arg::with_name("Song title")
                    .long("SONG-TITLE")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("TITLE"))
                .arg(Arg::with_name("Song title unicode")
                    .long("SONG-TITLE-UNICODE")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("TITLE"))
                .arg(Arg::with_name("Creator name")
                    .long("CREATOR-NAME")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NAME"))
                .arg(Arg::with_name("Difficulty")
                    .long("DIFFICULTY")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NAME")
                    .takes_value(false))
                .arg(Arg::with_name("Audio file name")
                    .long("AUDIO-FILE-NAME")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("FILENAME")
                    .takes_value(false))
                .arg(Arg::with_name("MD5 beatmap hash")
                    .long("MD5-BEATMAP-HASH")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("HASH")
                    .takes_value(false))
                .arg(Arg::with_name(".osu file name")
                    .long("DOTOSU-FILE-NAME")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("FILENAME")
                    .takes_value(false))
                .arg(Arg::with_name("Ranked status")
                    .long("RANKED-STATUS")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("STATUS")
                    .takes_value(false))
                .arg(Arg::with_name("Number of hitcircles")
                    .long("NUMBER-OF-HITCIRCLES")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NUMBER/RANGE")
                    .takes_value(false))
                .arg(Arg::with_name("Number of sliders")
                    .long("NUMBER-OF-SLIDERS")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NUMBER/RANGE")
                    .takes_value(false))
                .arg(Arg::with_name("Number of spinners")
                    .long("NUMBER-OF-SPINNERS")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NUMBER/RANGE")
                    .takes_value(false))
                .arg(Arg::with_name("Last modification time")
                    .long("LAST-MODIFICATION-TIME")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("DATE")
                    .takes_value(false))
                .arg(Arg::with_name("Approach rate")
                    .long("APPROACH-RATE")
                    .multiple(false)
                    .short("AR")
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NUMBER/RANGE")
                    .takes_value(false))
                .arg(Arg::with_name("Circle size")
                    .long("CIRCLE-SIZE")
                    .multiple(false)
                    .short("CS")
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NUMBER/RANGE")
                    .takes_value(false))
                .arg(Arg::with_name("HP drain")
                    .long("HP-DRAIN")
                    .multiple(false)
                    .short("HP")
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NUMBER/RANGE")
                    .takes_value(false))
                .arg(Arg::with_name("Overall difficulty")
                    .long("OVERALL-DIFFICULTY")
                    .multiple(false)
                    .short("OD")
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NUMBER/RANGE")
                    .takes_value(false))
                .arg(Arg::with_name("Slider velocity")
                    .long("SLIDER-VELOCITY")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NUMBER/RANGE")
                    .takes_value(false))
                .arg(Arg::with_name("Number of precalculated mod combo star ratings (standard)")
                    .long("NUM-MOD-COMBO-STAR-RATINGS-STANDARD")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NUMBER/RANGE")
                    .takes_value(false))
                .arg(Arg::with_name("Number of precalculated mod combo star ratings (taiko)")
                    .long("NUM-MOD-COMBO-STAR-RATINGS-TAIKO")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NUMBER/RANGE")
                    .takes_value(false))
                .arg(Arg::with_name("Number of precalculated mod combo star ratings (CTB)")
                    .long("NUM-MOD-COMBO-STAR-RATINGS-CTB")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("")
                    .takes_value(false))
                .arg(Arg::with_name("Number of precalculated mod combo star ratings (mania)")
                    .long("NUM-MOD-COMBO-STAR-RATINGS-MANIA")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NUMBER/RANGE")
                    .takes_value(false))
                .arg(Arg::with_name("Drain time")
                    .long("DRAIN-TIME")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NUMBER/RANGE")
                    .takes_value(false))
                .arg(Arg::with_name("Total time")
                    .long("TOTAL-TIME")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NUMBER/RANGE")
                    .takes_value(false))
                .arg(Arg::with_name("Preview offset from start (ms)")
                    .long("PREVIEW-OFFSET-FROM-START-MS")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NUMBER/RANGE")
                    .takes_value(false))
                .arg(Arg::with_name("Number of timing points")
                    .long("NUM-TIMING-POINTS")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NUMBER/RANGE")
                    .takes_value(false))
                .arg(Arg::with_name("Beatmap ID")
                    .long("BEATMAP-ID")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NUMBER/RANGE")
                    .takes_value(false))
                .arg(Arg::with_name("Beatmap set ID")
                    .long("BEATMAP-SET-ID")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NUMBER/RANGE")
                    .takes_value(false))
                .arg(Arg::with_name("Thread ID")
                    .long("THREAD-ID")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NUMBER/RANGE")
                    .takes_value(false))
                .arg(Arg::with_name("Standard grade")
                    .long("STANDARD-GRADE")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NUMBER/RANGE")
                    .takes_value(false))
                .arg(Arg::with_name("Taiko grade")
                    .long("TAIKO-GRADE")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NUMBER/RANGE")
                    .takes_value(false))
                .arg(Arg::with_name("CTB grade")
                    .long("CTB-GRADE")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NUMBER/RANGE")
                    .takes_value(false))
                .arg(Arg::with_name("Mania grade")
                    .long("MANIA-GRADE")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NUMBER/RANGE")
                    .takes_value(false))
                .arg(Arg::with_name("Local offset")
                    .long("LOCAL-OFFSET")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NUMBER/RANGE")
                    .takes_value(false))
                .arg(Arg::with_name("Stack leniency")
                    .long("STACK-LENIENCY")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NUMBER/RANGE")
                    .takes_value(false))
                .arg(Arg::with_name("Gameplay mode")
                    .long("GAMEPLAY-MODE")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("MODE")
                    .takes_value(false))
                .arg(Arg::with_name("Song source")
                    .long("SONG-SOURCE")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("SOURCE")
                    .takes_value(false))
                .arg(Arg::with_name("Song tags")
                    .long("SONG-TAGS")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("TAGS")
                    .takes_value(false))
                .arg(Arg::with_name("Online offset")
                    .long("ONLINE-OFFSET")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NUMBER/RANGE")
                    .takes_value(false))
                .arg(Arg::with_name("Font used for song title")
                    .long("FONT-USED-FOR-SONG-TITLE")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("FONT")
                    .takes_value(false))
                .arg(Arg::with_name("Unplayed")
                    .long("UNPLAYED")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("")
                    .takes_value(false))
                .arg(Arg::with_name("Last played")
                    .long("LAST-PLAYED")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("DATE")
                    .takes_value(false))
                .arg(Arg::with_name("Is OSZ2")
                    .long("IS-OSZ2")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("T/F")
                    .takes_value(false))
                .arg(Arg::with_name("Beatmap folder name")
                    .long("BEATMAP-FOLDER-NAME")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NAME")
                    .takes_value(false))
                .arg(Arg::with_name("Last checked against repo")
                    .long("LAST-CHECKED-AGAINST-REPO")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("DATE")
                    .takes_value(false))
                .arg(Arg::with_name("Ignore beatmap sound")
                    .long("IGNORE-BEATMAP-SOUND")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("T/F")
                    .takes_value(false))
                .arg(Arg::with_name("Ignore beatmap skin")
                    .long("IGNORE-BEATMAP-SKIN")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("T/F")
                    .takes_value(false))
                .arg(Arg::with_name("Disable storyboard")
                    .long("DISABLE-STORYBOARD")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("T/F")
                    .takes_value(false))
                .arg(Arg::with_name("Disable video")
                    .long("DISABLE-VIDEO")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("T/F")
                    .takes_value(false))
                .arg(Arg::with_name("Visual override")
                    .long("VISUAL-OVERRIDE")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("T/F")
                    .takes_value(false))
                .arg(Arg::with_name("Unknown short")
                    .long("UNKNOWN-SHORT")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NUMBER/RANGE")
                    .takes_value(false))
                .arg(Arg::with_name("Offset from song start in editor (ms)")
                    .long("OFFSET-FROM-SONG-START-IN-EDITOR-MS")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NUMBER/RANGE")
                    .takes_value(false))
                .arg(Arg::with_name("Mania scroll speed")
                    .long("MANIA-SCROLL-SPEED")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NUMBER/RANGE")
                    .takes_value(false)))
            .arg(Arg::with_name("Unknown int")
                .long("UNKNOWN-INT")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .get_matches_from(args.into_iter());
        if !self.version.is_ignore() {
            self.version = parse_from_arg_str::<i32>(&matches, "Version")?;
        }
        if !self.folder_count.is_ignore() {
            self.folder_count = parse_from_arg_str::<i32>(&matches, "Folder count")?;
        }
        if !self.account_unlocked.is_ignore() {
            self.account_unlocked = parse_from_arg_special::<bool>(&matches, "Account unlocked",
                SpecialArgType::bool)?;
        }
        if !self.account_unlock_date.is_ignore() {
            self.account_unlock_date = read_from_arg::<NaiveDate>(&matches, "Account unlock date",
                ArgType::NaiveDate)?;
        }
        if !self.player_name.is_ignore() {
            self.player_name = read_from_arg::<Option<String>>(&matches, "Player name",
                ArgType::OptionString)?;
        }
        if !self.number_of_beatmaps.is_ignore() {
            self.number_of_beatmaps = read_from_arg::<i32>(&matches, "Number of beatmaps",
                ArgType::i32)?;
        }
        if let Some(subcommand_matches) = matches.subcommand_matches("Beatmap query/filter") {
            if !self.beatmap_load_settings.entry_size.is_ignore() {
                self.entry_size = read_from_arg::<i32>(&subcommand_matches, "Entry size",
                    ArgType::i32)?;
            }
            if !self.beatmap_load_settings.artist_name.is_ignore() {
                self.artist_name = read_from_arg::<Option<String>>(&subcommand_matches,
                    "Artist name", ArgType::OptionString)?;
            }
            if !self.beatmap_load_settings.artist_name_unicode.is_ignore() {
                self.artist_name_unicode = read_from_arg::<Option<String>>(&subcommand_matches,
                    "Artist name unicode", ArgType::OptionString)?;
            }
            if !self.beatmap_load_settings.song_title.is_ignore() {
                self.song_title = read_from_arg::<Option<String>>(&subcommand_matches, "Song title",
                    ArgType::OptionString)?;
            }
            if !self.beatmap_load_settings.song_title_unicode.is_ignore() {
                self.song_title_unicode = read_from_arg::<Option<String>>(&subcommand_matches,
                    "Song title unicode", ArgType::OptionString)?;
            }
            if !self.beatmap_load_settings.creator_name.is_ignore() {
                self.creator_name = read_from_arg::<Option<String>>(&subcommand_matches,
                    "Creator name", ArgType::OptionString)?;
            }
            if !self.beatmap_load_settings.difficulty.is_ignore() {
                self.difficulty = read_from_arg::<Option<String>>(&subcommand_matches, "Difficulty",
                    ArgType::OptionString)?;
            }
            if !self.beatmap_load_settings.audio_file_name.is_ignore() {
                self.audio_file_name = read_from_arg::<Option<String>>(&subcommand_matches,
                    "Audio file name", ArgType::OptionString)?;
            }
            if !self.beatmap_load_settings.md5_beatmap_hash.is_ignore() {
                self.md5_beatmap_hash = read_from_arg::<String>(&subcommand_matches,
                    "MD5 beatmap hash", ArgType::String)?;
            }
            if !self.beatmap_load_settings.dotosu_file_name.is_ignore() {
                self.dotosu_file_name = read_from_arg::<Option<String>>(&subcommand_matches,
                    ".osu file name", ArgType::OptionString)?;
            }
            if !self.beatmap_load_settings.ranked_status.is_ignore() {
                self.ranked_status = read_from_arg::<RankedStatus>(&subcommand_matches,
                    "Ranked status", ArgType::RankedStatus)?;
            }
            if !self.beatmap_load_settings.number_of_hitcircles.is_ignore() {
                self.number_of_hitcircles = read_from_arg::<i16>(&subcommand_matches,
                    "Number of hitcircles", ArgType::i16)?;
            }
            if !self.beatmap_load_settings.number_of_sliders.is_ignore() {
                self.number_of_sliders = read_from_arg::<i16>(&subcommand_matches,
                    "Number of sliders", ArgType::i16)?;
            }
            if !self.beatmap_load_settings.number_of_spinners.is_ignore() {
                self.number_of_spinners = read_from_arg::<i16>(&subcommand_matches,
                    "Number of spinners", ArgType::i16)?;
            }
            if !self.beatmap_load_settings.last_modification_time.is_ignore() {
                self.last_modification_time = read_from_arg::<NaiveDate>(&subcommand_matches,
                    "Last modification time", ArgType::NaiveDate)?;
            }
            if !self.beatmap_load_settings.approach_rate.is_ignore() {
                self.approach_rate = read_from_arg::<ByteSingle>(&subcommand_matches,
                    "Approach rate", ArgType::ByteSingle)?;
            }
            if !self.beatmap_load_settings.circle_size.is_ignore() {
                self.circle_size = read_from_arg::<ByteSingle>(&subcommand_matches, "Circle size",
                    ArgType::ByteSingle)?;
            }
            if !self.beatmap_load_settings.hp_drain.is_ignore() {
                self.hp_drain = read_from_arg::<ByteSingle>(&subcommand_matches, "HP drain",
                    ArgType::ByteSingle)?;
            }
            if !self.beatmap_load_settings.overall_difficulty.is_ignore() {
                self.overall_difficulty = read_from_arg::<ByteSingle>(&subcommand_matches,
                    "Overall difficulty", ArgType::ByteSingle)?;
            }
            if !self.beatmap_load_settings.slider_velocity.is_ignore() {
                self.slider_velocity = read_from_arg::<f64>(&subcommand_matches,
                    "Slider velocity", ArgType::f64)?;
            }
            if !self.beatmap_load_settings.num_mod_combo_star_ratings_standard.is_ignore() {
                self.num_mod_combo_star_ratings_standard = read_from_arg::<i32>(&subcommand_matches,
                    "Number of precalculated mod combo star ratings (standard)", ArgType::i32)?;
            }
            if !self.beatmap_load_settings.num_mod_combo_star_ratings_taiko.is_ignore() {
                self.num_mod_combo_star_ratings_taiko = read_from_arg::<i32>(&subcommand_matches,
                    "Number of precalculated mod combo star ratings (taiko)", ArgType::i32)?;
            }
            if !self.beatmap_load_settings.num_mod_combo_star_ratings_ctb.is_ignore() {
                self.num_mod_combo_star_ratings_ctb = read_from_arg::<i32>(&subcommand_matches,
                    "Number of precalculated mod combo star ratings (CTB)", ArgType::i32)?;
            }
            if !self.beatmap_load_settings.num_mod_combo_star_ratings_mania.is_ignore() {
                self.num_mod_combo_star_ratings_mania = read_from_arg::<i32>(&subcommand_matches,
                    "Number of precalculated mod combo star ratings (mania)", ArgType::i32)?;
            }
            if !self.beatmap_load_settings.drain_time.is_ignore() {
                self.drain_time = read_from_arg::<i32>(&subcommand_matches, "Drain time",
                    ArgType::i32)?;
            }
            if !self.beatmap_load_settings.total_time.is_ignore() {
                self.total_time = read_from_arg::<i32>(&subcommand_matches, "Total time",
                    ArgType::i32)?;
            }
            if !self.beatmap_load_settings.preview_offset_from_start_ms.is_ignore() {
                self.preview_offset_from_start_ms = read_from_arg::<i32>(&subcommand_matches,
                    "Preview offset from start (ms)", ArgType::i32)?;
            }
            if !self.beatmap_load_settings.num_timing_points.is_ignore() {
                self.num_timing_points = read_from_arg::<i32>(&subcommand_matches,
                    "Number of timing points", ArgType::i32)?;
            }
            if !self.beatmap_load_settings.beatmap_id.is_ignore() {
                self.beatmap_id = read_from_arg::<i32>(&subcommand_matches, "Beatmap ID",
                    ArgType::i32)?;
            }
            if !self.beatmap_load_settings.beatmap_set_id.is_ignore() {
                self.beatmap_set_id = read_from_arg::<i32>(&subcommand_matches, "Beatmap set ID",
                    ArgType::i32)?;
            }
            if !self.beatmap_load_settings.thread_id.is_ignore() {
                self.thread_id = read_from_arg::<i32>(&subcommand_matches, "Thread ID",
                    ArgType::i32)?;
            }
            if !self.beatmap_load_settings.standard_grade.is_ignore() {
                self.standard_grade = read_from_arg::<u8>(&subcommand_matches, "Standard grade",
                    ArgType::u8)?;
            }
            if !self.beatmap_load_settings.taiko_grade.is_ignore() {
                self.taiko_grade = read_from_arg::<u8>(&subcommand_matches, "Taiko grade",
                    ArgType::u8)?;
            }
            if !self.beatmap_load_settings.ctb_grade.is_ignore() {
                self.ctb_grade = read_from_arg::<u8>(&subcommand_matches, "CTB grade",
                    ArgType::u8)?;
            }
            if !self.beatmap_load_settings.mania_grade.is_ignore() {
                self.mania_grade = read_from_arg::<u8>(&subcommand_matches, "Mania grade",
                    ArgType::u8)?;
            }
            if !self.beatmap_load_settings.local_offset.is_ignore() {
                self.local_offset = read_from_arg::<i16>(&subcommand_matches, "Local offset",
                    ArgType::i16)?;
            }
            if !self.beatmap_load_settings.stack_leniency.is_ignore() {
                self.stack_leniency = read_from_arg::<f32>(&subcommand_matches, "Stack leniency",
                    ArgType::f32)?;
            }
            if !self.beatmap_load_settings.gameplay_mode.is_ignore() {
                self.gameplay_mode = read_from_arg::<GameplayMode>(&subcommand_matches, "Gameplay mode", ArgType::GameplayMode)?;
            }
            if !self.beatmap_load_settings.song_source.is_ignore() {
                self.song_source = read_from_arg::<Option<String>>(&subcommand_matches,
                    "Song source", ArgType::OptionString)?;
            }
            if !self.beatmap_load_settings.song_tags.is_ignore() {
                self.song_tags = read_from_arg::<Option<String>>(&subcommand_matches, "Song tags",
                    ArgType::OptionString)?;
            }
            if !self.beatmap_load_settings.online_offset.is_ignore() {
                self.online_offset = read_from_arg::<i16>(&subcommand_matches, "", ArgType::i16)?;
            }
            if !self.beatmap_load_settings.font_used_for_song_title.is_ignore() {
                self.font_used_for_song_title = read_from_arg::<Option<String>>(&subcommand_matches, "", ArgType::OptionString)?;
            }
            if !self.beatmap_load_settings.unplayed.is_ignore() {
                self.unplayed = read_from_arg::<bool>(&subcommand_matches, "", ArgType::bool)?;
            }
            if !self.beatmap_load_settings.last_played.is_ignore() {
                self.last_played = read_from_arg::<NaiveDate>(&subcommand_matches, "", ArgType::NaiveDate)?;
            }
            if !self.beatmap_load_settings.is_osz2.is_ignore() {
                self.is_osz2 = read_from_arg::<bool>(&subcommand_matches, "", ArgType::bool)?;
            }
            if !self.beatmap_load_settings.beatmap_folder_name.is_ignore() {
                self.beatmap_folder_name = read_from_arg::<Option<String>>(&subcommand_matches, "", ArgType::OptionString)?;
            }
            if !self.beatmap_load_settings.last_checked_against_repo.is_ignore() {
                self.last_checked_against_repo = read_from_arg::<NaiveDate>(&subcommand_matches, "", ArgType::NaiveDate)?;
            }
            if !self.beatmap_load_settings.ignore_beatmap_sound.is_ignore() {
                self.ignore_beatmap_sound = read_from_arg::<bool>(&subcommand_matches, "", ArgType::bool)?;
            }
            if !self.beatmap_load_settings.ignore_beatmap_skin.is_ignore() {
                self.ignore_beatmap_skin = read_from_arg::<bool>(&subcommand_matches, "", ArgType::bool)?;
            }
            if !self.beatmap_load_settings.disable_storyboard.is_ignore() {
                self.disable_storyboard = read_from_arg::<bool>(&subcommand_matches, "", ArgType::bool)?;
            }
            if !self.beatmap_load_settings.disable_video.is_ignore() {
                self.disable_video = read_from_arg::<bool>(&subcommand_matches, "", ArgType::bool)?;
            }
            if !self.beatmap_load_settings.visual_override.is_ignore() {
                self.visual_override = read_from_arg::<bool>(&subcommand_matches, "", ArgType::bool)?;
            }
            if !self.beatmap_load_settings.offset_from_song_start_in_editor_ms.is_ignore() {
                self.offset_from_song_start_in_editor_ms = read_from_arg::<i32>(&subcommand_matches,
                    "", ArgType::i32)?;
            }
            if !self.beatmap_load_settings.mania_scroll_speed.is_ignore() {
                self.mania_scroll_speed = read_from_arg::<u8>(&subcommand_matches, "", ArgType::u8)?;
            }
        }
        let beatmap_query = if let Some(subcommand_matches) = matches.subcommand_matches("Beatmap query/filter") {
            get_parse_and_assign!(subcommand_matches {
                "Online offset", online_offset => i16;
                "Offset from song start in editor (ms)", offset_from_song_start_in_editor_ms => i32;
                "Mania scroll speed", mania_scroll_speed => u8;
            });
            get_and_assign_string!(subcommand_matches {
                "Beatmap folder name", beatmap_folder_name;
                "Font used for song title", font_used_for_song_title => String;
            });
            get_and_assign_datetime!(subcommand_matches {
                "Last played", last_played;
                "Last checked against repo", last_checked_against_repo;
            });
            get_and_assign_bool!(subcommand_matches {
                "Unplayed", unplayed;
                "Is OSZ2", is_osz2;
                "Ignore beatmap sound", ignore_beatmap_sound;
                "Ignore beatmap skin", ignore_beatmap_skin;
                "Disable storyboard", disable_storyboard;
                "Disable video", disable_video;
                "Visual override", visual_override;
            });
            Some(BeatmapLoadSettings {
                entry_size,
                artist_name,
                artist_name_unicode,
                song_title,
                song_title_unicode,
                creator_name,
                difficulty,
                audio_file_name,
                md5_beatmap_hash,
                dotosu_file_name,
                ranked_status,
                number_of_hitcircles,
                number_of_sliders,
                number_of_spinners,
                last_modification_time,
                approach_rate,
                circle_size,
                hp_drain,
                overall_difficulty,
                slider_velocity,
                num_mod_combo_star_ratings_standard,
                num_mod_combo_star_ratings_taiko,
                num_mod_combo_star_ratings_ctb,
                num_mod_combo_star_ratings_mania,
                drain_time,
                total_time,
                preview_offset_from_start_ms,
                num_timing_points,
                beatmap_id,
                beatmap_set_id,
                thread_id,
                standard_grade,
                taiko_grade,
                ctb_grade,
                mania_grade,
                local_offset,
                stack_leniency,
                gameplay_mode,
                song_source,
                song_tags,
                online_offset,
                font_used_for_song_title,
                unplayed,
                last_played,
                is_osz2,
                beatmap_folder_name,
                last_checked_against_repo,
                ignore_beatmap_sound,
                ignore_beatmap_skin,
                disable_storyboard,
                disable_video,
                visual_override,
                offset_from_song_start_in_editor_ms,
                mania_scroll_speed
            })
        } else {
            None
        };
        if matches.is_present("Unknown int") {
            unknown_int = true;
        }
        Ok(self)
    }
}