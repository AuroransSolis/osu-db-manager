use crate::databases::osu::{primitives::*, versions::ReadPartialVersionSpecificData};
use crate::deserialize_primitives::*;
use crate::load_settings::osu::beatmap_load_settings::BeatmapLoadSettings;
use crate::maybe_deserialize_primitives::*;
use crate::read_error::ParseFileResult;
use crate::{masks::osu_mask::BeatmapMask, maybe_print, maybe_print_vec};
use chrono::NaiveDate;

/// Partial beatmap struct - this is like a regular `Beatmap`, except it's possible to skip parsing
/// arbitrary fields. Skipped fields have a `None` value. The idea behind the `PartialBeatmap` is
/// only parsing the data that the user requests through a query and/or show options.
#[derive(Clone, Debug)]
pub struct PartialBeatmap<'a> {
    pub entry_size: Option<i32>,
    pub artist_name: Option<&'a str>,
    pub artist_name_unicode: Option<&'a str>,
    pub song_title: Option<&'a str>,
    pub song_title_unicode: Option<&'a str>,
    pub creator_name: Option<&'a str>,
    pub difficulty: Option<&'a str>,
    pub audio_file_name: Option<&'a str>,
    pub md5_beatmap_hash: Option<&'a str>,
    pub dotosu_file_name: Option<&'a str>,
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
    pub song_source: Option<&'a str>,
    pub song_tags: Option<&'a str>,
    pub online_offset: Option<i16>,
    pub font_used_for_song_title: Option<&'a str>,
    pub unplayed: Option<bool>,
    pub last_played: Option<NaiveDate>,
    pub is_osz2: Option<bool>,
    pub beatmap_folder_name: Option<&'a str>,
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

impl<'a> PartialBeatmap<'a> {
    pub fn read_from_bytes<T: ReadPartialVersionSpecificData>(
        settings: &BeatmapLoadSettings,
        bytes: &'a [u8],
        i: &mut usize,
    ) -> ParseFileResult<Self> {
        let mut skip = false;
        let entry_size = T::maybe_read_entry_size(settings.entry_size, &mut skip, bytes, i)?;
        let artist_name = maybe_read_str_utf8(
            &settings.artist_name,
            &mut skip,
            bytes,
            i,
            "non-Unicode artist name",
        )?;
        let artist_name_unicode = maybe_read_str_utf8(
            &settings.artist_name_unicode,
            &mut skip,
            bytes,
            i,
            "Unicode artist name",
        )?;
        let song_title = maybe_read_str_utf8(
            &settings.song_title,
            &mut skip,
            bytes,
            i,
            "non-Unicode song title",
        )?;
        let song_title_unicode = maybe_read_str_utf8(
            &settings.song_title_unicode,
            &mut skip,
            bytes,
            i,
            "Unicode song title",
        )?;
        let creator_name =
            maybe_read_str_utf8(&settings.creator_name, &mut skip, bytes, i, "creator name")?;
        let difficulty =
            maybe_read_str_utf8(&settings.difficulty, &mut skip, bytes, i, "difficulty")?;
        let audio_file_name = maybe_read_str_utf8(
            &settings.audio_file_name,
            &mut skip,
            bytes,
            i,
            "audio file name",
        )?;
        let md5_beatmap_hash =
            maybe_read_md5_hash(&settings.md5_beatmap_hash, &mut skip, bytes, i)?;
        let dotosu_file_name = maybe_read_str_utf8(
            &settings.dotosu_file_name,
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
        let (num_mcsr_standard, mcsr_standard) = T::maybe_read_mod_combo_star_ratings(
            settings.num_mod_combo_star_ratings_standard,
            settings.mod_combo_star_ratings_standard,
            &mut skip,
            bytes,
            i,
        )?;
        let (num_mcsr_taiko, mcsr_taiko) = T::maybe_read_mod_combo_star_ratings(
            settings.num_mod_combo_star_ratings_taiko,
            settings.mod_combo_star_ratings_taiko,
            &mut skip,
            bytes,
            i,
        )?;
        let (num_mcsr_ctb, mcsr_ctb) = T::maybe_read_mod_combo_star_ratings(
            settings.num_mod_combo_star_ratings_ctb,
            settings.mod_combo_star_ratings_ctb,
            &mut skip,
            bytes,
            i,
        )?;
        let (num_mcsr_mania, mcsr_mania) = T::maybe_read_mod_combo_star_ratings(
            settings.num_mod_combo_star_ratings_mania,
            settings.mod_combo_star_ratings_mania,
            &mut skip,
            bytes,
            i,
        )?;
        let drain_time = maybe_read_int(settings.drain_time, &mut skip, bytes, i)?;
        let total_time = maybe_read_int(settings.total_time, &mut skip, bytes, i)?;
        let preview_offset_from_start_ms =
            maybe_read_int(settings.preview_offset_from_start_ms, &mut skip, bytes, i)?;
        let num_timing_points = read_int(bytes, i)?;
        let timing_points = if settings.timing_points {
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
        let num_timing_points = if settings.timing_points {
            Some(num_timing_points)
        } else {
            None
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
            maybe_read_str_utf8(&settings.song_source, &mut skip, bytes, i, "song source")?;
        let song_tags = maybe_read_str_utf8(&settings.song_tags, &mut skip, bytes, i, "song tags")?;
        let online_offset = maybe_read_short(settings.online_offset, &mut skip, bytes, i)?;
        let font_used_for_song_title = maybe_read_str_utf8(
            &settings.font_used_for_song_title,
            &mut skip,
            bytes,
            i,
            "font used for song title",
        )?;
        let unplayed = maybe_read_boolean(settings.unplayed, &mut skip, bytes, i)?;
        let last_played = maybe_read_datetime(settings.last_played, &mut skip, bytes, i)?;
        let is_osz2 = maybe_read_boolean(settings.is_osz2, &mut skip, bytes, i)?;
        let beatmap_folder_name = maybe_read_str_utf8(
            &settings.beatmap_folder_name,
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
        let unknown_short =
            T::maybe_read_unknown_short(settings.unknown_short, &mut skip, bytes, i)?;
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

    pub fn display(&self, show: BeatmapMask) {
        maybe_print!(show.entry_size, self.entry_size, "    ");
        maybe_print!(show.artist_name, self.artist_name, "    ");
        maybe_print!(show.artist_name_unicode, self.artist_name_unicode, "    ");
        maybe_print!(show.song_title, self.song_title, "    ");
        maybe_print!(show.song_title_unicode, self.song_title_unicode, "    ");
        maybe_print!(show.creator_name, self.creator_name, "    ");
        maybe_print!(show.difficulty, self.difficulty, "    ");
        maybe_print!(show.audio_file_name, self.audio_file_name, "    ");
        maybe_print!(show.md5_beatmap_hash, self.md5_beatmap_hash, "    ");
        maybe_print!(show.dotosu_file_name, self.dotosu_file_name, "    ");
        maybe_print!(show.ranked_status, self.ranked_status, "    ");
        maybe_print!(show.number_of_hitcircles, self.number_of_hitcircles, "    ");
        maybe_print!(show.number_of_sliders, self.number_of_sliders, "    ");
        maybe_print!(show.number_of_spinners, self.number_of_spinners, "    ");
        maybe_print!(
            show.last_modification_time,
            self.last_modification_time,
            "    "
        );
        maybe_print!(show.approach_rate, self.approach_rate, "    ");
        maybe_print!(show.circle_size, self.circle_size, "    ");
        maybe_print!(show.hp_drain, self.hp_drain, "    ");
        maybe_print!(show.overall_difficulty, self.overall_difficulty, "    ");
        maybe_print!(show.slider_velocity, self.slider_velocity, "    ");
        maybe_print!(
            show.num_mod_combo_star_ratings_standard,
            self.num_mod_combo_star_ratings_standard,
            "    "
        );
        if show.mod_combo_star_ratings_standard && self.mod_combo_star_ratings_standard.is_some() {
            println!("    MCSR osu!standard {{");
            for (mods, star_rating) in self.mod_combo_star_ratings_standard.as_ref().unwrap() {
                println!("        mods: {:32b} | star rating: {}*", mods, star_rating);
            }
            println!("    }}");
        }
        maybe_print!(
            show.num_mod_combo_star_ratings_taiko,
            self.num_mod_combo_star_ratings_taiko,
            "    "
        );
        if show.mod_combo_star_ratings_taiko && self.mod_combo_star_ratings_taiko.is_some() {
            println!("    MCSR osu!taiko {{");
            for (mods, star_rating) in self.mod_combo_star_ratings_taiko.as_ref().unwrap() {
                println!("        mods: {:32b} | star rating: {}*", mods, star_rating);
            }
            println!("    }}");
        }
        maybe_print!(
            show.num_mod_combo_star_ratings_ctb,
            self.num_mod_combo_star_ratings_ctb,
            "    "
        );
        if show.mod_combo_star_ratings_ctb && self.mod_combo_star_ratings_ctb.is_some() {
            println!("    MCSR osu!ctb {{");
            for (mods, star_rating) in self.mod_combo_star_ratings_ctb.as_ref().unwrap() {
                println!("        mods: {:32b} | star rating: {}*", mods, star_rating);
            }
            println!("    }}");
        }
        maybe_print!(
            show.num_mod_combo_star_ratings_mania,
            self.num_mod_combo_star_ratings_mania,
            "    "
        );
        if show.mod_combo_star_ratings_mania && self.mod_combo_star_ratings_mania.is_some() {
            println!("    MCSR osu!mania {{");
            for (mods, star_rating) in self.mod_combo_star_ratings_mania.as_ref().unwrap() {
                println!("        mods: {:32b} | star rating: {}*", mods, star_rating);
            }
            println!("    }}");
        }
        maybe_print!(show.drain_time, self.drain_time, "    ");
        maybe_print!(show.total_time, self.total_time, "    ");
        maybe_print!(
            show.preview_offset_from_start_ms,
            self.preview_offset_from_start_ms,
            "    "
        );
        maybe_print!(show.num_timing_points, self.num_timing_points, "    ");
        maybe_print_vec!(show.timing_points, self.timing_points, "timing points");
        maybe_print!(show.beatmap_id, self.beatmap_id, "    ");
        maybe_print!(show.beatmap_set_id, self.beatmap_set_id, "    ");
        maybe_print!(show.thread_id, self.thread_id, "    ");
        maybe_print!(show.standard_grade, self.standard_grade, "    ");
        maybe_print!(show.taiko_grade, self.taiko_grade, "    ");
        maybe_print!(show.ctb_grade, self.ctb_grade, "    ");
        maybe_print!(show.mania_grade, self.mania_grade, "    ");
        maybe_print!(show.local_offset, self.local_offset, "    ");
        maybe_print!(show.stack_leniency, self.stack_leniency, "    ");
        maybe_print!(show.gameplay_mode, self.gameplay_mode, "    ");
        maybe_print!(show.song_source, self.song_source, "    ");
        maybe_print!(show.song_tags, self.song_tags, "    ");
        maybe_print!(show.online_offset, self.online_offset, "    ");
        maybe_print!(
            show.font_used_for_song_title,
            self.font_used_for_song_title,
            "    "
        );
        maybe_print!(show.unplayed, self.unplayed, "    ");
        maybe_print!(show.last_played, self.last_played, "    ");
        maybe_print!(show.is_osz2, self.is_osz2, "    ");
        maybe_print!(show.beatmap_folder_name, self.beatmap_folder_name, "    ");
        maybe_print!(
            show.last_checked_against_repo,
            self.last_checked_against_repo,
            "    "
        );
        maybe_print!(show.ignore_beatmap_sound, self.ignore_beatmap_sound, "    ");
        maybe_print!(show.ignore_beatmap_skin, self.ignore_beatmap_skin, "    ");
        maybe_print!(show.disable_storyboard, self.disable_storyboard, "    ");
        maybe_print!(show.disable_video, self.disable_video, "    ");
        maybe_print!(show.visual_override, self.visual_override, "    ");
        maybe_print!(show.unknown_short, self.unknown_short, "    ");
        maybe_print!(
            show.offset_from_song_start_in_editor_ms,
            self.offset_from_song_start_in_editor_ms
        );
        maybe_print!(show.mania_scroll_speed, self.mania_scroll_speed, "    ");
    }
}
