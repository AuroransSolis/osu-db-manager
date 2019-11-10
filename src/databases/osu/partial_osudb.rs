use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

use chrono::NaiveDate;

use crate::databases::{
    load::PartialLoad,
    osu::{
        partial_beatmap::PartialBeatmap,
        primitives::{GameplayMode, RankedStatus, TimingPoint},
        versions::{Legacy, Modern, ModernWithEntrySize, ReadPartialVersionSpecificData},
    },
};
use crate::deserialize_primitives::*;
use crate::load_settings::osu::beatmap_load_settings::BeatmapLoadSettings;
use crate::load_settings::osu::osudb_load_settings::OsuDbLoadSettings;
use crate::masks::osu_mask::{BeatmapMask, OsuDbMask};
use crate::maybe_deserialize_primitives::*;
use crate::read_error::{DbFileParseError, ParseErrorKind::*, ParseFileResult};

#[derive(Debug)]
pub struct PartialOsuDb {
    pub version: Option<i32>,
    pub folder_count: Option<i32>,
    pub account_unlocked: Option<bool>,
    pub account_unlock_date: Option<NaiveDate>,
    pub player_name: Option<String>,
    pub number_of_beatmaps: i32,
    pub beatmaps: Option<Vec<PartialBeatmap>>,
    pub unknown_short: Option<i16>,
}

impl PartialLoad<OsuDbMask, OsuDbLoadSettings> for PartialOsuDb {
    fn read_single_thread(settings: OsuDbLoadSettings, bytes: Vec<u8>) -> ParseFileResult<Self> {
        let mut index = 0;
        let i = &mut index;
        let mut skip = false;
        let s = &mut skip;
        let version = read_int(&bytes, i)?;
        let folder_count = maybe_read_int_nocomp(settings.folder_count, s, &bytes, i)?;
        let account_unlocked = maybe_read_boolean_nocomp(settings.account_unlocked, s, &bytes, i)?;
        let account_unlock_date = if let Some(true) = account_unlocked {
            *i += 8;
            None
        } else if *s {
            *i += 8;
            None
        } else {
            maybe_read_datetime_nocomp(settings.account_unlock_date, s, &bytes, i)?
        };
        let player_name = maybe_read_player_name_nocomp(settings.player_name, s, &bytes, i)?;
        let num_beatmaps = read_int(&bytes, i)?;
        let beatmaps = if settings.beatmap_load_settings.ignore_all() {
            None
        } else {
            let mut tmp = Vec::with_capacity(num_beatmaps as usize);
            if version < 20140609 {
                for _ in 0..num_beatmaps {
                    tmp.push(PartialBeatmap::read_from_bytes::<Legacy>(
                        &settings.beatmap_load_settings,
                        &bytes,
                        i,
                    )?);
                }
            } else if version >= 20140609 && version < 20160408 {
                for _ in 0..num_beatmaps {
                    tmp.push(PartialBeatmap::read_from_bytes::<Modern>(
                        &settings.beatmap_load_settings,
                        &bytes,
                        i,
                    )?);
                }
            } else if version >= 20160408 {
                for _ in 0..num_beatmaps {
                    tmp.push(PartialBeatmap::read_from_bytes::<ModernWithEntrySize>(
                        &settings.beatmap_load_settings,
                        &bytes,
                        i,
                    )?);
                }
            } else {
                let err_msg = format!(
                    "Read version with no associated beatmap loading method {}",
                    version
                );
                return Err(DbFileParseError::new(OsuDbError, err_msg.as_str()));
            }
        };
        let unknown_short = if version < 20140609 {
            Legacy::maybe_read_unknown_short(*s, &bytes, i)?
        } else if version >= 20140609 && version < 20160408 {
            Modern::maybe_read_unknown_short(*s, &bytes, i)?
        } else {
            ModernWithEntrySize::maybe_read_unknown_short(*s, &bytes, i)?
        };
        let version = if settings.version.is_ignore() {
            None
        } else {
            Some(version)
        };
        Ok(PartialOsuDb {
            version,
            folder_count,
            account_unlocked,
            account_unlock_date,
            player_name,
            number_of_beatmaps: num_beatmaps,
            beatmaps,
            unknown_short,
        })
    }

    fn read_multi_thread(
        settings: OsuDbLoadSettings,
        jobs: usize,
        bytes: Vec<u8>,
    ) -> ParseFileResult<Self> {
        let mut skip = false;
        let s = &mut skip;
        let (
            version,
            folder_count,
            account_unlocked,
            account_unlock_date,
            player_name,
            mut bytes_used,
        ) = {
            let mut index = 0;
            let i = &mut index;
            let version = read_int(&bytes, i)?;
            let folder_count = maybe_read_int_nocomp(settings.folder_count, s, &bytes, i)?;
            let account_unlocked =
                maybe_read_boolean_nocomp(settings.account_unlocked, s, &bytes, i)?;
            let account_unlock_date = if let Some(true) = account_unlocked {
                *i += 8;
                None
            } else if *s {
                *i += 8;
                None
            } else {
                maybe_read_datetime_nocomp(settings.account_unlock_date, s, &bytes, i)?
            };
            let player_name = maybe_read_player_name_nocomp(settings.player_name, s, &bytes, i)?;
            (
                version,
                folder_count,
                account_unlocked,
                account_unlock_date,
                player_name,
                *i,
            )
        };
        let num_beatmaps = read_int(&bytes, &mut bytes_used)?;
        let beatmaps = if settings.beatmap_load_settings.ignore_all() || num_beatmaps == 0 {
            None
        } else {
            let counter = Arc::new(Mutex::new(0));
            let start = Arc::new(Mutex::new(bytes_used));
            if version >= 20160408 {
                let threads = (0..jobs)
                    .map(|_| {
                        spawn_partial_beatmap_loader_thread(
                            m,
                            num_beatmaps as usize,
                            counter.clone(),
                            start.clone(),
                            &bytes,
                        )
                    })
                    .collect::<Vec<_>>();
                let mut results = threads
                    .into_iter()
                    .map(|joinhandle| joinhandle.join().unwrap())
                    .collect::<Vec<_>>();
                let mut beatmaps = results.pop().unwrap();
                for beatmap_result in results {
                    beatmaps.append(&mut beatmap_result?);
                }
                beatmaps.sort_by(|(a, _), (b, _)| a.cmp(b));
                let beatmaps = beatmaps
                    .into_iter()
                    .map(|(_, beatmap)| beatmap)
                    .collect::<Vec<_>>();
                Some(beatmaps)
            } else if version < 20160408 && version >= 20140609 {
                // catch valid versions
                return Err(DbFileParseError::new(
                    OsuDbError,
                    "osu!.db versions older than 20160408 do \
                     not support multithreaded loading due to lacking a specified entry size.",
                ));
            } else {
                let err_msg = format!(
                    "Read version with no associated beatmap loading method: {}",
                    version
                );
                return Err(DbFileParseError::new(OsuDbError, err_msg.as_str()));
            }
        };
        let unknown_short = if version < 20140609 {
            Legacy::maybe_read_unknown_short(*s, &bytes, i)?
        } else if version >= 20140609 && version < 20160408 {
            Modern::maybe_read_unknown_short(*s, &bytes, i)?
        } else {
            ModernWithEntrySize::maybe_read_unknown_short(*s, &bytes, i)?
        };
        let version = if mask.version { Some(version) } else { None };
        Ok(PartialOsuDb {
            version,
            folder_count,
            account_unlocked,
            account_unlock_date,
            player_name,
            number_of_beatmaps: num_beatmaps,
            beatmaps,
            unknown_short,
        })
    }
}

#[inline]
fn spawn_partial_beatmap_loader_thread(
    settings: *const BeatmapLoadSettings,
    number: usize,
    counter: Arc<Mutex<usize>>,
    start: Arc<Mutex<usize>>,
    bytes_pointer: *const Vec<u8>,
) -> JoinHandle<ParseFileResult<Vec<(usize, PartialBeatmap)>>> {
    let tmp_bp = bytes_pointer as usize;
    let tmp_s = settings as usize;
    thread::spawn(move || {
        let (settings, bytes) = unsafe {
            (
                &*(tmp_s as *const BeatmapLoadSettings),
                &*(tmp_bp as *const Vec<u8>),
            )
        };
        let mut beatmaps = Vec::new();
        loop {
            let (entry_size, mut start, num) = {
                let mut ctr = counter.lock().unwrap();
                if *ctr >= number {
                    return Ok(beatmaps);
                } else {
                    *ctr += 1;
                }
                let mut s = start.lock().unwrap();
                let start_at = *s + 4;
                let entry_size = read_int(bytes, &mut *s)?;
                *s += entry_size as usize;
                (entry_size, start_at, *ctr - 1)
            };
            let entry_size = settings.entry_size.apply(entry_size);
            if entry_size.is_none() {
                continue;
            }
            let i = &mut start;
            let mut skip = false;
            let s = &mut skip;
            let artist_name = maybe_read_string_utf8(
                settings.artist_name,
                s,
                bytes,
                i,
                "non-Unicode artist name",
            )?;
            if skip {
                continue;
            };
            let artist_name_unicode = maybe_read_string_utf8(
                settings.artist_name_unicode,
                s,
                bytes,
                i,
                "Unicode artist name",
            )?;
            if skip {
                continue;
            };
            let song_title =
                maybe_read_string_utf8(settings.song_title, s, bytes, i, "non-Unicode song title")?;
            if skip {
                continue;
            };
            let song_title_unicode = maybe_read_string_utf8(
                settings.song_title_unicode,
                s,
                bytes,
                i,
                "Unicode song title",
            )?;
            if skip {
                continue;
            };
            let creator_name =
                maybe_read_string_utf8(settings.creator_name, s, bytes, i, "creator name")?;
            if skip {
                continue;
            };
            let difficulty =
                maybe_read_string_utf8(settings.difficulty, s, bytes, i, "difficulty")?;
            if skip {
                continue;
            };
            let audio_file_name =
                maybe_read_string_utf8(settings.audio_file_name, s, bytes, i, "audio file name")?;
            if skip {
                continue;
            };
            let md5_beatmap_hash = maybe_read_md5_hash(settings.md5_beatmap_hash, s, bytes, i)?;
            if skip {
                continue;
            };
            let dotosu_file_name = maybe_read_string_utf8(
                settings.dotosu_file_name,
                s,
                bytes,
                i,
                "corresponding .osu file name",
            )?;
            if skip {
                continue;
            };
            let ranked_status =
                RankedStatus::maybe_read_from_bytes(settings.ranked_status, s, bytes, i)?;
            if skip {
                continue;
            };
            let number_of_hitcircles =
                maybe_read_short(settings.number_of_hitcircles, s, bytes, i)?;
            if skip {
                continue;
            };
            let number_of_sliders = maybe_read_short(settings.number_of_sliders, s, bytes, i)?;
            if skip {
                continue;
            };
            let number_of_spinners = maybe_read_short(settings.number_of_spinners, s, bytes, i)?;
            if skip {
                continue;
            };
            let last_modification_time =
                maybe_read_datetime(settings.last_modification_time, s, bytes, i)?;
            if skip {
                continue;
            };
            let approach_rate =
                ModernWithEntrySize::maybe_read_arcshpod(settings.approach_rate, s, bytes, i)?;
            if skip {
                continue;
            };
            let circle_size =
                ModernWithEntrySize::maybe_read_arcshpod(settings.circle_size, s, bytes, i)?;
            if skip {
                continue;
            };
            let hp_drain =
                ModernWithEntrySize::maybe_read_arcshpod(settings.hp_drain, s, bytes, i)?;
            if skip {
                continue;
            };
            let overall_difficulty =
                ModernWithEntrySize::maybe_read_arcshpod(settings.overall_difficulty, s, bytes, i)?;
            if skip {
                continue;
            };
            let slider_velocity = maybe_read_double(settings.slider_velocity, s, bytes, i)?;
            if skip {
                continue;
            };
            let (num_mcsr_standard, mcsr_standard) =
                ModernWithEntrySize::maybe_read_mod_combo_star_ratings(
                    settings.mod_combo_star_ratings_standard.is_ignore(),
                    bytes,
                    i,
                )?;
            let (num_mcsr_taiko, mcsr_taiko) =
                ModernWithEntrySize::maybe_read_mod_combo_star_ratings(
                    settings.mod_combo_star_ratings_standard.is_ignore(),
                    bytes,
                    i,
                )?;
            let (num_mcsr_ctb, mcsr_ctb) = ModernWithEntrySize::maybe_read_mod_combo_star_ratings(
                settings.mod_combo_star_ratings_standard.is_ignore(),
                bytes,
                i,
            )?;
            let (num_mcsr_mania, mcsr_mania) =
                ModernWithEntrySize::maybe_read_mod_combo_star_ratings(
                    settings.mod_combo_star_ratings_standard.is_ignore(),
                    bytes,
                    i,
                )?;
            let drain_time = maybe_read_int(settings.drain_time, s, bytes, i)?;
            if skip {
                continue;
            };
            let total_time = maybe_read_int(settings.total_time, s, bytes, i)?;
            if skip {
                continue;
            };
            let preview_offset_from_start_ms =
                maybe_read_int(settings.preview_offset_from_start_ms, s, bytes, i)?;
            if skip {
                continue;
            };
            let num_timing_points = read_int(bytes, i)?;
            let timing_points = if settings.timing_points.is_ignore() {
                *i += num_timing_points as usize * 17;
                None
            } else {
                let mut tmp = Vec::with_capacity(num_timing_points as usize);
                for _ in 0..num_timing_points {
                    tmp.push(TimingPoint::read_from_bytes(bytes, i)?);
                }
                Some(tmp)
            };
            let num_timing_points = settings.num_timing_points.apply(num_timing_points);
            let beatmap_id = maybe_read_int(settings.beatmap_id, s, bytes, i)?;
            if skip {
                continue;
            };
            let beatmap_set_id = maybe_read_int(settings.beatmap_set_id, s, bytes, i)?;
            if skip {
                continue;
            };
            let thread_id = maybe_read_int(settings.thread_id, s, bytes, i)?;
            if skip {
                continue;
            };
            let standard_grade = maybe_read_byte(settings.standard_grade, s, bytes, i)?;
            if skip {
                continue;
            };
            let taiko_grade = maybe_read_byte(settings.taiko_grade, s, bytes, i)?;
            if skip {
                continue;
            };
            let ctb_grade = maybe_read_byte(settings.ctb_grade, s, bytes, i)?;
            if skip {
                continue;
            };
            let mania_grade = maybe_read_byte(settings.mania_grade, s, bytes, i)?;
            if skip {
                continue;
            };
            let local_offset = maybe_read_short(settings.local_offset, s, bytes, i)?;
            if skip {
                continue;
            };
            let stack_leniency = maybe_read_single(settings.stack_leniency, s, bytes, i)?;
            if skip {
                continue;
            };
            let gameplay_mode =
                GameplayMode::maybe_read_from_bytes(settings.gameplay_mode, s, bytes, i)?;
            if skip {
                continue;
            };
            let song_source =
                maybe_read_string_utf8(settings.song_source, s, bytes, i, "song source")?;
            if skip {
                continue;
            };
            let song_tags = maybe_read_string_utf8(settings.song_tags, s, bytes, i, "song tags")?;
            if skip {
                continue;
            };
            let online_offset = maybe_read_short(settings.online_offset, s, bytes, i)?;
            if skip {
                continue;
            };
            let font_used_for_song_title = maybe_read_string_utf8(
                settings.font_used_for_song_title,
                s,
                bytes,
                i,
                "font used for song title",
            )?;
            if skip {
                continue;
            };
            let unplayed = maybe_read_boolean(settings.unplayed, s, bytes, i)?;
            if skip {
                continue;
            };
            let last_played = maybe_read_datetime(settings.last_played, s, bytes, i)?;
            if skip {
                continue;
            };
            let is_osz2 = maybe_read_boolean(settings.is_osz2, s, bytes, i)?;
            if skip {
                continue;
            };
            let beatmap_folder_name =
                maybe_read_string_utf8(settings.beatmap_folder_name, s, bytes, i, "folder name")?;
            if skip {
                continue;
            };
            let last_checked_against_repo =
                maybe_read_datetime(settings.last_checked_against_repo, s, bytes, i)?;
            if skip {
                continue;
            };
            let ignore_beatmap_sound =
                maybe_read_boolean(settings.ignore_beatmap_sound, s, bytes, i)?;
            if skip {
                continue;
            };
            let ignore_beatmap_skin =
                maybe_read_boolean(settings.ignore_beatmap_skin, s, bytes, i)?;
            if skip {
                continue;
            };
            let disable_storyboard = maybe_read_boolean(settings.disable_storyboard, s, bytes, i)?;
            if skip {
                continue;
            };
            let disable_video = maybe_read_boolean(settings.disable_video, s, bytes, i)?;
            if skip {
                continue;
            };
            let visual_override = maybe_read_boolean(settings.visual_override, s, bytes, i)?;
            if skip {
                continue;
            };
            let unknown_short = ModernWithEntrySize::maybe_read_unknown_short(*s, bytes, i)?;
            if skip {
                continue;
            };
            let offset_from_song_start_in_editor_ms =
                maybe_read_int(settings.offset_from_song_start_in_editor_ms, s, bytes, i)?;
            if skip {
                continue;
            };
            let mania_scroll_speed = maybe_read_byte(settings.mania_scroll_speed, s, bytes, i)?;
            if skip {
                continue;
            };
            beatmaps.push((
                num,
                PartialBeatmap {
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
                },
            ));
        }
    })
}
