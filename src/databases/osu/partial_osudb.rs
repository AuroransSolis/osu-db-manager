use std::thread::{self, JoinHandle};
use std::sync::{Arc, Mutex};

use chrono::NaiveDate;

use crate::deserialize_primitives::*;
use crate::maybe_deserialize_primitives::*;
use crate::read_error::{ParseFileResult, DbFileParseError, ParseErrorKind::*};
use crate::databases::{
    load::PartialLoad,
    osu::{
        partial_beatmap::PartialBeatmap,
        primitives::{GameplayMode, TimingPoint, RankedStatus},
        versions::{Legacy, Modern, ModernWithEntrySize, ReadPartialVersionSpecificData}
    }
};
use crate::masks::osu_mask::{OsuDbMask, BeatmapMask};

#[derive(Debug)]
pub struct PartialOsuDb {
    pub version: Option<i32>,
    pub folder_count: Option<i32>,
    pub account_unlocked: Option<bool>,
    pub account_unlock_date: Option<NaiveDate>,
    pub player_name: Option<String>,
    pub number_of_beatmaps: i32,
    pub beatmaps: Option<Vec<PartialBeatmap>>,
    pub unknown_int: Option<i32>
}

impl PartialLoad<OsuDbMask> for PartialOsuDb {
    fn read_single_thread(mask: OsuDbMask, bytes: Vec<u8>) -> ParseFileResult<Self> {
        let mut index = 0;
        let i = &mut index;
        let version = read_int(&bytes, i)?;
        let folder_count = maybe_read_int(mask.folder_count, &bytes, i)?;
        let account_unlocked = maybe_read_boolean(mask.account_unlocked, &bytes, i)?;
        let account_unlock_date = if let Some(true) = account_unlocked {
            *i += 8;
            None
        } else {
            maybe_read_datetime(mask.account_unlock_date, &bytes, i)?
        };
        let player_name = maybe_read_player_name(mask.player_name, &bytes, i)?;
        let num_beatmaps = read_int(&bytes, i)?;
        let (beatmaps, unknown_int) = if let Some(m) = mask.beatmap_mask {
            let mut tmp = Vec::with_capacity(num_beatmaps as usize);
            if version < 20140609 {
                for _ in 0..num_beatmaps {
                    tmp.push(PartialBeatmap::read_from_bytes::<Legacy>(m, &bytes, i)?);
                }
            } else if version >= 20140609 && version < 20160408 {
                for _ in 0..num_beatmaps {
                    tmp.push(PartialBeatmap::read_from_bytes::<Modern>(m, &bytes, i)?);
                }
            } else if version >= 20160408 {
                for _ in 0..num_beatmaps {
                    tmp.push(PartialBeatmap::read_from_bytes::<ModernWithEntrySize>(m, &bytes, i)?);
                }
            } else {
                let err_msg = format!("Read version with no associated beatmap loading method {}",
                    version);
                return Err(DbFileParseError::new(OsuDbError, err_msg.as_str()));
            }
            let unknown_int = maybe_read_int(mask.unknown_int, &bytes, i)?;
            (Some(tmp), unknown_int)
        } else {
            (None, maybe_read_int(mask.unknown_int, &bytes[bytes.len() - 4..bytes.len()], i)?)
        };
        let version = if mask.version {
            Some(version)
        } else {
            None
        };
        Ok(PartialOsuDb {
            version,
            folder_count,
            account_unlocked,
            account_unlock_date,
            player_name,
            number_of_beatmaps: num_beatmaps,
            beatmaps,
            unknown_int
        })
    }

    fn read_multi_thread(mask: OsuDbMask, jobs: usize, bytes: Vec<u8>) -> ParseFileResult<Self> {
        let (version, folder_count, account_unlocked, account_unlock_date, player_name,
            mut bytes_used) = {
            let mut index = 0;
            let i = &mut index;
            let version = read_int(&bytes, i)?;
            let folder_count = maybe_read_int(mask.folder_count, &bytes, i)?;
            let account_unlocked = maybe_read_boolean(mask.account_unlocked, &bytes, i)?;
            let account_unlock_date = if let Some(true) = account_unlocked {
                *i += 8;
                None
            } else {
                maybe_read_datetime(mask.account_unlock_date, &bytes, i)?
            };
            let player_name = maybe_read_player_name(mask.player_name, &bytes, i)?;
            (version, folder_count, account_unlocked, account_unlock_date, player_name, *i)
        };
        let num_beatmaps = read_int(&bytes, &mut bytes_used)?;
        let counter = Arc::new(Mutex::new(0));
        let start = Arc::new(Mutex::new(bytes_used));
        let (beatmaps, unknown_int) = if let Some(m) = mask.beatmap_mask {
            if version >= 20160408 {
                let threads = (0..jobs)
                    .map(|_| spawn_partial_beatmap_loader_thread(m, num_beatmaps as usize,
                        counter.clone(), start.clone(), &bytes)).collect::<Vec<_>>();
                let mut results = threads.into_iter().map(|joinhandle| joinhandle.join().unwrap())
                    .collect::<Vec<_>>();
                let mut beatmaps = results.pop().unwrap()?;
                for beatmap_result in results {
                    beatmaps.append(&mut beatmap_result?);
                }
                beatmaps.sort_by(|(a, _), (b, _)| a.cmp(b));
                let beatmaps = beatmaps.into_iter().map(|(_, beatmap)| beatmap).collect::<Vec<_>>();
                let unknown_int = maybe_read_int(mask.unknown_int, &bytes,
                    &mut start.lock().unwrap())?;
                (Some(beatmaps), unknown_int)
            } else if version / 1000 <= 2016 && version / 1000 >= 2007 { // catch valid versions
                return Err(DbFileParseError::new(OsuDbError, "osu!.db versions older than 20160408 do \
                not support multithreaded loading due to lacking an entry size."));
            } else {
                let err_msg = format!("Read version with no associated beatmap loading method: {}",
                    version);
                return Err(DbFileParseError::new(OsuDbError, err_msg.as_str()));
            }
        } else {
            (None, maybe_read_int(mask.unknown_int, &bytes[bytes.len() - 4..bytes.len()], &mut 0)?)
        };
        let version  = if mask.version {
            Some(version)
        } else {
            None
        };
        Ok(PartialOsuDb {
            version,
            folder_count,
            account_unlocked,
            account_unlock_date,
            player_name,
            number_of_beatmaps: num_beatmaps,
            beatmaps,
            unknown_int
        })
    }
}

#[inline]
fn spawn_partial_beatmap_loader_thread(mask: BeatmapMask, number: usize, counter: Arc<Mutex<usize>>,
    start: Arc<Mutex<usize>>, bytes_pointer: *const Vec<u8>)
    -> JoinHandle<ParseFileResult<Vec<(usize, PartialBeatmap)>>> {
    let tmp = bytes_pointer as usize;
    thread::spawn(move || {
        let bytes = unsafe { &*(tmp as *const Vec<u8>) };
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
            let i = &mut start;
            let artist_name = maybe_read_string_utf8(mask.artist_name, bytes, i,
                "non-Unicode artist name")?;
            let artist_name_unicode = maybe_read_string_utf8(mask.artist_name_unicode, bytes, i,
                "Unicode artist name")?;
            let song_title = maybe_read_string_utf8(mask.song_title, bytes, i,
                "non-Unicode song title")?;
            let song_title_unicode = maybe_read_string_utf8(mask.song_title_unicode, bytes, i,
                "Unicode song title")?;
            let creator_name = maybe_read_string_utf8(mask.creator_name, bytes, i, "creator name")?;
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
            let last_modification_time = maybe_read_datetime(mask.last_modification_time, bytes,
                i)?;
            let approach_rate = ModernWithEntrySize::maybe_read_arcshpod(mask.approach_rate, bytes,
                i)?;
            let circle_size = ModernWithEntrySize::maybe_read_arcshpod(mask.circle_size, bytes, i)?;
            let hp_drain = ModernWithEntrySize::maybe_read_arcshpod(mask.hp_drain, bytes, i)?;
            let overall_difficulty = ModernWithEntrySize::maybe_read_arcshpod(
                mask.overall_difficulty, bytes, i)?;
            let slider_velocity = maybe_read_double(mask.slider_velocity, bytes, i)?;
            let (num_mcsr_standard, mcsr_standard)
                = ModernWithEntrySize::maybe_read_mod_combo_star_ratings(
                mask.mod_combo_star_ratings_standard, bytes, i)?;
            let (num_mcsr_taiko, mcsr_taiko)
                = ModernWithEntrySize::maybe_read_mod_combo_star_ratings(
                mask.mod_combo_star_ratings_taiko, bytes, i)?;
            let (num_mcsr_ctb, mcsr_ctb)
                = ModernWithEntrySize::maybe_read_mod_combo_star_ratings(
                mask.mod_combo_star_ratings_ctb, bytes, i)?;
            let (num_mcsr_mania, mcsr_mania)
                = ModernWithEntrySize::maybe_read_mod_combo_star_ratings(
                mask.mod_combo_star_ratings_mania, bytes, i)?;
            let drain_time = maybe_read_int(mask.drain_time, bytes, i)?;
            let total_time = maybe_read_int(mask.total_time, bytes, i)?;
            let preview_offset_from_start_ms = maybe_read_int(mask.preview_offset_from_start_ms,
                bytes, i)?;
            let num_timing_points = read_int(bytes, i)?;
            let timing_points = if mask.timing_points {
                let mut tmp = Vec::with_capacity(num_timing_points as usize);
                for _ in 0..num_timing_points {
                    tmp.push(TimingPoint::read_from_bytes(bytes, i)?);
                }
                Some(tmp)
            } else {
                *i += num_timing_points as usize * 17;
                None
            };
            let num_timing_points = if mask.num_timing_points {
                Some(num_timing_points)
            } else {
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
            let font_used_for_song_title = maybe_read_string_utf8(mask.font_used_for_song_title,
                bytes, i, "font used for song title")?;
            let unplayed = maybe_read_boolean(mask.unplayed, bytes, i)?;
            let last_played = maybe_read_datetime(mask.last_played, bytes, i)?;
            let is_osz2 = maybe_read_boolean(mask.is_osz2, bytes, i)?;
            let beatmap_folder_name = maybe_read_string_utf8(mask.beatmap_folder_name, bytes, i,
                "folder name")?;
            let last_checked_against_repo = maybe_read_datetime(mask.last_checked_against_repo,
                bytes, i)?;
            let ignore_beatmap_sound = maybe_read_boolean(mask.ignore_beatmap_sound, bytes, i)?;
            let ignore_beatmap_skin = maybe_read_boolean(mask.ignore_beatmap_skin, bytes, i)?;
            let disable_storyboard = maybe_read_boolean(mask.disable_storyboard, bytes, i)?;
            let disable_video = maybe_read_boolean(mask.disable_video, bytes, i)?;
            let visual_override = maybe_read_boolean(mask.visual_override, bytes, i)?;
            let unknown_short = ModernWithEntrySize::maybe_read_unknown_short(mask.unknown_short,
                bytes, i)?;
            let offset_from_song_start_in_editor_ms = maybe_read_int(
                mask.offset_from_song_start_in_editor_ms, bytes, i)?;
            let mania_scroll_speed = maybe_read_byte(mask.mania_scroll_speed, bytes, i)?;
            beatmaps.push((num, PartialBeatmap {
                entry_size: Some(entry_size),
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
            }));
        }
    })
}