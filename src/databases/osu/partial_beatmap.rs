use std::time::SystemTime;

use crate::deserialize_primitives::*;
use crate::maybe_deserialize_primitives::*;
use crate::read_error::{ParseFileResult, DbFileParseError, ParseErrorKind::PrimitiveError};
use crate::databases::osu::{primitives::*, versions::ReadPartialVersionSpecificData};
use crate::masks::osu_mask::BeatmapMask;

#[derive(Clone, Debug)]
pub struct PartialBeatmap {
    entry_size: Option<i32>,
    artist_name: Option<String>,
    artist_name_unicode: Option<String>,
    song_title: Option<String>,
    song_title_unicode: Option<String>,
    creator_name: Option<String>,
    difficulty: Option<String>,
    audio_file_name: Option<String>,
    md5_beatmap_hash: Option<String>,
    dotosu_file_name: Option<String>,
    ranked_status: Option<RankedStatus>,
    number_of_hitcircles: Option<i16>,
    number_of_sliders: Option<i16>,
    number_of_spinners: Option<i16>,
    last_modification_time: Option<SystemTime>,
    approach_rate: Option<ByteSingle>,
    circle_size: Option<ByteSingle>,
    hp_drain: Option<ByteSingle>,
    overall_difficulty: Option<ByteSingle>,
    slider_velocity: Option<f64>,
    num_mod_combo_star_ratings_standard: Option<i32>,
    mod_combo_star_ratings_standard: Option<Vec<(i32, f64)>>,
    num_mod_combo_star_ratings_taiko: Option<i32>,
    mod_combo_star_ratings_taiko: Option<Vec<(i32, f64)>>,
    num_mod_combo_star_ratings_ctb: Option<i32>,
    mod_combo_star_ratings_ctb: Option<Vec<(i32, f64)>>,
    num_mod_combo_star_ratings_mania: Option<i32>,
    mod_combo_star_ratings_mania: Option<Vec<(i32, f64)>>,
    drain_time: Option<i32>,
    total_time: Option<i32>,
    preview_offset_from_start_ms: Option<i32>,
    num_timing_points: Option<i32>,
    timing_points: Option<Vec<TimingPoint>>,
    beatmap_id: Option<i32>,
    beatmap_set_id: Option<i32>,
    thread_id: Option<i32>,
    standard_grade: Option<u8>,
    taiko_grade: Option<u8>,
    ctb_grade: Option<u8>,
    mania_grade: Option<u8>,
    local_offset: Option<i16>,
    stack_leniency: Option<f32>,
    gameplay_mode: Option<GameplayMode>,
    song_source: Option<String>,
    song_tags: Option<String>,
    online_offset: Option<i16>,
    font_used_for_song_title: Option<String>,
    unplayed: Option<bool>,
    last_played: Option<SystemTime>,
    is_osz2: Option<bool>,
    beatmap_folder_name: Option<String>,
    last_checked_against_repo: Option<SystemTime>,
    ignore_beatmap_sound: Option<bool>,
    ignore_beatmap_skin: Option<bool>,
    disable_storyboard: Option<bool>,
    disable_video: Option<bool>,
    visual_override: Option<bool>,
    unknown_short: Option<i16>,
    offset_from_song_start_in_editor_ms: Option<i32>,
    mania_scroll_speed: Option<u8>
}

macro_rules! read_or_advance_sized {
    ($condition:expr, $true_branch:expr, $index:ident + $amt:literal) => {{
        if $condition {
            Some($true_branch)
        } else {
            *$index += $amt;
            None
        }
    }};
}

fn read_or_advance_sized<T>(condition: bool, bytes: &[u8], i: &mut usize, increment: usize,
    read: fn(&[u8], &mut usize) -> T) -> ParseFileResult<Option<T>> {
    if condition {
        Some(read(bytes, i)?)
    } else {
        *i += increment;
        None
    }
}

macro_rules! read_or_advance_string {
    ($condition:expr, read_string_utf8($bytes:ident, $index:ident, $field:literal)) => {{
        if $condition {
            Some(read_string_utf8($bytes, $index, $field)?)
        } else {
            let indicator = read_byte($bytes, $index)?;
            if indicator == 0 {
                None
            } else if indicator == 0x0b {
                let len = read_uleb128($bytes, $index)?;
                *$index += len;
                None
            } else {
                return Err(DbFileParseError::new(PrimitiveError, format!("Read invalid string \
                    indicator for {}", $field)));
            }
        }
    }};
}

impl PartialBeatmap {
    pub fn read_from_bytes<T: ReadPartialVersionSpecificData>(mask: BeatmapMask, bytes: &[u8],
        i: &mut usize) -> ParseFileResult<Self> {
        let entry_size = T::maybe_read_entry_size(c, bytes, i)?;
        let entry_size = read_or_advance_sized(mask.entry_size, bytes, i, 4,
            |b, ind| T::read_entry_size(b, ind))?;
        let entry_size = read_or_advance_sized!(mask.entry_size, T::read_entry_size(bytes, i)?,
            i + 4);
        let artist_name = maybe_read_string_utf8(mask.artist_name, bytes, i,
            "non-Unicode artist name")?;
        let artist_name_unicode = maybe_read_string_utf8(mask.artist_name_unicode, bytes, i,
            "Unicode artist name")?;
        let song_title = maybe_read_string_utf8(mask.song_title, bytes, i,
            "non-Unicode song title")?;
        let song_title_unicode = maybe_read_string_utf8(mask.song_title_unicode, bytes, i,
            "Unicode song title")?;
        let creator_name = maybe_read_string_utf8(mask.creator_name, bytes, i, "creator name");
        let difficulty = maybe_read_string_utf8(mask.difficulty, byte, i, "difficulty");
        let difficulty = maybe_read_string_utf8(mask.difficulty, bytes, i, "difficulty")?;
        let audio_file_name = maybe_read_string_utf8(mask.audio_file_name, bytes, i,
            "audio file name")?;
        let md5_beatmap_hash = maybe_read_md5_hash(mask.md5_beatmap_hash, bytes, i)?;
        let dotosu_file_name = maybe_read_string_utf8(mask.dotosu_file_name, bytes, i,
            "corresponding .osu file name")?;
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
        let song_source = read_string_utf8(bytes, i, "song source")?;
        let song_tags = read_string_utf8(bytes, i, "song tags")?;
        let online_offset = read_short(bytes, i)?;
        let font_used_for_song_title = read_string_utf8(bytes, i, "font used for song title")?;
        let unplayed = read_boolean(bytes, i)?;
        let last_played = read_datetime(bytes, i)?;
        let is_osz2 = read_boolean(bytes, i)?;
        let beatmap_folder_name = read_string_utf8(bytes, i, "folder name")?;
        let last_checked_against_repo = read_datetime(bytes, i)?;
        let ignore_beatmap_sound = read_boolean(bytes, i)?;
        let ignore_beatmap_skin = read_boolean(bytes, i)?;
        let disable_storyboard = read_boolean(bytes, i)?;
        let disable_video = read_boolean(bytes, i)?;
        let visual_override = read_boolean(bytes, i)?;
        let unknown_short = T::read_unknown_short(bytes, i)?;
        let offset_from_song_start_in_editor_ms = read_int(bytes, i)?;
        let mania_scroll_speed = read_byte(bytes, i)?;
        Ok(PartialBeatmap {
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
            mania_scroll_speed
        })
    }
}