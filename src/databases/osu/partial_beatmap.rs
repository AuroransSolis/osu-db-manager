use chrono::NaiveDate;

use crate::databases::osu::{primitives::*, versions::ReadPartialVersionSpecificData};
use crate::deserialize_primitives::*;
use crate::load_settings::osu::beatmap_load_settings::BeatmapLoadSettings;
use crate::masks::osu_mask::BeatmapMask;
use crate::maybe_deserialize_primitives::*;
use crate::read_error::ParseFileResult;

#[derive(Clone, Debug)]
pub struct PartialBeatmap {
    pub entry_size: Option<i32>,
    pub artist_name: Option<String>,
    pub artist_name_unicode: Option<String>,
    pub song_title: Option<String>,
    pub song_title_unicode: Option<String>,
    pub creator_name: Option<String>,
    pub difficulty: Option<String>,
    pub audio_file_name: Option<String>,
    pub md5_beatmap_hash: Option<String>,
    pub dotosu_file_name: Option<String>,
    pub ranked_status: Option<RankedStatus>,
    pub number_of_hitcircles: Option<i16>,
    pub number_of_sliders: Option<i16>,
    pub number_of_spinners: Option<i16>,
    pub last_modification_time: Option<NaiveDate>,
    pub approach_rate: Option<ByteSingle>,
    pub circle_size: Option<ByteSingle>,
    pub hp_drain: Option<ByteSingle>,
    pub overall_difficulty: Option<ByteSingle>,
    pub slider_velocity: Option<f64>,
    pub num_mod_combo_star_ratings_standard: Option<i32>,
    pub mod_combo_star_ratings_standard: Option<Vec<(i32, f64)>>,
    pub num_mod_combo_star_ratings_taiko: Option<i32>,
    pub mod_combo_star_ratings_taiko: Option<Vec<(i32, f64)>>,
    pub num_mod_combo_star_ratings_ctb: Option<i32>,
    pub mod_combo_star_ratings_ctb: Option<Vec<(i32, f64)>>,
    pub num_mod_combo_star_ratings_mania: Option<i32>,
    pub mod_combo_star_ratings_mania: Option<Vec<(i32, f64)>>,
    pub drain_time: Option<i32>,
    pub total_time: Option<i32>,
    pub preview_offset_from_start_ms: Option<i32>,
    pub num_timing_points: Option<i32>,
    pub timing_points: Option<Vec<TimingPoint>>,
    pub beatmap_id: Option<i32>,
    pub beatmap_set_id: Option<i32>,
    pub thread_id: Option<i32>,
    pub standard_grade: Option<u8>,
    pub taiko_grade: Option<u8>,
    pub ctb_grade: Option<u8>,
    pub mania_grade: Option<u8>,
    pub local_offset: Option<i16>,
    pub stack_leniency: Option<f32>,
    pub gameplay_mode: Option<GameplayMode>,
    pub song_source: Option<String>,
    pub song_tags: Option<String>,
    pub online_offset: Option<i16>,
    pub font_used_for_song_title: Option<String>,
    pub unplayed: Option<bool>,
    pub last_played: Option<NaiveDate>,
    pub is_osz2: Option<bool>,
    pub beatmap_folder_name: Option<String>,
    pub last_checked_against_repo: Option<NaiveDate>,
    pub ignore_beatmap_sound: Option<bool>,
    pub ignore_beatmap_skin: Option<bool>,
    pub disable_storyboard: Option<bool>,
    pub disable_video: Option<bool>,
    pub visual_override: Option<bool>,
    pub unknown_short: Option<i16>,
    pub offset_from_song_start_in_editor_ms: Option<i32>,
    pub mania_scroll_speed: Option<u8>,
}

impl PartialBeatmap {
    pub fn read_from_bytes<T: ReadPartialVersionSpecificData>(
        settings: &BeatmapLoadSettings,
        bytes: &[u8],
        i: &mut usize,
    ) -> ParseFileResult<Self> {
        let mut skip = false;
        let entry_size = T::maybe_read_entry_size(settings.entry_size, &mut skip, bytes, i)?;
        let artist_name = maybe_read_string_utf8(
            settings.artist_name,
            &mut skip,
            bytes,
            i,
            "non-Unicode artist name",
        )?;
        let artist_name_unicode = maybe_read_string_utf8(
            settings.artist_name_unicode,
            &mut skip,
            bytes,
            i,
            "Unicode artist name",
        )?;
        let song_title = maybe_read_string_utf8(
            settings.song_title,
            &mut skip,
            bytes,
            i,
            "non-Unicode song title",
        )?;
        let song_title_unicode = maybe_read_string_utf8(
            settings.song_title_unicode,
            &mut skip,
            bytes,
            i,
            "Unicode song title",
        )?;
        let creator_name =
            maybe_read_string_utf8(settings.creator_name, &mut skip, bytes, i, "creator name")?;
        let difficulty =
            maybe_read_string_utf8(settings.difficulty, &mut skip, bytes, i, "difficulty")?;
        let audio_file_name = maybe_read_string_utf8(
            settings.audio_file_name,
            &mut skip,
            bytes,
            i,
            "audio file name",
        )?;
        let md5_beatmap_hash = maybe_read_md5_hash(settings.md5_beatmap_hash, &mut skip, bytes, i)?;
        let dotosu_file_name = maybe_read_string_utf8(
            settings.dotosu_file_name,
            &mut skip,
            bytes,
            i,
            "corresponding .osu file name",
        )?;
        let ranked_status =
            RankedStatus::maybe_read_from_bytes(settings.ranked_status, &mut skip, bytes, i)?;
        let number_of_hitcircles =
            maybe_read_short(settings.number_of_hitcircles, &mut skip, bytes, i)?;
        let number_of_sliders = maybe_read_short(settings.number_of_sliders, &mut skip, bytes, i)?;
        let number_of_spinners =
            maybe_read_short(settings.number_of_spinners, &mut skip, bytes, i)?;
        let last_modification_time =
            maybe_read_datetime(settings.last_modification_time, &mut skip, bytes, i)?;
        let approach_rate = T::maybe_read_arcshpod(settings.approach_rate, &mut skip, bytes, i)?;
        let circle_size = T::maybe_read_arcshpod(settings.circle_size, &mut skip, bytes, i)?;
        let hp_drain = T::maybe_read_arcshpod(settings.hp_drain, &mut skip, bytes, i)?;
        let overall_difficulty =
            T::maybe_read_arcshpod(settings.overall_difficulty, &mut skip, bytes, i)?;
        let slider_velocity = maybe_read_double(settings.slider_velocity, &mut skip, bytes, i)?;
        let (num_mcsr_standard, mcsr_standard) =
            T::maybe_read_mod_combo_star_ratings(skip, bytes, i)?;
        let (num_mcsr_taiko, mcsr_taiko) = T::maybe_read_mod_combo_star_ratings(skip, bytes, i)?;
        let (num_mcsr_ctb, mcsr_ctb) = T::maybe_read_mod_combo_star_ratings(skip, bytes, i)?;
        let (num_mcsr_mania, mcsr_mania) = T::maybe_read_mod_combo_star_ratings(skip, bytes, i)?;
        let drain_time = maybe_read_int(settings.drain_time, &mut skip, bytes, i)?;
        let total_time = maybe_read_int(settings.total_time, &mut skip, bytes, i)?;
        let preview_offset_from_start_ms =
            maybe_read_int(settings.preview_offset_from_start_ms, &mut skip, bytes, i)?;
        let num_timing_points = read_int(bytes, i)?;
        let timing_points = if mask.timing_points {
            if num_timing_points == 0 {
                None
            } else {
                let mut tmp = Vec::with_capacity(num_timing_points as usize);
                for _ in 0..num_timing_points {
                    tmp.push(TimingPoint::read_from_bytes(bytes, i)?);
                }
                Some(tmp)
            }
        } else {
            *i += num_timing_points as usize * 17;
            None
        };
        let num_timing_points = if settings.timing_points.is_ignore() {
            None
        } else {
            Some(num_timing_points)
        };
        let beatmap_id = maybe_read_int(settings.beatmap_id, &mut skip, bytes, i)?;
        let beatmap_set_id = maybe_read_int(settings.beatmap_set_id, &mut skip, bytes, i)?;
        let thread_id = maybe_read_int(settings.thread_id, &mut skip, bytes, i)?;
        let standard_grade = maybe_read_byte(settings.standard_grade, &mut skip, bytes, i)?;
        let taiko_grade = maybe_read_byte(settings.taiko_grade, &mut skip, bytes, i)?;
        let ctb_grade = maybe_read_byte(settings.ctb_grade, &mut skip, bytes, i)?;
        let mania_grade = maybe_read_byte(settings.mania_grade, &mut skip, bytes, i)?;
        let local_offset = maybe_read_short(settings.local_offset, &mut skip, bytes, i)?;
        let stack_leniency = maybe_read_single(settings.stack_leniency, &mut skip, bytes, i)?;
        let gameplay_mode =
            GameplayMode::maybe_read_from_bytes(settings.gameplay_mode, &mut skip, bytes, i)?;
        let song_source =
            maybe_read_string_utf8(settings.song_source, &mut skip, bytes, i, "song source")?;
        let song_tags =
            maybe_read_string_utf8(settings.song_tags, &mut skip, bytes, i, "song tags")?;
        let online_offset = maybe_read_short(settings.online_offset, &mut skip, bytes, i)?;
        let font_used_for_song_title = maybe_read_string_utf8(
            settings.font_used_for_song_title,
            &mut skip,
            bytes,
            i,
            "font used for song title",
        )?;
        let unplayed = maybe_read_boolean(settings.unplayed, &mut skip, bytes, i)?;
        let last_played = maybe_read_datetime(settings.last_played, &mut skip, bytes, i)?;
        let is_osz2 = maybe_read_boolean(settings.is_osz2, &mut skip, bytes, i)?;
        let beatmap_folder_name = maybe_read_string_utf8(
            settings.beatmap_folder_name,
            &mut skip,
            bytes,
            i,
            "folder name",
        )?;
        let last_checked_against_repo =
            maybe_read_datetime(settings.last_checked_against_repo, &mut skip, bytes, i)?;
        let ignore_beatmap_sound =
            maybe_read_boolean(settings.ignore_beatmap_sound, &mut skip, bytes, i)?;
        let ignore_beatmap_skin =
            maybe_read_boolean(settings.ignore_beatmap_skin, &mut skip, bytes, i)?;
        let disable_storyboard =
            maybe_read_boolean(settings.disable_storyboard, &mut skip, bytes, i)?;
        let disable_video = maybe_read_boolean(settings.disable_video, &mut skip, bytes, i)?;
        let visual_override = maybe_read_boolean(settings.visual_override, &mut skip, bytes, i)?;
        let unknown_short = T::maybe_read_unknown_short(skip, bytes, i)?;
        let offset_from_song_start_in_editor_ms = maybe_read_int(
            settings.offset_from_song_start_in_editor_ms,
            &mut skip,
            bytes,
            i,
        )?;
        let mania_scroll_speed = maybe_read_byte(settings.mania_scroll_speed, &mut skip, bytes, i)?;
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
            mania_scroll_speed,
        })
    }
}
