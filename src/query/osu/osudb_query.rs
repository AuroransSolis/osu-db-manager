use std::io::{Result as IoResult, Error as IoError, ErrorKind::InvalidInput};

use clap::{Arg, App, SubCommand, AppSettings, ArgGroup};

use crate::read_error::{ParseFileResult, DbFileParseError, ParseErrorKind::QueryError};
use crate::query::{
    AskCompareIgnore,
    Comparison,
    osu::beatmap_query::BeatmapQuery,
    query::QueryStruct
};
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

impl QueryStruct for OsuDbQuery {}

impl OsuDbQuery {
    pub fn from_args(args: Vec<&str>) -> IoResult<Self> {
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
        let [mut version, mut folder_count, mut account_unlocked, mut account_unlock_date,
            mut player_name, mut number_of_beatmaps, mut unknown_int] = [false; 7];
        let version = matches.is_present("Version");
        let folder_count = matches.is_present("Folder count");
        let account_unlocked = matches.is_present("Account unlocked");
        let account_unlock_date = matches.is_present("Account unlock date");
        let player_name = matches.is_present("Player name");
        let number_of_beatmaps = matches.is_present("Number of beatmaps");
        let beatmap_query = if let Some(subcommand_matches) = matches.subcommand_matches("Beatmap query/filter") {
            get_parse_and_assign!(subcommand_matches {
                "Entry size", entry_size => i32;
                "Ranked status", ranked_status => RankedStatus;
                "Number of hitcircles", number_of_hitcircles => i16;
                "Number of sliders", number_of_sliders => i16;
                "Number of spinners", number_of_spinners => i16;
                "Approach rate", approach_rate => ByteSingle;
                "Circle size", circle_size => ByteSingle;
                "HP drain", hp_drain => ByteSingle;
                "Overall difficulty", overall_difficulty => ByteSingle;
                "Slider velocity", slider_velocity => f64;
                "Number of precalculated mod combo star ratings (standard)";
                    num_mod_combo_star_ratings_standard => i32;
                "Number of precalculated mod combo star ratings (taiko)";
                    num_mod_combo_star_ratings_taiko => i32;
                "Number of precalculated mod combo star ratings (CTB)";
                    num_mod_combo_star_ratings_ctb => i32;
                "Number of precalculated mod combo star ratings (mania)";
                    num_mod_combo_star_ratings_mania => i32;
                "Drain time", drain_time => i32;
                "Total time", total_time => i32;
                "Preview offset from start (ms)", preview_offset_from_start_ms => i32;
                "Number of timing points", num_timing_points, i32;
                "Beatmap ID", beatmap_id => i32;
                "Beatmap set ID", beatmap_set_id => i32;
                "Thread ID", thread_id => i32;
                "Standard grade", standard_grade => u8;
                "Taiko grade", taiko_grade => u8;
                "CTB grade", ctb_grade => u8;
                "Mania grade", mania_grade => u8;
                "Local offset", local_offset => i16;
                "Stack leniency", stack_leniency => f32;
                "Gameplay mode", gameplay_mode => GameplayMode;
                "Online offset", online_offset => i16;
                "Offset from song start in editor (ms)", offset_from_song_start_in_editor_ms => i32;
                "Mania scroll speed", mania_scroll_speed => u8;
            });
            get_and_assign_string!(subcommand_matches {
                "Artist name", artist_name;
                "Artist name unicode", artist_name_unicode;
                "Song title", song_title;
                "Song title unicode", song_title_unicode;
                "Creator name", creator_name;
                "Difficulty", difficulty;
                "Audio file name", audio_file_name;
                "MD5 beatmap hash", md5_beatmap_hash;
                ".osu file name", dotosu_file_name;
                "Beatmap folder name", beatmap_folder_name;
                "Song source", song_source => String;
                "Song tags", song_tags => String;
                "Font used for song title", font_used_for_song_title => String;
            });
            get_and_assign_datetime!(subcommand_matches {
                "Last modification time", last_modification_time;
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
            Some(BeatmapQuery {
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