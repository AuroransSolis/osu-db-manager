use std::io::{Result as IoResult, Error as IoError, ErrorKind::InvalidInput};
use std::time::SystemTime;

use clap::{Arg, App, SubCommand, AppSettings, ArgGroup};

use crate::query::Comparison;
use crate::databases::osu::primitives::{RankedStatus, ByteSingle, GameplayMode};

macro_rules! get_matches_and_assign {
    ($matches:ident { $($arg_name:literal, $var:ident => $t:ty);+ }) => {
        $(
            if let Some(m) = $matches.value_of($arg_name) {
                $var = Some(Comparison::from_str(m)?);
            }
        )*
    }
}

pub struct BeatmapQuery {
    pub entry_size: Option<Comparison<i32>>,
    pub artist_name: Option<Comparison<String>>,
    pub artist_name_unicode: Option<Comparison<String>>,
    pub song_title: Option<Comparison<String>>,
    pub song_title_unicode: Option<Comparison<String>>,
    pub creator_name: Option<Comparison<String>>,
    pub difficulty: Option<Comparison<String>>,
    pub audio_file_name: Option<Comparison<String>>,
    pub md5_beatmap_hash: Option<Comparison<String>>,
    pub dotosu_file_name: Option<Comparison<String>>,
    pub ranked_status: Option<Comparison<RankedStatus>>,
    pub number_of_hitcircles: Option<Comparison<i16>>,
    pub number_of_sliders: Option<Comparison<i16>>,
    pub number_of_spinners: Option<Comparison<i16>>,
    pub last_modification_time: Option<Comparison<SystemTime>>,
    pub approach_rate: Option<Comparison<ByteSingle>>,
    pub circle_size: Option<Comparison<ByteSingle>>,
    pub hp_drain: Option<Comparison<ByteSingle>>,
    pub overall_difficulty: Option<Comparison<ByteSingle>>,
    pub slider_velocity: Option<Comparison<f64>>,
    pub num_mod_combo_star_ratings_standard: Option<Comparison<i32>>,
    pub num_mod_combo_star_ratings_taiko: Option<Comparison<i32>>,
    pub num_mod_combo_star_ratings_ctb: Option<Comparison<i32>>,
    pub num_mod_combo_star_ratings_mania: Option<Comparison<i32>>,
    pub drain_time: Option<Comparison<i32>>,
    pub total_time: Option<Comparison<i32>>,
    pub preview_offset_from_start_ms: Option<Comparison<i32>>,
    pub num_timing_points: Option<Comparison<i32>>,
    pub beatmap_id: Option<Comparison<i32>>,
    pub beatmap_set_id: Option<Comparison<i32>>,
    pub thread_id: Option<Comparison<i32>>,
    pub standard_grade: Option<Comparison<u8>>,
    pub taiko_grade: Option<Comparison<u8>>,
    pub ctb_grade: Option<Comparison<u8>>,
    pub mania_grade: Option<Comparison<u8>>,
    pub local_offset: Option<Comparison<i16>>,
    pub stack_leniency: Option<Comparison<f32>>,
    pub gameplay_mode: Option<Comparison<GameplayMode>>,
    pub song_source: Option<Comparison<String>>,
    pub song_tags: Option<Comparison<String>>,
    pub online_offset: Option<Comparison<i16>>,
    pub font_used_for_song_title: Option<Comparison<String>>,
    pub unplayed: Option<Comparison<bool>>,
    pub last_played: Option<Comparison<SystemTime>>,
    pub is_osz2: Option<Comparison<bool>>,
    pub beatmap_folder_name: Option<Comparison<String>>,
    pub last_checked_against_repo: Option<Comparison<SystemTime>>,
    pub ignore_beatmap_sound: Option<Comparison<bool>>,
    pub ignore_beatmap_skin: Option<Comparison<bool>>,
    pub disable_storyboard: Option<Comparison<bool>>,
    pub disable_video: Option<Comparison<bool>>,
    pub visual_override: Option<Comparison<bool>>,
    pub offset_from_song_start_in_editor_ms: Option<Comparison<i32>>,
    pub mania_scroll_speed: Option<Comparison<u8>>
}

impl BeatmapQuery {
    pub fn from_arg(s: &str) -> IoResult<Self> {
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
        let matches = App::new("Beatmap query parser")
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
                .takes_value(false))
            .get_matches_from(args.into_iter());
        let (mut entry_size, mut artist_name, mut artist_name_unicode, mut song_title,
            mut song_title_unicode, mut creator_name, mut difficulty, mut audio_file_name,
            mut md5_beatmap_hash, mut dotosu_file_name, mut ranked_status, mut number_of_hitcircles,
            mut number_of_sliders, mut number_of_spinners, mut last_modification_time,
            mut approach_rate, mut circle_size, mut hp_drain, mut overall_difficulty,
            mut slider_velocity, mut num_mod_combo_star_ratings_standard,
            mut num_mod_combo_star_ratings_taiko, mut num_mod_combo_star_ratings_ctb,
            mut num_mod_combo_star_ratings_mania, mut drain_time, mut total_time,
            mut preview_offset_from_start_ms, mut num_timing_points, mut beatmap_id,
            mut beatmap_set_id, mut thread_id, mut standard_grade, mut taiko_grade, mut ctb_grade,
            mut mania_grade, mut local_offset, mut stack_leniency, mut gameplay_mode,
            mut song_source, mut song_tags, mut online_offset, mut font_used_for_song_title,
            mut unplayed, mut last_played, mut is_osz2, mut beatmap_folder_name,
            mut last_checked_against_repo, mut ignore_beatmap_sound, mut ignore_beatmap_skin,
            mut disable_storyboard, mut disable_video, mut visual_override,
            mut offset_from_song_start_in_editor_ms, mut mania_scroll_speed) = (None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None);
        get_matches_and_assign!(matches {
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
            "Song source", song_source => String;
            "Song tags", song_tags => String;
            "Online offset", online_offset => i16;
            "Font used for song title", font_used_for_song_title => String;
            "Unplayed", unplayed => bool;
            "Last played", last_played => SystemTime;
            "Is OSZ2", is_osz2 => bool;
            "Ignore beatmap sound", ignore_beatmap_sound => bool;
            "Ignore beatmap skin", ignore_beatmap_skin => bool;
            "Disable storyboard", disable_storyboard => bool;
            "Disable video", disable_video => bool;
            "Visual override", visual_override => bool;
            "Offset from song start in editor (ms)", offset_from_song_start_in_editor_ms => i32;
            "Mania scroll speed", mania_scroll_speed => u8;
        });
        Ok(BeatmapQuery {
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
    }
}