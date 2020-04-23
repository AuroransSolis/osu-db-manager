use std::io::{Error as IoError, ErrorKind::InvalidInput, Result as IoResult};

use chrono::naive::NaiveDate;
use clap::{App, AppSettings, Arg, ArgGroup, SubCommand};

use crate::databases::osu::primitives::{ByteSingle, GameplayMode, RankedStatus};
use crate::load_settings::{query::QueryStruct, EqualClone, EqualCopy, LoadSetting, Relational};
use crate::masks::osu_mask::BeatmapMask;

#[derive(Clone)]
pub struct BeatmapLoadSettings {
    pub entry_size: Relational<i32>,
    pub artist_name: EqualClone<String>,
    pub artist_name_unicode: EqualClone<String>,
    pub song_title: EqualClone<String>,
    pub song_title_unicode: EqualClone<String>,
    pub creator_name: EqualClone<String>,
    pub difficulty: EqualClone<String>,
    pub audio_file_name: EqualClone<String>,
    pub md5_beatmap_hash: EqualClone<String>,
    pub dotosu_file_name: EqualClone<String>,
    pub ranked_status: EqualCopy<RankedStatus>,
    pub number_of_hitcircles: Relational<i16>,
    pub number_of_sliders: Relational<i16>,
    pub number_of_spinners: Relational<i16>,
    pub last_modification_time: Relational<NaiveDate>,
    pub approach_rate: Relational<ByteSingle>,
    pub circle_size: Relational<ByteSingle>,
    pub hp_drain: Relational<ByteSingle>,
    pub overall_difficulty: Relational<ByteSingle>,
    pub slider_velocity: Relational<f64>,
    pub num_mod_combo_star_ratings_standard: Relational<i32>,
    pub mod_combo_star_ratings_standard: bool,
    pub num_mod_combo_star_ratings_taiko: Relational<i32>,
    pub mod_combo_star_ratings_taiko: bool,
    pub num_mod_combo_star_ratings_ctb: Relational<i32>,
    pub mod_combo_star_ratings_ctb: bool,
    pub num_mod_combo_star_ratings_mania: Relational<i32>,
    pub mod_combo_star_ratings_mania: bool,
    pub drain_time: Relational<i32>,
    pub total_time: Relational<i32>,
    pub preview_offset_from_start_ms: Relational<i32>,
    pub num_timing_points: Relational<i32>,
    pub timing_points: bool,
    pub beatmap_id: Relational<i32>,
    pub beatmap_set_id: Relational<i32>,
    pub thread_id: Relational<i32>,
    pub standard_grade: Relational<u8>,
    pub taiko_grade: Relational<u8>,
    pub ctb_grade: Relational<u8>,
    pub mania_grade: Relational<u8>,
    pub local_offset: Relational<i16>,
    pub stack_leniency: Relational<f32>,
    pub gameplay_mode: EqualCopy<GameplayMode>,
    pub song_source: EqualClone<String>,
    pub song_tags: EqualClone<String>,
    pub online_offset: Relational<i16>,
    pub font_used_for_song_title: EqualClone<String>,
    pub unplayed: EqualCopy<bool>,
    pub last_played: Relational<NaiveDate>,
    pub is_osz2: EqualCopy<bool>,
    pub beatmap_folder_name: EqualClone<String>,
    pub last_checked_against_repo: Relational<NaiveDate>,
    pub ignore_beatmap_sound: EqualCopy<bool>,
    pub ignore_beatmap_skin: EqualCopy<bool>,
    pub disable_storyboard: EqualCopy<bool>,
    pub disable_video: EqualCopy<bool>,
    pub visual_override: EqualCopy<bool>,
    pub unknown_short: bool,
    pub offset_from_song_start_in_editor_ms: Relational<i32>,
    pub mania_scroll_speed: Relational<u8>,
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
            mania_scroll_speed: LoadSetting::Ignore,
        }
    }
}

impl BeatmapLoadSettings {
    pub fn load_all(&self) -> bool {
        self.entry_size.is_load()
            && self.artist_name.is_load()
            && self.artist_name_unicode.is_load()
            && self.song_title.is_load()
            && self.song_title_unicode.is_load()
            && self.creator_name.is_load()
            && self.difficulty.is_load()
            && self.audio_file_name.is_load()
            && self.md5_beatmap_hash.is_load()
            && self.dotosu_file_name.is_load()
            && self.ranked_status.is_load()
            && self.number_of_hitcircles.is_load()
            && self.number_of_sliders.is_load()
            && self.number_of_spinners.is_load()
            && self.last_modification_time.is_load()
            && self.approach_rate.is_load()
            && self.circle_size.is_load()
            && self.hp_drain.is_load()
            && self.overall_difficulty.is_load()
            && self.slider_velocity.is_load()
            && self.num_mod_combo_star_ratings_standard.is_load()
            && self.mod_combo_star_ratings_standard
            && self.num_mod_combo_star_ratings_taiko.is_load()
            && self.mod_combo_star_ratings_taiko
            && self.num_mod_combo_star_ratings_ctb.is_load()
            && self.mod_combo_star_ratings_ctb
            && self.num_mod_combo_star_ratings_mania.is_load()
            && self.mod_combo_star_ratings_mania
            && self.drain_time.is_load()
            && self.total_time.is_load()
            && self.preview_offset_from_start_ms.is_load()
            && self.num_timing_points.is_load()
            && self.timing_points
            && self.beatmap_id.is_load()
            && self.beatmap_set_id.is_load()
            && self.thread_id.is_load()
            && self.standard_grade.is_load()
            && self.taiko_grade.is_load()
            && self.ctb_grade.is_load()
            && self.mania_grade.is_load()
            && self.local_offset.is_load()
            && self.stack_leniency.is_load()
            && self.gameplay_mode.is_load()
            && self.song_source.is_load()
            && self.song_tags.is_load()
            && self.online_offset.is_load()
            && self.font_used_for_song_title.is_load()
            && self.unplayed.is_load()
            && self.last_played.is_load()
            && self.is_osz2.is_load()
            && self.beatmap_folder_name.is_load()
            && self.last_checked_against_repo.is_load()
            && self.ignore_beatmap_sound.is_load()
            && self.ignore_beatmap_skin.is_load()
            && self.disable_storyboard.is_load()
            && self.disable_video.is_load()
            && self.visual_override.is_load()
            && self.unknown_short
            && self.offset_from_song_start_in_editor_ms.is_load()
            && self.mania_scroll_speed.is_load()
    }

    pub fn ignore_all(&self) -> bool {
        self.entry_size.is_ignore()
            && self.artist_name.is_ignore()
            && self.artist_name_unicode.is_ignore()
            && self.song_title.is_ignore()
            && self.song_title_unicode.is_ignore()
            && self.creator_name.is_ignore()
            && self.difficulty.is_ignore()
            && self.audio_file_name.is_ignore()
            && self.md5_beatmap_hash.is_ignore()
            && self.dotosu_file_name.is_ignore()
            && self.ranked_status.is_ignore()
            && self.number_of_hitcircles.is_ignore()
            && self.number_of_sliders.is_ignore()
            && self.number_of_spinners.is_ignore()
            && self.last_modification_time.is_ignore()
            && self.approach_rate.is_ignore()
            && self.circle_size.is_ignore()
            && self.hp_drain.is_ignore()
            && self.overall_difficulty.is_ignore()
            && self.slider_velocity.is_ignore()
            && self.num_mod_combo_star_ratings_standard.is_ignore()
            && !self.mod_combo_star_ratings_standard
            && self.num_mod_combo_star_ratings_taiko.is_ignore()
            && !self.mod_combo_star_ratings_taiko
            && self.num_mod_combo_star_ratings_ctb.is_ignore()
            && !self.mod_combo_star_ratings_ctb
            && self.num_mod_combo_star_ratings_mania.is_ignore()
            && !self.mod_combo_star_ratings_mania
            && self.drain_time.is_ignore()
            && self.total_time.is_ignore()
            && self.preview_offset_from_start_ms.is_ignore()
            && self.num_timing_points.is_ignore()
            && !self.timing_points
            && self.beatmap_id.is_ignore()
            && self.beatmap_set_id.is_ignore()
            && self.thread_id.is_ignore()
            && self.standard_grade.is_ignore()
            && self.taiko_grade.is_ignore()
            && self.ctb_grade.is_ignore()
            && self.mania_grade.is_ignore()
            && self.local_offset.is_ignore()
            && self.stack_leniency.is_ignore()
            && self.gameplay_mode.is_ignore()
            && self.song_source.is_ignore()
            && self.song_tags.is_ignore()
            && self.online_offset.is_ignore()
            && self.font_used_for_song_title.is_ignore()
            && self.unplayed.is_ignore()
            && self.last_played.is_ignore()
            && self.is_osz2.is_ignore()
            && self.beatmap_folder_name.is_ignore()
            && self.last_checked_against_repo.is_ignore()
            && self.ignore_beatmap_sound.is_ignore()
            && self.ignore_beatmap_skin.is_ignore()
            && self.disable_storyboard.is_ignore()
            && self.disable_video.is_ignore()
            && self.visual_override.is_ignore()
            && !self.unknown_short
            && self.offset_from_song_start_in_editor_ms.is_ignore()
            && self.mania_scroll_speed.is_ignore()
    }

    pub fn is_partial(&self) -> bool {
        self.entry_size.is_ignore()
            || self.artist_name.is_ignore()
            || self.artist_name_unicode.is_ignore()
            || self.song_title.is_ignore()
            || self.song_title_unicode.is_ignore()
            || self.creator_name.is_ignore()
            || self.difficulty.is_ignore()
            || self.audio_file_name.is_ignore()
            || self.md5_beatmap_hash.is_ignore()
            || self.dotosu_file_name.is_ignore()
            || self.ranked_status.is_ignore()
            || self.number_of_hitcircles.is_ignore()
            || self.number_of_sliders.is_ignore()
            || self.number_of_spinners.is_ignore()
            || self.last_modification_time.is_ignore()
            || self.approach_rate.is_ignore()
            || self.circle_size.is_ignore()
            || self.hp_drain.is_ignore()
            || self.overall_difficulty.is_ignore()
            || self.slider_velocity.is_ignore()
            || self.num_mod_combo_star_ratings_standard.is_ignore()
            || !self.mod_combo_star_ratings_standard
            || self.num_mod_combo_star_ratings_taiko.is_ignore()
            || !self.mod_combo_star_ratings_taiko
            || self.num_mod_combo_star_ratings_ctb.is_ignore()
            || !self.mod_combo_star_ratings_ctb
            || self.num_mod_combo_star_ratings_mania.is_ignore()
            || !self.mod_combo_star_ratings_mania
            || self.drain_time.is_ignore()
            || self.total_time.is_ignore()
            || self.preview_offset_from_start_ms.is_ignore()
            || self.num_timing_points.is_ignore()
            || !self.timing_points
            || self.beatmap_id.is_ignore()
            || self.beatmap_set_id.is_ignore()
            || self.thread_id.is_ignore()
            || self.standard_grade.is_ignore()
            || self.taiko_grade.is_ignore()
            || self.ctb_grade.is_ignore()
            || self.mania_grade.is_ignore()
            || self.local_offset.is_ignore()
            || self.stack_leniency.is_ignore()
            || self.gameplay_mode.is_ignore()
            || self.song_source.is_ignore()
            || self.song_tags.is_ignore()
            || self.online_offset.is_ignore()
            || self.font_used_for_song_title.is_ignore()
            || self.unplayed.is_ignore()
            || self.last_played.is_ignore()
            || self.is_osz2.is_ignore()
            || self.beatmap_folder_name.is_ignore()
            || self.last_checked_against_repo.is_ignore()
            || self.ignore_beatmap_sound.is_ignore()
            || self.ignore_beatmap_skin.is_ignore()
            || self.disable_storyboard.is_ignore()
            || self.disable_video.is_ignore()
            || self.visual_override.is_ignore()
            || !self.unknown_short
            || self.offset_from_song_start_in_editor_ms.is_ignore()
            || self.mania_scroll_speed.is_ignore()
    }

    pub fn set_from_query(&mut self, args: Vec<&str>) -> IoResult<()> {
        if args.len() == 0 {
            return Ok(());
        }
        let matches = App::new("osu!.db query parser")
            .arg(
                Arg::with_name("Entry size")
                    .long("ENTRY-SIZE")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("SIZE"),
            )
            .arg(
                Arg::with_name("Artist name")
                    .long("ARTIST-NAME")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NAME"),
            )
            .arg(
                Arg::with_name("Artist name unicode")
                    .long("ARTIST-NAME-UNICODE")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NAME"),
            )
            .arg(
                Arg::with_name("Song title")
                    .long("SONG-TITLE")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("TITLE"),
            )
            .arg(
                Arg::with_name("Song title unicode")
                    .long("SONG-TITLE-UNICODE")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("TITLE"),
            )
            .arg(
                Arg::with_name("Creator name")
                    .long("CREATOR-NAME")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NAME"),
            )
            .arg(
                Arg::with_name("Difficulty")
                    .long("DIFFICULTY")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NAME")
                    .takes_value(false),
            )
            .arg(
                Arg::with_name("Audio file name")
                    .long("AUDIO-FILE-NAME")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("FILENAME")
                    .takes_value(false),
            )
            .arg(
                Arg::with_name("MD5 beatmap hash")
                    .long("MD5-BEATMAP-HASH")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("HASH")
                    .takes_value(false),
            )
            .arg(
                Arg::with_name(".osu file name")
                    .long("DOTOSU-FILE-NAME")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("FILENAME")
                    .takes_value(false),
            )
            .arg(
                Arg::with_name("Ranked status")
                    .long("RANKED-STATUS")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("STATUS")
                    .takes_value(false),
            )
            .arg(
                Arg::with_name("Number of hitcircles")
                    .long("NUMBER-OF-HITCIRCLES")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NUMBER/RANGE")
                    .takes_value(false),
            )
            .arg(
                Arg::with_name("Number of sliders")
                    .long("NUMBER-OF-SLIDERS")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NUMBER/RANGE")
                    .takes_value(false),
            )
            .arg(
                Arg::with_name("Number of spinners")
                    .long("NUMBER-OF-SPINNERS")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NUMBER/RANGE")
                    .takes_value(false),
            )
            .arg(
                Arg::with_name("Last modification time")
                    .long("LAST-MODIFICATION-TIME")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("DATE")
                    .takes_value(false),
            )
            .arg(
                Arg::with_name("Approach rate")
                    .long("APPROACH-RATE")
                    .multiple(false)
                    .short("AR")
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NUMBER/RANGE")
                    .takes_value(false),
            )
            .arg(
                Arg::with_name("Circle size")
                    .long("CIRCLE-SIZE")
                    .multiple(false)
                    .short("CS")
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NUMBER/RANGE")
                    .takes_value(false),
            )
            .arg(
                Arg::with_name("HP drain")
                    .long("HP-DRAIN")
                    .multiple(false)
                    .short("HP")
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NUMBER/RANGE")
                    .takes_value(false),
            )
            .arg(
                Arg::with_name("Overall difficulty")
                    .long("OVERALL-DIFFICULTY")
                    .multiple(false)
                    .short("OD")
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NUMBER/RANGE")
                    .takes_value(false),
            )
            .arg(
                Arg::with_name("Slider velocity")
                    .long("SLIDER-VELOCITY")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NUMBER/RANGE")
                    .takes_value(false),
            )
            .arg(
                Arg::with_name("Number of precalculated mod combo star ratings (standard)")
                    .long("NUM-MOD-COMBO-STAR-RATINGS-STANDARD")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NUMBER/RANGE")
                    .takes_value(false),
            )
            .arg(
                Arg::with_name("Number of precalculated mod combo star ratings (taiko)")
                    .long("NUM-MOD-COMBO-STAR-RATINGS-TAIKO")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NUMBER/RANGE")
                    .takes_value(false),
            )
            .arg(
                Arg::with_name("Number of precalculated mod combo star ratings (CTB)")
                    .long("NUM-MOD-COMBO-STAR-RATINGS-CTB")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("")
                    .takes_value(false),
            )
            .arg(
                Arg::with_name("Number of precalculated mod combo star ratings (mania)")
                    .long("NUM-MOD-COMBO-STAR-RATINGS-MANIA")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NUMBER/RANGE")
                    .takes_value(false),
            )
            .arg(
                Arg::with_name("Drain time")
                    .long("DRAIN-TIME")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NUMBER/RANGE")
                    .takes_value(false),
            )
            .arg(
                Arg::with_name("Total time")
                    .long("TOTAL-TIME")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NUMBER/RANGE")
                    .takes_value(false),
            )
            .arg(
                Arg::with_name("Preview offset from start (ms)")
                    .long("PREVIEW-OFFSET-FROM-START-MS")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NUMBER/RANGE")
                    .takes_value(false),
            )
            .arg(
                Arg::with_name("Number of timing points")
                    .long("NUM-TIMING-POINTS")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NUMBER/RANGE")
                    .takes_value(false),
            )
            .arg(
                Arg::with_name("Beatmap ID")
                    .long("BEATMAP-ID")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NUMBER/RANGE")
                    .takes_value(false),
            )
            .arg(
                Arg::with_name("Beatmap set ID")
                    .long("BEATMAP-SET-ID")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NUMBER/RANGE")
                    .takes_value(false),
            )
            .arg(
                Arg::with_name("Thread ID")
                    .long("THREAD-ID")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NUMBER/RANGE")
                    .takes_value(false),
            )
            .arg(
                Arg::with_name("Standard grade")
                    .long("STANDARD-GRADE")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NUMBER/RANGE")
                    .takes_value(false),
            )
            .arg(
                Arg::with_name("Taiko grade")
                    .long("TAIKO-GRADE")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NUMBER/RANGE")
                    .takes_value(false),
            )
            .arg(
                Arg::with_name("CTB grade")
                    .long("CTB-GRADE")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NUMBER/RANGE")
                    .takes_value(false),
            )
            .arg(
                Arg::with_name("Mania grade")
                    .long("MANIA-GRADE")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NUMBER/RANGE")
                    .takes_value(false),
            )
            .arg(
                Arg::with_name("Local offset")
                    .long("LOCAL-OFFSET")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NUMBER/RANGE")
                    .takes_value(false),
            )
            .arg(
                Arg::with_name("Stack leniency")
                    .long("STACK-LENIENCY")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NUMBER/RANGE")
                    .takes_value(false),
            )
            .arg(
                Arg::with_name("Gameplay mode")
                    .long("GAMEPLAY-MODE")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("MODE")
                    .takes_value(false),
            )
            .arg(
                Arg::with_name("Song source")
                    .long("SONG-SOURCE")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("SOURCE")
                    .takes_value(false),
            )
            .arg(
                Arg::with_name("Song tags")
                    .long("SONG-TAGS")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("TAGS")
                    .takes_value(false),
            )
            .arg(
                Arg::with_name("Online offset")
                    .long("ONLINE-OFFSET")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NUMBER/RANGE")
                    .takes_value(false),
            )
            .arg(
                Arg::with_name("Font used for song title")
                    .long("FONT-USED-FOR-SONG-TITLE")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("FONT")
                    .takes_value(false),
            )
            .arg(
                Arg::with_name("Unplayed")
                    .long("UNPLAYED")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("")
                    .takes_value(false),
            )
            .arg(
                Arg::with_name("Last played")
                    .long("LAST-PLAYED")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("DATE")
                    .takes_value(false),
            )
            .arg(
                Arg::with_name("Is OSZ2")
                    .long("IS-OSZ2")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("T/F")
                    .takes_value(false),
            )
            .arg(
                Arg::with_name("Beatmap folder name")
                    .long("BEATMAP-FOLDER-NAME")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NAME")
                    .takes_value(false),
            )
            .arg(
                Arg::with_name("Last checked against repo")
                    .long("LAST-CHECKED-AGAINST-REPO")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("DATE")
                    .takes_value(false),
            )
            .arg(
                Arg::with_name("Ignore beatmap sound")
                    .long("IGNORE-BEATMAP-SOUND")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("T/F")
                    .takes_value(false),
            )
            .arg(
                Arg::with_name("Ignore beatmap skin")
                    .long("IGNORE-BEATMAP-SKIN")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("T/F")
                    .takes_value(false),
            )
            .arg(
                Arg::with_name("Disable storyboard")
                    .long("DISABLE-STORYBOARD")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("T/F")
                    .takes_value(false),
            )
            .arg(
                Arg::with_name("Disable video")
                    .long("DISABLE-VIDEO")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("T/F")
                    .takes_value(false),
            )
            .arg(
                Arg::with_name("Visual override")
                    .long("VISUAL-OVERRIDE")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("T/F")
                    .takes_value(false),
            )
            .arg(
                Arg::with_name("Unknown short")
                    .long("UNKNOWN-SHORT")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NUMBER/RANGE")
                    .takes_value(false),
            )
            .arg(
                Arg::with_name("Offset from song start in editor (ms)")
                    .long("OFFSET-FROM-SONG-START-IN-EDITOR-MS")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NUMBER/RANGE")
                    .takes_value(false),
            )
            .arg(
                Arg::with_name("Mania scroll speed")
                    .long("MANIA-SCROLL-SPEED")
                    .multiple(false)
                    .required(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NUMBER/RANGE")
                    .takes_value(false),
            )
            .get_matches_from(args.into_iter());
        self.entry_size = Relational::from_matches(&matches, "Entry size")?;
        self.artist_name = EqualClone::from_matches(&matches, "Artist name")?;
        self.artist_name_unicode = EqualClone::from_matches(&matches, "Artist name unicode")?;
        self.song_title = EqualClone::from_matches(&matches, "Song title")?;
        self.song_title_unicode = EqualClone::from_matches(&matches, "Song title unicode")?;
        self.creator_name = EqualClone::from_matches(&matches, "Creator name")?;
        self.difficulty = EqualClone::from_matches(&matches, "Difficulty")?;
        self.audio_file_name = EqualClone::from_matches(&matches, "Audio file name")?;
        self.md5_beatmap_hash = EqualClone::from_matches(&matches, "MD5 beatmap hash")?;
        self.dotosu_file_name = EqualClone::from_matches(&matches, ".osu file name")?;
        self.ranked_status = EqualCopy::from_matches(&matches, "Ranked status")?;
        self.number_of_hitcircles = Relational::from_matches(&matches, "Number of hitcircles")?;
        self.number_of_sliders = Relational::from_matches(&matches, "Number of sliders")?;
        self.number_of_spinners = Relational::from_matches(&matches, "Number of spinners")?;
        self.last_modification_time =
            Relational::date_from_matches(&matches, "Last modification time")?;
        self.approach_rate = Relational::from_matches(&matches, "Approach rate")?;
        self.circle_size = Relational::from_matches(&matches, "Circle size")?;
        self.hp_drain = Relational::from_matches(&matches, "HP drain")?;
        self.overall_difficulty = Relational::from_matches(&matches, "Overall difficulty")?;
        self.slider_velocity = Relational::from_matches(&matches, "Slider velocity")?;
        self.num_mod_combo_star_ratings_standard = Relational::from_matches(
            &matches,
            "Number of precalculated mod combo star ratings (standard)",
        )?;
        self.num_mod_combo_star_ratings_taiko = Relational::from_matches(
            &matches,
            "Number of precalculated mod combo star ratings (taiko)",
        )?;
        self.num_mod_combo_star_ratings_ctb = Relational::from_matches(
            &matches,
            "Number of precalculated mod combo star ratings (CTB)",
        )?;
        self.num_mod_combo_star_ratings_mania = Relational::from_matches(
            &matches,
            "Number of precalculated mod combo star ratings (mania)",
        )?;
        self.drain_time = Relational::from_matches(&matches, "Drain time")?;
        self.total_time = Relational::from_matches(&matches, "Total time")?;
        self.preview_offset_from_start_ms =
            Relational::from_matches(&matches, "Preview offset from start (ms)")?;
        self.num_timing_points = Relational::from_matches(&matches, "Number of timing points")?;
        self.beatmap_id = Relational::from_matches(&matches, "Beatmap ID")?;
        self.beatmap_set_id = Relational::from_matches(&matches, "Beatmap set ID")?;
        self.thread_id = Relational::from_matches(&matches, "Thread ID")?;
        self.standard_grade = Relational::from_matches(&matches, "Standard grade")?;
        self.taiko_grade = Relational::from_matches(&matches, "Taiko grade")?;
        self.ctb_grade = Relational::from_matches(&matches, "CTB grade")?;
        self.mania_grade = Relational::from_matches(&matches, "Mania grade")?;
        self.local_offset = Relational::from_matches(&matches, "Local offset")?;
        self.stack_leniency = Relational::from_matches(&matches, "Stack leniency")?;
        self.gameplay_mode = EqualCopy::from_matches(&matches, "Gameplay mode")?;
        self.song_source = EqualClone::from_matches(&matches, "Song source")?;
        self.song_tags = EqualClone::from_matches(&matches, "Song tags")?;
        self.online_offset = Relational::from_matches(&matches, "Online offset")?;
        self.font_used_for_song_title =
            EqualClone::from_matches(&matches, "Font used for song title")?;
        self.unplayed = EqualCopy::bool_from_matches(&matches, "Unplayed")?;
        self.last_played = Relational::date_from_matches(&matches, "Last played")?;
        self.is_osz2 = EqualCopy::bool_from_matches(&matches, "Is OSZ2")?;
        self.beatmap_folder_name = EqualClone::from_matches(&matches, "Beatmap folder name")?;
        self.last_checked_against_repo =
            Relational::date_from_matches(&matches, "Last checked against repo")?;
        self.ignore_beatmap_sound = EqualCopy::bool_from_matches(&matches, "Ignore beatmap sound")?;
        self.ignore_beatmap_skin = EqualCopy::bool_from_matches(&matches, "Ignore beatmap skin")?;
        self.disable_storyboard = EqualCopy::bool_from_matches(&matches, "Disable storyboard")?;
        self.disable_video = EqualCopy::bool_from_matches(&matches, "Disable video")?;
        self.visual_override = EqualCopy::bool_from_matches(&matches, "Visual override")?;
        self.offset_from_song_start_in_editor_ms =
            Relational::from_matches(&matches, "Offset from song start in editor (ms)")?;
        self.mania_scroll_speed = Relational::from_matches(&matches, "Mania scroll speed")?;
        Ok(())
    }

    pub fn set_from_mask(&mut self, mask: &BeatmapMask) {
        self.entry_size.apply_mask(mask.entry_size);
        self.artist_name.apply_mask(mask.artist_name);
        self.artist_name_unicode
            .apply_mask(mask.artist_name_unicode);
        self.song_title.apply_mask(mask.song_title);
        self.song_title_unicode.apply_mask(mask.song_title_unicode);
        self.creator_name.apply_mask(mask.creator_name);
        self.difficulty.apply_mask(mask.difficulty);
        self.audio_file_name.apply_mask(mask.audio_file_name);
        self.md5_beatmap_hash.apply_mask(mask.md5_beatmap_hash);
        self.dotosu_file_name.apply_mask(mask.dotosu_file_name);
        self.ranked_status.apply_mask(mask.ranked_status);
        self.number_of_hitcircles
            .apply_mask(mask.number_of_hitcircles);
        self.number_of_sliders.apply_mask(mask.number_of_sliders);
        self.number_of_spinners.apply_mask(mask.number_of_spinners);
        self.last_modification_time
            .apply_mask(mask.last_modification_time);
        self.approach_rate.apply_mask(mask.approach_rate);
        self.circle_size.apply_mask(mask.circle_size);
        self.hp_drain.apply_mask(mask.hp_drain);
        self.overall_difficulty.apply_mask(mask.overall_difficulty);
        self.slider_velocity.apply_mask(mask.slider_velocity);
        self.num_mod_combo_star_ratings_standard
            .apply_mask(mask.num_mod_combo_star_ratings_standard);
        self.mod_combo_star_ratings_standard |= mask.mod_combo_star_ratings_standard;
        self.num_mod_combo_star_ratings_taiko
            .apply_mask(mask.num_mod_combo_star_ratings_taiko);
        self.mod_combo_star_ratings_taiko |= mask.mod_combo_star_ratings_taiko;
        self.num_mod_combo_star_ratings_ctb
            .apply_mask(mask.num_mod_combo_star_ratings_ctb);
        self.mod_combo_star_ratings_ctb |= mask.mod_combo_star_ratings_ctb;
        self.num_mod_combo_star_ratings_mania
            .apply_mask(mask.num_mod_combo_star_ratings_mania);
        self.mod_combo_star_ratings_mania |= mask.mod_combo_star_ratings_mania;
        self.drain_time.apply_mask(mask.drain_time);
        self.total_time.apply_mask(mask.total_time);
        self.preview_offset_from_start_ms
            .apply_mask(mask.preview_offset_from_start_ms);
        self.num_timing_points.apply_mask(mask.num_timing_points);
        self.timing_points |= mask.timing_points;
        self.beatmap_id.apply_mask(mask.beatmap_id);
        self.beatmap_set_id.apply_mask(mask.beatmap_set_id);
        self.thread_id.apply_mask(mask.thread_id);
        self.standard_grade.apply_mask(mask.standard_grade);
        self.taiko_grade.apply_mask(mask.taiko_grade);
        self.ctb_grade.apply_mask(mask.ctb_grade);
        self.mania_grade.apply_mask(mask.mania_grade);
        self.local_offset.apply_mask(mask.local_offset);
        self.stack_leniency.apply_mask(mask.stack_leniency);
        self.gameplay_mode.apply_mask(mask.gameplay_mode);
        self.song_source.apply_mask(mask.song_source);
        self.song_tags.apply_mask(mask.song_tags);
        self.online_offset.apply_mask(mask.online_offset);
        self.font_used_for_song_title
            .apply_mask(mask.font_used_for_song_title);
        self.unplayed.apply_mask(mask.unplayed);
        self.last_played.apply_mask(mask.last_played);
        self.is_osz2.apply_mask(mask.is_osz2);
        self.beatmap_folder_name
            .apply_mask(mask.beatmap_folder_name);
        self.last_checked_against_repo
            .apply_mask(mask.last_checked_against_repo);
        self.ignore_beatmap_sound
            .apply_mask(mask.ignore_beatmap_sound);
        self.ignore_beatmap_skin
            .apply_mask(mask.ignore_beatmap_skin);
        self.disable_storyboard.apply_mask(mask.disable_storyboard);
        self.disable_video.apply_mask(mask.disable_video);
        self.visual_override.apply_mask(mask.visual_override);
        self.unknown_short |= mask.unknown_short;
        if self.offset_from_song_start_in_editor_ms.is_ignore()
            && mask.offset_from_song_start_in_editor_ms
        {
            self.offset_from_song_start_in_editor_ms = LoadSetting::Load;
        }
        self.mania_scroll_speed.apply_mask(mask.mania_scroll_speed);
    }
}
