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
    num_timing_points: i32,
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

impl PartialBeatmap {
    pub fn read_from_bytes<T: ReadPartialVersionSpecificData>(mask: BeatmapMask, bytes: &[u8],
        i: &mut usize) -> ParseFileResult<Self> {
        let entry_size = T::maybe_read_entry_size(c, bytes, i)?;
        let artist_name = maybe_read_string_utf8(mask.artist_name, bytes, i,
            "non-Unicode artist name")?;
        let artist_name_unicode = maybe_read_string_utf8(mask.artist_name_unicode, bytes, i,
            "Unicode artist name")?;
        let song_title = maybe_read_string_utf8(mask.song_title, bytes, i,
            "non-Unicode song title")?;
        let song_title_unicode = maybe_read_string_utf8(mask.song_title_unicode, bytes, i,
            "Unicode song title")?;
        let creator_name = maybe_read_string_utf8(mask.creator_name, bytes, i, "creator name")?;
        let difficulty = maybe_read_string_utf8(mask.difficulty, byte, i, "difficulty")?;
        let difficulty = maybe_read_string_utf8(mask.difficulty, bytes, i, "difficulty")?;
        let audio_file_name = maybe_read_string_utf8(mask.audio_file_name, bytes, i,
            "audio file name")?;
        let md5_beatmap_hash = maybe_read_md5_hash(mask.md5_beatmap_hash, bytes, i)?;
        let dotosu_file_name = maybe_read_string_utf8(mask.dotosu_file_name, bytes, i,
            "corresponding .osu file name")?;
        let ranked_status = RankedStatus::maybe_read_from_bytes(mask.ranked_status, bytes, i)?;
        let number_of_hitcircles = maybe_read_short(mask.number_of_hitcircles, bytes, i)?;
        let number_of_sliders = maybe_read_short(mask.number_of_sliders, bytes, i)?;
        let number_of_spinners = maybe_read_short(mask.number_of_spinners, bytes, i)?;
        let last_modification_time = maybe_read_datetime(mask.last_modification_time, bytes, i)?;
        let approach_rate = T::maybe_read_arcshpod(mask.approach_rate, bytes, i)?;
        let circle_size = T::maybe_read_arcshpod(mask.circle_size, bytes, i)?;
        let hp_drain = T::maybe_read_arcshpod(mask.hp_drain, bytes, i)?;
        let overall_difficulty = T::maybe_read_arcshpod(mask.overall_difficulty, bytes, i)?;
        let slider_velocity = maybe_read_double(mask.slider_velocity, bytes, i)?;
        let (num_mcsr_standard, mcsr_standard) = T::maybe_read_mod_combo_star_ratings(
            mask.mod_combo_star_ratings_standard, bytes, i)?;
        let (num_mcsr_taiko, mcsr_taiko) = T::maybe_read_mod_combo_star_ratings(
            mask.mod_combo_star_ratings_taiko, bytes, i)?;
        let (num_mcsr_ctb, mcsr_ctb) = T::maybe_read_mod_combo_star_ratings(
            mask.mod_combo_star_ratings_ctb, bytes, i)?;
        let (num_mcsr_mania, mcsr_mania) = T::maybe_read_mod_combo_star_ratings(
            mask.mod_combo_star_ratings_mania, bytes, i)?;
        let drain_time = maybe_read_int(mask.drain_time, bytes, i)?;
        let total_time = maybe_read_int(mask.total_time, bytes, i)?;
        let preview_offset_from_start_ms = maybe_read_int(mask.preview_offset_from_start_ms, bytes, i)?;
        let num_timing_points = read_int(bytes, i)?;
        let mut timing_points = if mask.timing_points {
            let mut tmp = Vec::with_capacity(num_timing_points as usize);
            for _ in 0..num_timing_points {
                tmp.push(TimingPoint::read_from_bytes(bytes, i)?);
            }
            Some(tmp)
        } else {
            *i += num_timing_points as usize * 17;
            None
        };
        let beatmap_id = maybe_read_int(mask.beatmap_id, bytes, i)?;
        let beatmap_set_id = maybe_read_int(mask.beatmap_set_id, bytes, i)?;
        let thread_id = maybe_read_int(mask.thread_id, bytes, i)?;
        let standard_grade = maybe_read_byte(mask.standard_grade, bytes, i)?;
        let taiko_grade = maybe_read_byte(mask.taiko_grade, bytes, i)?;
        let ctb_grade = maybe_read_byte(mask.ctb_grade, bytes, i)?;
        let mania_grade = maybe_read_byte(mask.mania_grade, bytes, i)?;
        let local_offset = maybe_read_short(mask.local_offset, bytes, i)?;
        let stack_leniency = maybe_read_single(mask.stack_leniency, bytes, i)?;
        let gameplay_mode = GameplayMode::maybe_read_from_bytes(mask.gameplay_mode, bytes, i)?;
        let song_source = maybe_read_string_utf8(mask.song_source, bytes, i, "song source")?;
        let song_tags = maybe_read_string_utf8(mask.song_tags, bytes, i, "song tags")?;
        let online_offset = maybe_read_short(mask.online_offset, bytes, i)?;
        let font_used_for_song_title = maybe_read_string_utf8(mask.font_used_for_song_title, bytes,
            i, "font used for song title")?;
        let unplayed = maybe_read_boolean(mask.unplayed, bytes, i)?;
        let last_played = maybe_read_datetime(mask.last_played, bytes, i)?;
        let is_osz2 = maybe_read_boolean(mask.is_osz2, bytes, i)?;
        let beatmap_folder_name = maybe_read_string_utf8(mask.beatmap_folder_name, bytes, i,
            "folder name")?;
        let last_checked_against_repo = maybe_read_datetime(mask.last_checked_against_repo, bytes,
            i)?;
        let ignore_beatmap_sound = maybe_read_boolean(mask.ignore_beatmap_sound, bytes, i)?;
        let ignore_beatmap_skin = maybe_read_boolean(mask.ignore_beatmap_skin, bytes, i)?;
        let disable_storyboard = maybe_read_boolean(mask.disable_storyboard, bytes, i)?;
        let disable_video = maybe_read_boolean(mask.disable_video, bytes, i)?;
        let visual_override = maybe_read_boolean(mask.visual_override, bytes, i)?;
        let unknown_short = T::maybe_read_unknown_short(mask.unknown_short, bytes, i)?;
        let offset_from_song_start_in_editor_ms = maybe_read_int(
            mask.offset_from_song_start_in_editor_ms, bytes, i)?;
        let ania_scroll_speed = maybe_read_byte(mask.mania_scroll_speed, bytes, i)?;
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