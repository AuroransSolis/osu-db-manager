use crate::databases::osu::{
    partial_beatmap::PartialBeatmap,
    primitives::*,
    versions::{
        Legacy, Modern, ModernWithEntrySize, ModernWithPermissions, ReadPartialVersionSpecificData,
    },
};
use crate::deserialize_primitives::*;
use crate::load_settings::osu::beatmap_load_settings::BeatmapLoadSettings;
use crate::load_settings::osu::osudb_load_settings::OsuDbLoadSettings;
use crate::maybe_deserialize_primitives::*;
use crate::read_error::{DbFileParseError, ParseErrorKind, ParseFileResult};
use crate::{masks::osu_mask::OsuDbMask, maybe_print};
use chrono::NaiveDate;
use crossbeam_utils::thread::{self, Scope, ScopedJoinHandle};
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct PartialOsuDb<'a> {
    pub version: Option<i32>,
    pub folder_count: Option<i32>,
    pub account_unlocked: Option<bool>,
    pub account_unlock_date: Option<NaiveDate>,
    pub player_name: Option<&'a str>,
    pub number_of_beatmaps: Option<i32>,
    pub beatmaps: Option<Vec<PartialBeatmap<'a>>>,
    pub unknown_short_or_permissions: Option<UnknownShortOrUserPermissions>,
}

impl<'a> PartialOsuDb<'a> {
    pub fn read_from_bytes(
        settings: OsuDbLoadSettings,
        jobs: usize,
        bytes: &'a [u8],
    ) -> ParseFileResult<Self> {
        if jobs == 1 {
            Self::read_single_thread(settings, bytes)
        } else {
            Self::read_multi_thread(settings, jobs, bytes)
        }
    }

    pub fn read_single_thread(
        settings: OsuDbLoadSettings,
        bytes: &'a [u8],
    ) -> ParseFileResult<Self> {
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
            // The following version numbers were graciously provided by OMKelderman#8113, excepting
            // the most recent which was provided by tdeo#6188 (20191107 as of time of writing
            // this). See versions.rs in this directory for more information on osu!.db versions.
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
            } else if version >= 20160408 && version < 20191107 {
                for _ in 0..num_beatmaps {
                    tmp.push(PartialBeatmap::read_from_bytes::<ModernWithEntrySize>(
                        &settings.beatmap_load_settings,
                        &bytes,
                        i,
                    )?);
                }
            } else if version >= 20191107 {
                for _ in 0..num_beatmaps {
                    tmp.push(PartialBeatmap::read_from_bytes::<ModernWithPermissions>(
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
                return Err(DbFileParseError::new(
                    ParseErrorKind::OsuDbError,
                    err_msg.as_str(),
                ));
            }
            Some(tmp)
        };
        let unknown_short_or_permissions = if version < 20140609 {
            Legacy::maybe_read_unknown_short_or_user_permissions(
                settings.unknown_short_or_permissions,
                s,
                &bytes,
                i,
            )?
        } else if version >= 20140609 && version < 20160408 {
            Modern::maybe_read_unknown_short_or_user_permissions(
                settings.unknown_short_or_permissions,
                s,
                &bytes,
                i,
            )?
        } else if version >= 20160408 && version < 20191107 {
            ModernWithEntrySize::maybe_read_unknown_short_or_user_permissions(
                settings.unknown_short_or_permissions,
                s,
                &bytes,
                i,
            )?
        } else {
            ModernWithPermissions::maybe_read_unknown_short_or_user_permissions(
                settings.unknown_short_or_permissions,
                s,
                &bytes,
                i,
            )?
        };
        let version = if settings.version {
            Some(version)
        } else {
            None
        };
        let number_of_beatmaps = if settings.number_of_beatmaps {
            Some(num_beatmaps)
        } else {
            None
        };
        Ok(PartialOsuDb {
            version,
            folder_count,
            account_unlocked,
            account_unlock_date,
            player_name,
            number_of_beatmaps,
            beatmaps,
            unknown_short_or_permissions,
        })
    }

    pub fn read_multi_thread(
        settings: OsuDbLoadSettings,
        jobs: usize,
        bytes: &'a [u8],
    ) -> ParseFileResult<Self> {
        let mut skip = false;
        let s = &mut skip;
        let mut index = 0;
        let i = &mut index;
        let version = read_int(&bytes, i)?;
        let folder_count = maybe_read_int_nocomp(settings.folder_count, s, &bytes, i)?;
        let account_unlocked = maybe_read_boolean_nocomp(settings.folder_count, s, &bytes, i)?;
        let account_unlock_date = if let Some(false) = account_unlocked {
            maybe_read_datetime_nocomp(settings.account_unlock_date, s, &bytes, i)?
        } else {
            *i += 8;
            None
        };
        let player_name = maybe_read_player_name_nocomp(settings.player_name, s, &bytes, i)?;
        let num_beatmaps = read_int(&bytes, i)?;
        let beatmaps = if settings.beatmap_load_settings.ignore_all() || num_beatmaps == 0 {
            Ok(None)
        } else {
            let counter = Arc::new(Mutex::new(0));
            let start = Arc::new(Mutex::new(*i));
            if version >= 20160408 && version < 20191107 {
                let mut results = thread::scope(|s| {
                    // Spawn a thread for each requested job, collect handles into a vec.
                    let threads = (0..jobs)
                        .map(|_| {
                            spawn_partial_beatmap_loader_thread(
                                s,
                                num_beatmaps as usize,
                                counter.clone(),
                                start.clone(),
                                bytes,
                                &settings.beatmap_load_settings,
                            )
                        })
                        .collect::<Vec<_>>();
                    threads
                        .into_iter()
                        .map(|joinhandle| {
                            joinhandle.join().map_err(|_| {
                                DbFileParseError::new(
                                    ParseErrorKind::OsuDbError,
                                    "Failed to join osu!.db partial beatmap parsing thread.",
                                )
                            })?
                        })
                        .collect::<Vec<_>>()
                })
                .map_err(|_| {
                    DbFileParseError::new(
                        ParseErrorKind::OsuDbError,
                        "Failed to retrieve result from osu!.db partial beatmap parsing scope.",
                    )
                })?;
                let mut beatmaps = results.pop().unwrap()?;
                for beatmap_result in results {
                    // I'm using a `for` loop here with the `pop` above instead of `into_iter()`
                    // and `for_each()` or `fold()` because of the `?`. `?` only returns from
                    // the most immediate function closure, and I want the `?` to return out of
                    // this method call.
                    beatmaps.append(&mut beatmap_result?);
                }
                // Sort by their number so that the parsed data is in the same order as it
                // appears in the database file.
                beatmaps.sort_by(|(a, _), (b, _)| a.cmp(b));
                // Keep only the beatmaps - drop the counting number.
                Ok(Some(
                    beatmaps
                        .into_iter()
                        .map(|(_, beatmap)| beatmap)
                        .collect::<Vec<_>>(),
                ))
            } else if (version < 20160408 && version >= 20140609) || version >= 20191107 {
                // Catch valid versions.
                Err(DbFileParseError::new(
                    ParseErrorKind::OsuDbError,
                    "osu!.db versions older than 20160408 or newer than 20191107 do \
                     not support multithreaded loading due to lacking a specified entry size.",
                ))
            } else {
                let err_msg = format!(
                    "Read version with no associated beatmap loading method: {}",
                    version
                );
                Err(DbFileParseError::new(
                    ParseErrorKind::OsuDbError,
                    err_msg.as_str(),
                ))
            }
        }?;
        let unknown_short_or_permissions = if version < 20140609 {
            Legacy::maybe_read_unknown_short_or_user_permissions(
                settings.unknown_short_or_permissions,
                s,
                &bytes,
                i,
            )?
        } else if version >= 20140609 && version < 20160408 {
            Modern::maybe_read_unknown_short_or_user_permissions(
                settings.unknown_short_or_permissions,
                s,
                &bytes,
                i,
            )?
        } else if version >= 20160408 && version < 20191107 {
            ModernWithEntrySize::maybe_read_unknown_short_or_user_permissions(
                settings.unknown_short_or_permissions,
                s,
                &bytes,
                i,
            )?
        } else {
            ModernWithPermissions::maybe_read_unknown_short_or_user_permissions(
                settings.unknown_short_or_permissions,
                s,
                &bytes,
                i,
            )?
        };
        let version = if settings.version {
            Some(version)
        } else {
            None
        };
        let number_of_beatmaps = if settings.number_of_beatmaps {
            Some(num_beatmaps)
        } else {
            None
        };
        Ok(PartialOsuDb {
            version,
            folder_count,
            account_unlocked,
            account_unlock_date,
            player_name,
            number_of_beatmaps,
            beatmaps,
            unknown_short_or_permissions,
        })
    }

    pub fn display(&self, show: OsuDbMask) {
        if !show.ignore_all() {
            maybe_print!(show.version, self.version);
            maybe_print!(show.folder_count, self.folder_count);
            maybe_print!(show.account_unlocked, self.account_unlocked);
            maybe_print!(show.account_unlock_date, self.account_unlock_date);
            maybe_print!(show.player_name, self.player_name);
            maybe_print!(show.number_of_beatmaps, self.number_of_beatmaps);
            if !show.beatmap_mask.ignore_all() && self.beatmaps.is_some() {
                for beatmap in self.beatmaps.as_ref().unwrap() {
                    beatmap.display(show.beatmap_mask);
                }
            }
            maybe_print!(
                show.unknown_short_or_permissions,
                self.unknown_short_or_permissions
            );
        }
    }
}

fn spawn_partial_beatmap_loader_thread<'scope, 'b: 'scope, 'a: 'b>(
    scope: &'scope Scope<'b>,
    number: usize,
    counter: Arc<Mutex<usize>>,
    start_read: Arc<Mutex<usize>>,
    bytes: &'a [u8],
    settings: &'b BeatmapLoadSettings,
) -> ScopedJoinHandle<'scope, ParseFileResult<Vec<(usize, PartialBeatmap<'a>)>>> {
    scope.spawn(move |_| {
        let mut beatmaps = Vec::new();
        loop {
            let (entry_size, mut start, num) = {
                // Lock the counter.
                let mut ctr = counter.lock().unwrap();
                if *ctr >= number {
                    // Return the collected beatmaps if the requisite number
                    // have been parsed.
                    return Ok(beatmaps);
                } else {
                    // Otherwise, increment the counter and carry on.
                    *ctr += 1;
                }
                // Lock the start index.
                let mut s = start_read.lock().unwrap();
                // Keep track of where the rest of the beatmap entry
                // actually begins.
                let start_at = *s + 4;
                let entry_size = read_int(bytes, &mut *s)?;
                // Increase the start index by the size of this entry.
                *s += entry_size as usize;
                (entry_size, start_at, *ctr - 1)
                // Drop the lock on the counter and start index so other
                // threads can get started on parsing.
            };
            // Use of the entry size means that we can safely skip to the
            // next beatmap if this one doesn't meet the query criteria,
            // since we know where to skip to. The `continue_if!()`s below
            // do just that - they expand to an `if` statement where the
            // conditional block is just `continue;` and the condition is
            // the expression passed to the macro.
            let entry_size = if settings.entry_size.is_ignore() {
                None
            } else {
                Some(entry_size)
            };
            if entry_size.is_none() {
                continue;
            }
            let i = &mut start;
            let mut skip = false;
            let s = &mut skip;
            let artist_name = maybe_read_str_utf8(
                &settings.artist_name,
                s,
                bytes,
                i,
                "non-Unicode artist name",
            )?;
            continue_if!(*s);
            let artist_name_unicode = maybe_read_str_utf8(
                &settings.artist_name_unicode,
                s,
                bytes,
                i,
                "Unicode artist name",
            )?;
            continue_if!(*s);
            let song_title =
                maybe_read_str_utf8(&settings.song_title, s, bytes, i, "non-Unicode song title")?;
            continue_if!(*s);
            let song_title_unicode = maybe_read_str_utf8(
                &settings.song_title_unicode,
                s,
                bytes,
                i,
                "Unicode song title",
            )?;
            continue_if!(*s);
            let creator_name =
                maybe_read_str_utf8(&settings.creator_name, s, bytes, i, "creator name")?;
            continue_if!(*s);
            let difficulty = maybe_read_str_utf8(&settings.difficulty, s, bytes, i, "difficulty")?;
            continue_if!(*s);
            let audio_file_name =
                maybe_read_str_utf8(&settings.audio_file_name, s, bytes, i, "audio file name")?;
            continue_if!(*s);
            let md5_beatmap_hash = maybe_read_md5_hash(&settings.md5_beatmap_hash, s, bytes, i)?;
            continue_if!(*s);
            let dotosu_file_name = maybe_read_str_utf8(
                &settings.dotosu_file_name,
                s,
                bytes,
                i,
                "corresponding .osu file name",
            )?;
            continue_if!(*s);
            let ranked_status =
                RankedStatus::maybe_read_from_bytes(settings.ranked_status, s, bytes, i)?;
            continue_if!(*s);
            let number_of_hitcircles =
                maybe_read_short(settings.number_of_hitcircles, s, bytes, i)?;
            continue_if!(*s);
            let number_of_sliders = maybe_read_short(settings.number_of_sliders, s, bytes, i)?;
            continue_if!(*s);
            let number_of_spinners = maybe_read_short(settings.number_of_spinners, s, bytes, i)?;
            continue_if!(*s);
            let last_modification_time =
                maybe_read_datetime(settings.last_modification_time, s, bytes, i)?;
            continue_if!(*s);
            let approach_rate =
                ModernWithEntrySize::maybe_read_arcshpod(settings.approach_rate, s, bytes, i)?;
            continue_if!(*s);
            let circle_size =
                ModernWithEntrySize::maybe_read_arcshpod(settings.circle_size, s, bytes, i)?;
            continue_if!(*s);
            let hp_drain =
                ModernWithEntrySize::maybe_read_arcshpod(settings.hp_drain, s, bytes, i)?;
            continue_if!(*s);
            let overall_difficulty =
                ModernWithEntrySize::maybe_read_arcshpod(settings.overall_difficulty, s, bytes, i)?;
            continue_if!(*s);
            let slider_velocity = maybe_read_double(settings.slider_velocity, s, bytes, i)?;
            continue_if!(*s);
            let (num_mcsr_standard, mcsr_standard) =
                ModernWithEntrySize::maybe_read_mod_combo_star_ratings(
                    settings.num_mod_combo_star_ratings_standard,
                    settings.mod_combo_star_ratings_standard,
                    s,
                    bytes,
                    i,
                )?;
            let (num_mcsr_taiko, mcsr_taiko) =
                ModernWithEntrySize::maybe_read_mod_combo_star_ratings(
                    settings.num_mod_combo_star_ratings_taiko,
                    settings.mod_combo_star_ratings_taiko,
                    s,
                    bytes,
                    i,
                )?;
            let (num_mcsr_ctb, mcsr_ctb) = ModernWithEntrySize::maybe_read_mod_combo_star_ratings(
                settings.num_mod_combo_star_ratings_ctb,
                settings.mod_combo_star_ratings_ctb,
                s,
                bytes,
                i,
            )?;
            let (num_mcsr_mania, mcsr_mania) =
                ModernWithEntrySize::maybe_read_mod_combo_star_ratings(
                    settings.num_mod_combo_star_ratings_mania,
                    settings.mod_combo_star_ratings_mania,
                    s,
                    bytes,
                    i,
                )?;
            let drain_time = maybe_read_int(settings.drain_time, s, bytes, i)?;
            continue_if!(*s);
            let total_time = maybe_read_int(settings.total_time, s, bytes, i)?;
            continue_if!(*s);
            let preview_offset_from_start_ms =
                maybe_read_int(settings.preview_offset_from_start_ms, s, bytes, i)?;
            continue_if!(*s);
            let num_timing_points = read_int(bytes, i)?;
            let timing_points = if settings.timing_points {
                let mut tmp = Vec::with_capacity(num_timing_points as usize);
                for _ in 0..num_timing_points {
                    tmp.push(TimingPoint::read_from_bytes(bytes, i)?);
                }
                Some(tmp)
            } else {
                *i += num_timing_points as usize * 17;
                None
            };
            let num_timing_points = if settings.num_timing_points.is_ignore() {
                None
            } else {
                Some(num_timing_points)
            };
            let beatmap_id = maybe_read_int(settings.beatmap_id, s, bytes, i)?;
            continue_if!(*s);
            let beatmap_set_id = maybe_read_int(settings.beatmap_set_id, s, bytes, i)?;
            continue_if!(*s);
            let thread_id = maybe_read_int(settings.thread_id, s, bytes, i)?;
            continue_if!(*s);
            let standard_grade = maybe_read_byte(settings.standard_grade, s, bytes, i)?;
            continue_if!(*s);
            let taiko_grade = maybe_read_byte(settings.taiko_grade, s, bytes, i)?;
            continue_if!(*s);
            let ctb_grade = maybe_read_byte(settings.ctb_grade, s, bytes, i)?;
            continue_if!(*s);
            let mania_grade = maybe_read_byte(settings.mania_grade, s, bytes, i)?;
            continue_if!(*s);
            let local_offset = maybe_read_short(settings.local_offset, s, bytes, i)?;
            continue_if!(*s);
            let stack_leniency = maybe_read_single(settings.stack_leniency, s, bytes, i)?;
            continue_if!(*s);
            let gameplay_mode =
                GameplayMode::maybe_read_from_bytes(settings.gameplay_mode, s, bytes, i)?;
            continue_if!(*s);
            let song_source =
                maybe_read_str_utf8(&settings.song_source, s, bytes, i, "song source")?;
            continue_if!(*s);
            let song_tags = maybe_read_str_utf8(&settings.song_tags, s, bytes, i, "song tags")?;
            continue_if!(*s);
            let online_offset = maybe_read_short(settings.online_offset, s, bytes, i)?;
            continue_if!(*s);
            let font_used_for_song_title = maybe_read_str_utf8(
                &settings.font_used_for_song_title,
                s,
                bytes,
                i,
                "font used for song title",
            )?;
            continue_if!(*s);
            let unplayed = maybe_read_boolean(settings.unplayed, s, bytes, i)?;
            continue_if!(*s);
            let last_played = maybe_read_datetime(settings.last_played, s, bytes, i)?;
            continue_if!(*s);
            let is_osz2 = maybe_read_boolean(settings.is_osz2, s, bytes, i)?;
            continue_if!(*s);
            let beatmap_folder_name =
                maybe_read_str_utf8(&settings.beatmap_folder_name, s, bytes, i, "folder name")?;
            continue_if!(*s);
            let last_checked_against_repo =
                maybe_read_datetime(settings.last_checked_against_repo, s, bytes, i)?;
            continue_if!(*s);
            let ignore_beatmap_sound =
                maybe_read_boolean(settings.ignore_beatmap_sound, s, bytes, i)?;
            continue_if!(*s);
            let ignore_beatmap_skin =
                maybe_read_boolean(settings.ignore_beatmap_skin, s, bytes, i)?;
            continue_if!(*s);
            let disable_storyboard = maybe_read_boolean(settings.disable_storyboard, s, bytes, i)?;
            continue_if!(*s);
            let disable_video = maybe_read_boolean(settings.disable_video, s, bytes, i)?;
            continue_if!(*s);
            let visual_override = maybe_read_boolean(settings.visual_override, s, bytes, i)?;
            continue_if!(*s);
            let unknown_short =
                ModernWithEntrySize::maybe_read_unknown_short(settings.unknown_short, s, bytes, i)?;
            continue_if!(*s);
            let offset_from_song_start_in_editor_ms =
                maybe_read_int(settings.offset_from_song_start_in_editor_ms, s, bytes, i)?;
            continue_if!(*s);
            let mania_scroll_speed = maybe_read_byte(settings.mania_scroll_speed, s, bytes, i)?;
            continue_if!(*s);
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
