use chrono::NaiveDate;

use crate::databases::osu::{primitives::*, versions::ReadVersionSpecificData};
use crate::deserialize_primitives::*;
use crate::read_error::{DbFileParseError, ParseErrorKind::*, ParseFileResult};

/// Beatmap struct according to documentation linked in README.
#[derive(Clone, Debug)]
pub struct Beatmap<'a> {
    pub entry_size: Option<i32>,
    pub artist_name: Option<&'a str>,
    pub artist_name_unicode: Option<&'a str>,
    pub song_title: Option<&'a str>,
    pub song_title_unicode: Option<&'a str>,
    pub creator_name: Option<&'a str>,
    pub difficulty: Option<&'a str>,
    pub audio_file_name: Option<&'a str>,
    pub md5_beatmap_hash: &'a str,
    pub dotosu_file_name: Option<&'a str>,
    pub ranked_status: RankedStatus,
    pub number_of_hitcircles: i16,
    pub number_of_sliders: i16,
    pub number_of_spinners: i16,
    pub last_modification_time: NaiveDate,
    pub approach_rate: ByteSingle,
    pub circle_size: ByteSingle,
    pub hp_drain: ByteSingle,
    pub overall_difficulty: ByteSingle,
    pub slider_velocity: f64,
    pub num_mod_combo_star_ratings_standard: Option<i32>,
    pub mod_combo_star_ratings_standard: Option<Vec<(i32, f64)>>,
    pub num_mod_combo_star_ratings_taiko: Option<i32>,
    pub mod_combo_star_ratings_taiko: Option<Vec<(i32, f64)>>,
    pub num_mod_combo_star_ratings_ctb: Option<i32>,
    pub mod_combo_star_ratings_ctb: Option<Vec<(i32, f64)>>,
    pub num_mod_combo_star_ratings_mania: Option<i32>,
    pub mod_combo_star_ratings_mania: Option<Vec<(i32, f64)>>,
    pub drain_time: i32,
    pub total_time: i32,
    pub preview_offset_from_start_ms: i32,
    pub num_timing_points: i32,
    pub timing_points: Vec<TimingPoint>,
    pub beatmap_id: i32,
    pub beatmap_set_id: i32,
    pub thread_id: i32,
    pub standard_grade: u8,
    pub taiko_grade: u8,
    pub ctb_grade: u8,
    pub mania_grade: u8,
    pub local_offset: i16,
    pub stack_leniency: f32,
    pub gameplay_mode: GameplayMode,
    pub song_source: Option<&'a str>,
    pub song_tags: Option<&'a str>,
    pub online_offset: i16,
    pub font_used_for_song_title: Option<&'a str>,
    pub unplayed: bool,
    pub last_played: NaiveDate,
    pub is_osz2: bool,
    pub beatmap_folder_name: Option<&'a str>,
    pub last_checked_against_repo: NaiveDate,
    pub ignore_beatmap_sound: bool,
    pub ignore_beatmap_skin: bool,
    pub disable_storyboard: bool,
    pub disable_video: bool,
    pub visual_override: bool,
    pub unknown_short: Option<i16>,
    pub offset_from_song_start_in_editor_ms: i32,
    pub mania_scroll_speed: u8,
}

impl<'a> Beatmap<'a> {
    pub fn read_from_bytes<T: ReadVersionSpecificData>(
        bytes: &'a [u8],
        i: &mut usize,
    ) -> ParseFileResult<Self> {
        let entry_size = T::read_entry_size(bytes, i)?;
        let artist_name = read_str_utf8(bytes, i, "non-Unicode artist name")?;
        let artist_name_unicode = read_str_utf8(bytes, i, "Unicode artist name")?;
        let song_title = read_str_utf8(bytes, i, "non-Unicode song title")?;
        let song_title_unicode = read_str_utf8(bytes, i, "Unicode song title")?;
        let creator_name = read_player_name(bytes, i).map_err(|_| {
            let msg = format!("Error reading creator name.");
            DbFileParseError::new(PrimitiveError, msg.as_str())
        })?;
        let difficulty = read_str_utf8(bytes, i, "difficulty")?;
        let audio_file_name = read_str_utf8(bytes, i, "audio file name")?;
        let md5_beatmap_hash = read_md5_hash(bytes, i)?;
        let dotosu_file_name = read_str_utf8(bytes, i, "corresponding .osu file name")?;
        let ranked_status = RankedStatus::read_from_bytes(bytes, i)?;
        let number_of_hitcircles = read_short(bytes, i)?;
        let number_of_sliders = read_short(bytes, i)?;
        let number_of_spinners = read_short(bytes, i)?;
        let last_modification_time = read_datetime(bytes, i)?;
        let approach_rate = T::read_arcshpod(bytes, i)?;
        let circle_size = T::read_arcshpod(bytes, i)?;
        let hp_drain = T::read_arcshpod(bytes, i)?;
        let overall_difficulty = T::read_arcshpod(bytes, i)?;
        let slider_velocity = read_double(bytes, i)?;
        let (num_mcsr_standard, mcsr_standard) = T::read_mod_combo_star_ratings(bytes, i)?;
        let (num_mcsr_taiko, mcsr_taiko) = T::read_mod_combo_star_ratings(bytes, i)?;
        let (num_mcsr_ctb, mcsr_ctb) = T::read_mod_combo_star_ratings(bytes, i)?;
        let (num_mcsr_mania, mcsr_mania) = T::read_mod_combo_star_ratings(bytes, i)?;
        let drain_time = read_int(bytes, i)?;
        let total_time = read_int(bytes, i)?;
        let preview_offset_from_start_ms = read_int(bytes, i)?;
        let num_timing_points = read_int(bytes, i)?;
        let mut timing_points = Vec::with_capacity(num_timing_points as usize);
        for _ in 0..num_timing_points {
            timing_points.push(TimingPoint::read_from_bytes(bytes, i)?);
        }
        let beatmap_id = read_int(bytes, i)?;
        let beatmap_set_id = read_int(bytes, i)?;
        let thread_id = read_int(bytes, i)?;
        let standard_grade = read_byte(bytes, i)?;
        let taiko_grade = read_byte(bytes, i)?;
        let ctb_grade = read_byte(bytes, i)?;
        let mania_grade = read_byte(bytes, i)?;
        let local_offset = read_short(bytes, i)?;
        let stack_leniency = read_single(bytes, i)?;
        let gameplay_mode = GameplayMode::read_from_bytes(bytes, i)?;
        let song_source = read_str_utf8(bytes, i, "song source")?;
        let song_tags = read_str_utf8(bytes, i, "song tags")?;
        let online_offset = read_short(bytes, i)?;
        let font_used_for_song_title = read_str_utf8(bytes, i, "font used for song title")?;
        let unplayed = read_boolean(bytes, i)?;
        let last_played = read_datetime(bytes, i)?;
        let is_osz2 = read_boolean(bytes, i)?;
        let beatmap_folder_name = read_str_utf8(bytes, i, "folder name")?;
        let last_checked_against_repo = read_datetime(bytes, i)?;
        let ignore_beatmap_sound = read_boolean(bytes, i)?;
        let ignore_beatmap_skin = read_boolean(bytes, i)?;
        let disable_storyboard = read_boolean(bytes, i)?;
        let disable_video = read_boolean(bytes, i)?;
        let visual_override = read_boolean(bytes, i)?;
        let unknown_short = T::read_unknown_short(bytes, i)?;
        let offset_from_song_start_in_editor_ms = read_int(bytes, i)?;
        let mania_scroll_speed = read_byte(bytes, i)?;
        Ok(Beatmap {
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
            num_mod_combo_star_ratings_standard: num_mcsr_standard,
            mod_combo_star_ratings_standard: mcsr_standard,
            num_mod_combo_star_ratings_taiko: num_mcsr_taiko,
            mod_combo_star_ratings_taiko: mcsr_taiko,
            num_mod_combo_star_ratings_ctb: num_mcsr_ctb,
            mod_combo_star_ratings_ctb: mcsr_ctb,
            num_mod_combo_star_ratings_mania: num_mcsr_mania,
            mod_combo_star_ratings_mania: mcsr_mania,
            drain_time,
            total_time,
            preview_offset_from_start_ms,
            num_timing_points,
            timing_points,
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
            unknown_short,
            offset_from_song_start_in_editor_ms,
            mania_scroll_speed,
        })
    }
}
