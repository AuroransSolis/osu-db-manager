use std::io::{Result as IoResult, Error as IoError, ErrorKind::InvalidInput};

use clap::{Arg, App, SubCommand, AppSettings, ArgGroup};
use chrono::naive::NaiveDate;

use crate::load_settings::{
    LoadSetting,
    SpecialArgType,
    parse_from_arg,
    parse_from_arg_special,
    query::QueryStruct
};
use crate::databases::osu::primitives::{RankedStatus, ByteSingle, GameplayMode};
use crate::masks::osu_mask::BeatmapMask;

pub struct BeatmapLoadSettings {
    pub entry_size: LoadSetting<Option<i32>>,
    pub artist_name: LoadSetting<Option<String>>,
    pub artist_name_unicode: LoadSetting<Option<String>>,
    pub song_title: LoadSetting<Option<String>>,
    pub song_title_unicode: LoadSetting<Option<String>>,
    pub creator_name: LoadSetting<Option<String>>,
    pub difficulty: LoadSetting<Option<String>>,
    pub audio_file_name: LoadSetting<Option<String>>,
    pub md5_beatmap_hash: LoadSetting<String>,
    pub dotosu_file_name: LoadSetting<Option<String>>,
    pub ranked_status: LoadSetting<RankedStatus>,
    pub number_of_hitcircles: LoadSetting<i16>,
    pub number_of_sliders: LoadSetting<i16>,
    pub number_of_spinners: LoadSetting<i16>,
    pub last_modification_time: LoadSetting<NaiveDate>,
    pub approach_rate: LoadSetting<ByteSingle>,
    pub circle_size: LoadSetting<ByteSingle>,
    pub hp_drain: LoadSetting<ByteSingle>,
    pub overall_difficulty: LoadSetting<ByteSingle>,
    pub slider_velocity: LoadSetting<f64>,
    pub num_mod_combo_star_ratings_standard: LoadSetting<i32>,
    pub mod_combo_star_ratings_standard: LoadSetting<()>,
    pub num_mod_combo_star_ratings_taiko: LoadSetting<i32>,
    pub mod_combo_star_ratings_taiko: LoadSetting<()>,
    pub num_mod_combo_star_ratings_ctb: LoadSetting<i32>,
    pub mod_combo_star_ratings_ctb: LoadSetting<()>,
    pub num_mod_combo_star_ratings_mania: LoadSetting<i32>,
    pub mod_combo_star_ratings_mania: LoadSetting<()>,
    pub drain_time: LoadSetting<i32>,
    pub total_time: LoadSetting<i32>,
    pub preview_offset_from_start_ms: LoadSetting<i32>,
    pub num_timing_points: LoadSetting<i32>,
    pub timing_points: LoadSetting<()>,
    pub beatmap_id: LoadSetting<i32>,
    pub beatmap_set_id: LoadSetting<i32>,
    pub thread_id: LoadSetting<i32>,
    pub standard_grade: LoadSetting<u8>,
    pub taiko_grade: LoadSetting<u8>,
    pub ctb_grade: LoadSetting<u8>,
    pub mania_grade: LoadSetting<u8>,
    pub local_offset: LoadSetting<i16>,
    pub stack_leniency: LoadSetting<f32>,
    pub gameplay_mode: LoadSetting<GameplayMode>,
    pub song_source: LoadSetting<Option<String>>,
    pub song_tags: LoadSetting<Option<String>>,
    pub online_offset: LoadSetting<i16>,
    pub font_used_for_song_title: LoadSetting<Option<String>>,
    pub unplayed: LoadSetting<bool>,
    pub last_played: LoadSetting<NaiveDate>,
    pub is_osz2: LoadSetting<bool>,
    pub beatmap_folder_name: LoadSetting<Option<String>>,
    pub last_checked_against_repo: LoadSetting<NaiveDate>,
    pub ignore_beatmap_sound: LoadSetting<bool>,
    pub ignore_beatmap_skin: LoadSetting<bool>,
    pub disable_storyboard: LoadSetting<bool>,
    pub disable_video: LoadSetting<bool>,
    pub visual_override: LoadSetting<bool>,
    pub unknown_short: LoadSetting<Option<i16>>,
    pub offset_from_song_start_in_editor_ms: LoadSetting<i32>,
    pub mania_scroll_speed: LoadSetting<u8>
}

impl Default for BeatmapLoadSettings {
    fn default() -> Self {
        BeatmapLoadSettings {
            entry_size: LoadSetting::Ignore,
            artist_name: LoadSetting::Ignore,
            artist_name_unicode: LoadSetting::Ignore,
            song_title: LoadSetting::Ignore,
            song_title_unicode: LoadSetting::Ignore,
            creator_name: LoadSetting::Ignore,
            difficulty: LoadSetting::Ignore,
            audio_file_name: LoadSetting::Ignore,
            md5_beatmap_hash: LoadSetting::Ignore,
            dotosu_file_name: LoadSetting::Ignore,
            ranked_status: LoadSetting::Ignore,
            number_of_hitcircles: LoadSetting::Ignore,
            number_of_sliders: LoadSetting::Ignore,
            number_of_spinners: LoadSetting::Ignore,
            last_modification_time: LoadSetting::Ignore,
            approach_rate: LoadSetting::Ignore,
            circle_size: LoadSetting::Ignore,
            hp_drain: LoadSetting::Ignore,
            overall_difficulty: LoadSetting::Ignore,
            slider_velocity: LoadSetting::Ignore,
            num_mod_combo_star_ratings_standard: LoadSetting::Ignore,
            mod_combo_star_ratings_standard: LoadSetting::Ignore,
            num_mod_combo_star_ratings_taiko: LoadSetting::Ignore,
            mod_combo_star_ratings_taiko: LoadSetting::Ignore,
            num_mod_combo_star_ratings_ctb: LoadSetting::Ignore,
            mod_combo_star_ratings_ctb: LoadSetting::Ignore,
            num_mod_combo_star_ratings_mania: LoadSetting::Ignore,
            mod_combo_star_ratings_mania: LoadSetting::Ignore,
            drain_time: LoadSetting::Ignore,
            total_time: LoadSetting::Ignore,
            preview_offset_from_start_ms: LoadSetting::Ignore,
            num_timing_points: LoadSetting::Ignore,
            timing_points: LoadSetting::Ignore,
            beatmap_id: LoadSetting::Ignore,
            beatmap_set_id: LoadSetting::Ignore,
            thread_id: LoadSetting::Ignore,
            standard_grade: LoadSetting::Ignore,
            taiko_grade: LoadSetting::Ignore,
            ctb_grade: LoadSetting::Ignore,
            mania_grade: LoadSetting::Ignore,
            local_offset: LoadSetting::Ignore,
            stack_leniency: LoadSetting::Ignore,
            gameplay_mode: LoadSetting::Ignore,
            song_source: LoadSetting::Ignore,
            song_tags: LoadSetting::Ignore,
            online_offset: LoadSetting::Ignore,
            font_used_for_song_title: LoadSetting::Ignore,
            unplayed: LoadSetting::Ignore,
            last_played: LoadSetting::Ignore,
            is_osz2: LoadSetting::Ignore,
            beatmap_folder_name: LoadSetting::Ignore,
            last_checked_against_repo: LoadSetting::Ignore,
            ignore_beatmap_sound: LoadSetting::Ignore,
            ignore_beatmap_skin: LoadSetting::Ignore,
            disable_storyboard: LoadSetting::Ignore,
            disable_video: LoadSetting::Ignore,
            visual_override: LoadSetting::Ignore,
            unknown_short: LoadSetting::Ignore,
            offset_from_song_start_in_editor_ms: LoadSetting::Ignore,
            mania_scroll_speed: LoadSetting::Ignore
        }
    }
}

impl BeatmapLoadSettings {
    pub fn load_all(&self) -> bool {
        !(self.entry_size.is_ignore() || self.artist_name.is_ignore()
            || self.artist_name_unicode.is_ignore() || self.song_title.is_ignore()
            || self.song_title_unicode.is_ignore() || self.creator_name.is_ignore()
            || self.difficulty.is_ignore() || self.audio_file_name.is_ignore()
            || self.md5_beatmap_hash.is_ignore() || self.dotosu_file_name.is_ignore()
            || self.ranked_status.is_ignore() || self.number_of_hitcircles.is_ignore()
            || self.number_of_sliders.is_ignore() || self.number_of_spinners.is_ignore()
            || self.last_modification_time.is_ignore() || self.approach_rate.is_ignore()
            || self.circle_size.is_ignore() || self.hp_drain.is_ignore()
            || self.overall_difficulty.is_ignore() || self.slider_velocity.is_ignore()
            || self.num_mod_combo_star_ratings_standard.is_ignore()
            || self.mod_combo_star_ratings_standard.is_ignore()
            || self.num_mod_combo_star_ratings_taiko.is_ignore()
            || self.mod_combo_star_ratings_taiko.is_ignore()
            || self.num_mod_combo_star_ratings_ctb.is_ignore()
            || self.mod_combo_star_ratings_ctb.is_ignore()
            || self.num_mod_combo_star_ratings_mania.is_ignore()
            || self.mod_combo_star_ratings_mania.is_ignore() || self.drain_time.is_ignore()
            || self.total_time.is_ignore() || self.preview_offset_from_start_ms.is_ignore()
            || self.num_timing_points.is_ignore() || self.timing_points.is_ignore()
            || self.beatmap_id.is_ignore() || self.beatmap_set_id.is_ignore()
            || self.thread_id.is_ignore() || self.standard_grade.is_ignore()
            || self.taiko_grade.is_ignore() || self.ctb_grade.is_ignore()
            || self.mania_grade.is_ignore() || self.local_offset.is_ignore()
            || self.stack_leniency.is_ignore() || self.gameplay_mode.is_ignore()
            || self.song_source.is_ignore() || self.song_tags.is_ignore()
            || self.online_offset.is_ignore() || self.font_used_for_song_title.is_ignore()
            || self.unplayed.is_ignore() || self.last_played.is_ignore() || self.is_osz2.is_ignore()
            || self.beatmap_folder_name.is_ignore() || self.last_checked_against_repo.is_ignore()
            || self.ignore_beatmap_sound.is_ignore() || self.ignore_beatmap_skin.is_ignore()
            || self.disable_storyboard.is_ignore() || self.disable_video.is_ignore()
            || self.visual_override.is_ignore()
            || self.offset_from_song_start_in_editor_ms.is_ignore()
            || self.mania_scroll_speed.is_ignore())
    }

    pub fn set_from_query(&mut self, query_args: Vec<&str>) -> IoResult<()> {
        if query_args.len() == 0 {
            return Ok(());
        }
        let matches = App::new("osu!.db query parser")
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
            .get_matches_from(query_args.into_iter());
        self.entry_size = parse_from_arg::<i32>(&matches, "Entry size")?;
        self.artist_name = parse_from_arg_special::<Option<String>>(&matches, "Artist name",
            SpecialArgType::OptionString)?;
        self.artist_name_unicode = parse_from_arg_special::<Option<String>>(&matches,
            "Artist name unicode", SpecialArgType::OptionString)?;
        self.song_title = parse_from_arg_special::<Option<String>>(&matches, "Song title",
            SpecialArgType::OptionString)?;
        self.song_title_unicode = parse_from_arg_special::<Option<String>>(&matches,
            "Song title unicode", SpecialArgType::OptionString)?;
        self.creator_name = parse_from_arg_special::<Option<String>>(&matches, "Creator name",
            SpecialArgType::OptionString)?;
        self.difficulty = parse_from_arg_special::<Option<String>>(&matches, "Difficulty",
            SpecialArgType::OptionString)?;
        self.audio_file_name = parse_from_arg_special::<Option<String>>(&matches, "Audio file name",
            SpecialArgType::OptionString)?;
        self.md5_beatmap_hash = parse_from_arg_special::<String>(&matches, "MD5 beatmap hash",
            SpecialArgType::String)?;
        self.dotosu_file_name = parse_from_arg_special::<Option<String>>(&matches, ".osu file name",
            SpecialArgType::OptionString)?;
        self.ranked_status = parse_from_arg::<RankedStatus>(&matches, "Ranked status")?;
        self.number_of_hitcircles = parse_from_arg::<i16>(&matches, "Number of hitcircles")?;
        self.number_of_sliders = parse_from_arg::<i16>(&matches, "Number of sliders")?;
        self.number_of_spinners = parse_from_arg::<i16>(&matches, "Number of spinners")?;
        self.last_modification_time = parse_from_arg_special::<NaiveDate>(&matches,
            "Last modification time", SpecialArgType::NaiveDate)?;
        self.approach_rate = parse_from_arg::<ByteSingle>(&matches,"Approach rate")?;
        self.circle_size = parse_from_arg::<ByteSingle>(&matches,"Circle size")?;
        self.hp_drain = parse_from_arg::<ByteSingle>(&matches, "HP drain")?;
        self.overall_difficulty = parse_from_arg::<ByteSingle>(&matches,"Overall difficulty")?;
        self.slider_velocity = parse_from_arg::<f64>(&matches,"Slider velocity")?;
        self.num_mod_combo_star_ratings_standard = parse_from_arg::<i32>(&matches,
            "Number of precalculated mod combo star ratings (standard)")?;
        self.num_mod_combo_star_ratings_taiko = parse_from_arg::<i32>(&matches,
            "Number of precalculated mod combo star ratings (taiko)")?;
        self.num_mod_combo_star_ratings_ctb = parse_from_arg::<i32>(&matches,
            "Number of precalculated mod combo star ratings (CTB)")?;
        self.num_mod_combo_star_ratings_mania = parse_from_arg::<i32>(&matches,
            "Number of precalculated mod combo star ratings (mania)")?;
        self.drain_time = parse_from_arg::<i32>(&matches, "Drain time")?;
        self.total_time = parse_from_arg::<i32>(&matches, "Total time")?;
        self.preview_offset_from_start_ms = parse_from_arg::<i32>(&matches,
            "Preview offset from start (ms)")?;
        self.num_timing_points = parse_from_arg::<i32>(&matches,
            "Number of timing points")?;
        self.beatmap_id = parse_from_arg::<i32>(&matches, "Beatmap ID")?;
        self.beatmap_set_id = parse_from_arg::<i32>(&matches, "Beatmap set ID")?;
        self.thread_id = parse_from_arg::<i32>(&matches, "Thread ID")?;
        self.standard_grade = parse_from_arg::<u8>(&matches, "Standard grade")?;
        self.taiko_grade = parse_from_arg::<u8>(&matches, "Taiko grade")?;
        self.ctb_grade = parse_from_arg::<u8>(&matches, "CTB grade")?;
        self.mania_grade = parse_from_arg::<u8>(&matches, "Mania grade")?;
        self.local_offset = parse_from_arg::<i16>(&matches, "Local offset")?;
        self.stack_leniency = parse_from_arg::<f32>(&matches, "Stack leniency")?;
        self.gameplay_mode = parse_from_arg::<GameplayMode>(&matches, "Gameplay mode")?;
        self.song_source = parse_from_arg_special::<Option<String>>(&matches, "Song source",
            SpecialArgType::OptionString)?;
        self.song_tags = parse_from_arg_special::<Option<String>>(&matches, "Song tags",
            SpecialArgType::OptionString)?;
        self.online_offset = parse_from_arg::<i16>(&matches, "Online offset")?;
        self.font_used_for_song_title = parse_from_arg_special::<Option<String>>(&matches,
            "Font used for song title", SpecialArgType::OptionString)?;
        self.unplayed = parse_from_arg_special::<bool>(&matches, "Unplayed", SpecialArgType::bool)?;
        self.last_played = parse_from_arg_special::<NaiveDate>(&matches, "Last played",
            SpecialArgType::NaiveDate)?;
        self.is_osz2 = parse_from_arg_special::<bool>(&matches, "Is OSZ2", SpecialArgType::bool)?;
        self.beatmap_folder_name = parse_from_arg_special::<Option<String>>(&matches,
            "Beatmap folder name", SpecialArgType::OptionString)?;
        self.last_checked_against_repo = parse_from_arg_special::<NaiveDate>(&matches,
            "Last checked against repo", SpecialArgType::NaiveDate)?;
        self.ignore_beatmap_sound = parse_from_arg_special::<bool>(&matches, "Ignore beatmap sound",
            SpecialArgType::bool)?;
        self.ignore_beatmap_skin = parse_from_arg_special::<bool>(&matches, "Ignore beatmap skin",
            SpecialArgType::bool)?;
        self.disable_storyboard = parse_from_arg_special::<bool>(&matches, "Disable storyboard",
            SpecialArgType::bool)?;
        self.disable_video = parse_from_arg_special::<bool>(&matches, "Disable video",
            SpecialArgType::bool)?;
        self.visual_override = parse_from_arg_special::<bool>(&matches, "Visual override",
            SpecialArgType::bool)?;
        self.offset_from_song_start_in_editor_ms = parse_from_arg::<i32>(&matches,
            "Offset from song start in editor (ms)")?;
        self.mania_scroll_speed = parse_from_arg::<u8>(&matches, "Mania scroll speed")?;
        Ok(())
    }

    pub fn set_from_mask(&mut self, mask: &BeatmapMask) {
        if self.entry_size.is_ignore() && mask.entry_size {
            self.entry_size = LoadSetting::Load;
        }
        if self.artist_name.is_ignore() && mask.artist_name {
            self.artist_name = LoadSetting::Load;
        }
        if self.artist_name_unicode.is_ignore() && mask.artist_name_unicode {
            self.artist_name_unicode = LoadSetting::Load;
        }
        if self.song_title.is_ignore() && mask.song_title {
            self.song_title = LoadSetting::Load;
        }
        if self.song_title_unicode.is_ignore() && mask.song_title_unicode {
            self.song_title_unicode = LoadSetting::Load;
        }
        if self.creator_name.is_ignore() && mask.creator_name {
            self.creator_name = LoadSetting::Load;
        }
        if self.difficulty.is_ignore() && mask.difficulty {
            self.difficulty = LoadSetting::Load;
        }
        if self.audio_file_name.is_ignore() && mask.audio_file_name {
            self.audio_file_name = LoadSetting::Load;
        }
        if self.md5_beatmap_hash.is_ignore() && mask.md5_beatmap_hash {
            self.md5_beatmap_hash = LoadSetting::Load;
        }
        if self.dotosu_file_name.is_ignore() && mask.dotosu_file_name {
            self.dotosu_file_name = LoadSetting::Load;
        }
        if self.ranked_status.is_ignore() && mask.ranked_status {
            self.ranked_status = LoadSetting::Load;
        }
        if self.number_of_hitcircles.is_ignore() && mask.number_of_hitcircles {
            self.number_of_hitcircles = LoadSetting::Load;
        }
        if self.number_of_sliders.is_ignore() && mask.number_of_sliders {
            self.number_of_sliders = LoadSetting::Load;
        }
        if self.number_of_spinners.is_ignore() && mask.number_of_spinners {
            self.number_of_spinners = LoadSetting::Load;
        }
        if self.last_modification_time.is_ignore() && mask.last_modification_time {
            self.last_modification_time = LoadSetting::Load;
        }
        if self.approach_rate.is_ignore() && mask.approach_rate {
            self.approach_rate = LoadSetting::Load;
        }
        if self.circle_size.is_ignore() && mask.circle_size {
            self.circle_size = LoadSetting::Load;
        }
        if self.hp_drain.is_ignore() && mask.hp_drain {
            self.hp_drain = LoadSetting::Load;
        }
        if self.overall_difficulty.is_ignore() && mask.overall_difficulty {
            self.overall_difficulty = LoadSetting::Load;
        }
        if self.slider_velocity.is_ignore() && mask.slider_velocity {
            self.slider_velocity = LoadSetting::Load;
        }
        if self.num_mod_combo_star_ratings_standard.is_ignore()
            && mask.num_mod_combo_star_ratings_standard {
            self.num_mod_combo_star_ratings_standard = LoadSetting::Load;
        }
        if self.mod_combo_star_ratings_standard.is_ignore()
            && mask.mod_combo_star_ratings_standard {
            self.mod_combo_star_ratings_standard = LoadSetting::Load;
        }
        if self.num_mod_combo_star_ratings_taiko.is_ignore()
            && mask.num_mod_combo_star_ratings_taiko {
            self.num_mod_combo_star_ratings_taiko = LoadSetting::Load;
        }
        if self.mod_combo_star_ratings_taiko.is_ignore() && mask.mod_combo_star_ratings_taiko {
            self.mod_combo_star_ratings_taiko = LoadSetting::Load;
        }
        if self.num_mod_combo_star_ratings_ctb.is_ignore() && mask.num_mod_combo_star_ratings_ctb {
            self.num_mod_combo_star_ratings_ctb = LoadSetting::Load;
        }
        if self.mod_combo_star_ratings_ctb.is_ignore() && mask.mod_combo_star_ratings_ctb {
            self.mod_combo_star_ratings_ctb = LoadSetting::Load;
        }
        if self.num_mod_combo_star_ratings_mania.is_ignore()
            && mask.num_mod_combo_star_ratings_mania {
            self.num_mod_combo_star_ratings_mania = LoadSetting::Load;
        }
        if self.mod_combo_star_ratings_mania.is_ignore() && mask.mod_combo_star_ratings_mania {
            self.mod_combo_star_ratings_mania = LoadSetting::Load;
        }
        if self.drain_time.is_ignore() && mask.drain_time {
            self.drain_time = LoadSetting::Load;
        }
        if self.total_time.is_ignore() && mask.total_time {
            self.total_time = LoadSetting::Load;
        }
        if self.preview_offset_from_start_ms.is_ignore() && mask.preview_offset_from_start_ms {
            self.preview_offset_from_start_ms = LoadSetting::Load;
        }
        if self.num_timing_points.is_ignore() && mask.num_timing_points {
            self.num_timing_points = LoadSetting::Load;
        }
        if self.timing_points.is_ignore() && mask.timing_points {
            self.timing_points = LoadSetting::Load;
        }
        if self.beatmap_id.is_ignore() && mask.beatmap_id {
            self.beatmap_id = LoadSetting::Load;
        }
        if self.beatmap_set_id.is_ignore() && mask.beatmap_set_id {
            self.beatmap_set_id = LoadSetting::Load;
        }
        if self.thread_id.is_ignore() && mask.thread_id {
            self.thread_id = LoadSetting::Load;
        }
        if self.standard_grade.is_ignore() && mask.standard_grade {
            self.standard_grade = LoadSetting::Load;
        }
        if self.taiko_grade.is_ignore() && mask.taiko_grade {
            self.taiko_grade = LoadSetting::Load;
        }
        if self.ctb_grade.is_ignore() && mask.ctb_grade {
            self.ctb_grade = LoadSetting::Load;
        }
        if self.mania_grade.is_ignore() && mask.mania_grade {
            self.mania_grade = LoadSetting::Load;
        }
        if self.local_offset.is_ignore() && mask.local_offset {
            self.local_offset = LoadSetting::Load;
        }
        if self.stack_leniency.is_ignore() && mask.stack_leniency {
            self.stack_leniency = LoadSetting::Load;
        }
        if self.gameplay_mode.is_ignore() && mask.gameplay_mode {
            self.gameplay_mode = LoadSetting::Load;
        }
        if self.song_source.is_ignore() && mask.song_source {
            self.song_source = LoadSetting::Load;
        }
        if self.song_tags.is_ignore() && mask.song_tags {
            self.song_tags = LoadSetting::Load;
        }
        if self.online_offset.is_ignore() && mask.online_offset {
            self.online_offset = LoadSetting::Load;
        }
        if self.font_used_for_song_title.is_ignore() && mask.font_used_for_song_title {
            self.font_used_for_song_title = LoadSetting::Load;
        }
        if self.unplayed.is_ignore() && mask.unplayed {
            self.unplayed = LoadSetting::Load;
        }
        if self.last_played.is_ignore() && mask.last_played {
            self.last_played = LoadSetting::Load;
        }
        if self.is_osz2.is_ignore() && mask.is_osz2 {
            self.is_osz2 = LoadSetting::Load;
        }
        if self.beatmap_folder_name.is_ignore() && mask.beatmap_folder_name {
            self.beatmap_folder_name = LoadSetting::Load;
        }
        if self.last_checked_against_repo.is_ignore() && mask.last_checked_against_repo {
            self.last_checked_against_repo = LoadSetting::Load;
        }
        if self.ignore_beatmap_sound.is_ignore() && mask.ignore_beatmap_sound {
            self.ignore_beatmap_sound = LoadSetting::Load;
        }
        if self.ignore_beatmap_skin.is_ignore() && mask.ignore_beatmap_skin {
            self.ignore_beatmap_skin = LoadSetting::Load;
        }
        if self.disable_storyboard.is_ignore() && mask.disable_storyboard {
            self.disable_storyboard = LoadSetting::Load;
        }
        if self.disable_video.is_ignore() && mask.disable_video {
            self.disable_video = LoadSetting::Load;
        }
        if self.visual_override.is_ignore() && mask.visual_override {
            self.visual_override = LoadSetting::Load;
        }
        if self.unknown_short.is_ignore() && mask.unknown_short {
            self.unknown_short = LoadSetting::Load;
        }
        if self.offset_from_song_start_in_editor_ms.is_ignore()
            && mask.offset_from_song_start_in_editor_ms {
            self.offset_from_song_start_in_editor_ms = LoadSetting::Load;
        }
        if self.mania_scroll_speed.is_ignore() && mask.mania_scroll_speed {
            self.mania_scroll_speed = LoadSetting::Load;
        }
    }
}