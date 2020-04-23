use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

use chrono::NaiveDate;

use crate::databases::{
    load::Load,
    osu::{
        beatmap::Beatmap,
        primitives::*,
        versions::{Legacy, Modern, ModernWithEntrySize, ReadVersionSpecificData},
    },
};
use crate::deserialize_primitives::*;
use crate::read_error::{DbFileParseError, ParseErrorKind::*, ParseFileResult};

/// osu!.db struct according to documentation linked in README.
#[derive(Debug, Clone)]
pub struct OsuDb {
    pub version: i32,
    pub folder_count: i32,
    pub account_unlocked: bool,
    pub account_unlock_date: Option<NaiveDate>,
    pub player_name: Option<String>,
    pub number_of_beatmaps: i32,
    pub beatmaps: Vec<Beatmap>,
    pub unknown_short: i16,
}

impl Load for OsuDb {
    fn read_single_thread(bytes: Vec<u8>) -> ParseFileResult<Self> {
        let mut index = 0;
        let version = read_int(&bytes, &mut index)?;
        let folder_count = read_int(&bytes, &mut index)?;
        let account_unlocked = read_boolean(&bytes, &mut index)?;
        let account_unlock_date = if !account_unlocked {
            Some(read_datetime(&bytes, &mut index)?)
        } else {
            let _ = read_datetime(&bytes, &mut index)?;
            None
        };
        let player_name = read_string_utf8(&bytes, &mut index, "player name")?;
        let num_beatmaps = read_int(&bytes, &mut index)?;
        let mut beatmaps = Vec::with_capacity(num_beatmaps as usize);
        // The following version numbers were graciously provided by OMKelderman#8113, excepting the
        // most recent which was provided by tdeo#6188 (20191107 as of time of writing this). See
        // versions.rs in this directory for more information on osu!.db versions.
        if version < 20140609 {
            for _ in 0..num_beatmaps {
                beatmaps.push(Beatmap::read_from_bytes::<Legacy>(&bytes, &mut index)?);
            }
        } else if version >= 20140609 && version < 20160408 || version >= 20191107 {
            for _ in 0..num_beatmaps {
                beatmaps.push(Beatmap::read_from_bytes::<Modern>(&bytes, &mut index)?);
            }
        } else if version >= 20160408 && version < 20191107 {
            for _ in 0..num_beatmaps {
                beatmaps.push(Beatmap::read_from_bytes::<ModernWithEntrySize>(
                    &bytes, &mut index,
                )?);
            }
        } else {
            let err_msg = format!(
                "Read version with no associated beatmap loading method {}",
                version
            );
            return Err(DbFileParseError::new(OsuDbError, err_msg.as_str()));
        }
        let unknown_short = read_short(&bytes, &mut index)?;
        Ok(OsuDb {
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

    fn read_multi_thread(jobs: usize, bytes: Vec<u8>) -> ParseFileResult<Self> {
        let mut index = 0;
        let i = &mut index;
        let version = read_int(&bytes, i)?;
        let folder_count = read_int(&bytes, i)?;
        let account_unlocked = read_boolean(&bytes, i)?;
        let account_unlock_date = if !account_unlocked {
            Some(read_datetime(&bytes, i)?)
        } else {
            let _ = read_datetime(&bytes, i)?;
            None
        };
        let player_name = read_player_name(&bytes, i)?;
        let num_beatmaps = read_int(&bytes, &mut bytes_used)?;
        let counter = Arc::new(Mutex::new(0));
        let start = Arc::new(Mutex::new(bytes_used));
        let beatmaps = if version >= 20160408 && version < 20191107 {
            // Spawn a thread for each requested job, collect handles into a vec.
            let threads = (0..jobs)
                .map(|_| {
                    spawn_beatmap_loader_thread(
                        num_beatmaps as usize,
                        counter.clone(),
                        start.clone(),
                        &bytes,
                    )
                })
                .collect::<Vec<_>>();
            // Pull results from each thread.
            let mut results = threads
                .into_iter()
                .map(|joinhandle| joinhandle.join().unwrap())
                .collect::<Vec<_>>();
            let mut beatmaps = results.pop().unwrap()?;
            for beatmap_result in results {
                // I'm using a `for` loop here with the `pop` above instead of `into_iter()` and
                // `for_each()` or `fold()` because of the `?`. `?` only returns from the most
                // immediate function closure, and I want the `?` to return out of this method call.
                beatmaps.append(&mut beatmap_result?);
            }
            // Sort by their number so that the parsed data is in the same order as it appears in
            // the database file.
            beatmaps.sort_by(|(a, _), (b, _)| a.cmp(b));
            // Keep only the beatmaps - drop the counting number.
            beatmaps
                .into_iter()
                .map(|(_, beatmap)| beatmap)
                .collect::<Vec<_>>()
        } else if version / 1000 <= 2016 && version / 1000 >= 2007 || version / 1000 == 2019 {
            // Catch valid versions.
            return Err(DbFileParseError::new(
                OsuDbError,
                "osu!.db versions older than 20160408 and newer than and including \
                 20191107 do not support multithreaded loading.",
            ));
        } else {
            let err_msg = format!(
                "Read version with no associated beatmap loading method: {}",
                version
            );
            return Err(DbFileParseError::new(OsuDbError, err_msg.as_str()));
        };
        let unknown_short = read_short(&bytes, &mut *start.lock().unwrap())?;
        Ok(OsuDb {
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
fn spawn_beatmap_loader_thread(
    number: usize,
    counter: Arc<Mutex<usize>>,
    start: Arc<Mutex<usize>>,
    bytes_pointer: *const Vec<u8>,
) -> JoinHandle<ParseFileResult<Vec<(usize, Beatmap)>>> {
    // Cast the pointer to the file bytes to a usize so that we can pass it into the thread.
    let tmp = bytes_pointer as usize;
    thread::spawn(move || {
        // This pointer dereference is *technically* safe. Rust just needs the guarantee that the
        // data being referenced - the file data - will live longer than the reference to it, which
        // is `bytes`. Since the threads are `join()`ed in the usage of this function above before
        // the file bytes vector is dropped, this is technically safe to use. Just I'm the one
        // upholding the reference lifetime invariant rather than rustc.
        let bytes = unsafe { &*(tmp as *const Vec<u8>) };
        let mut beatmaps = Vec::new();
        loop {
            let (entry_size, mut start, num) = {
                // Lock the counter.
                let mut ctr = counter.lock().unwrap();
                if *ctr >= number {
                    // Return the collected beatmaps if the requisite number have been parsed.
                    return Ok(beatmaps);
                } else {
                    // Otherwise, increment the counter and carry on.
                    *ctr += 1;
                }
                // Lock the start index.
                let mut s = start.lock().unwrap();
                // Keep track of where the rest of the beatmap entry actually begins.
                let start_at = *s + 4;
                let entry_size = read_int(bytes, &mut *s)?;
                // Increase the start index by the size of this entry.
                *s += entry_size as usize;
                (entry_size, start_at, *ctr - 1)
                // Drop the lock on the counter and start index so other threads can get started on
                // parsing.
            };
            let i = &mut start;
            let artist_name = read_string_utf8(bytes, i, "non-Unicode artist name")?;
            let artist_name_unicode = read_string_utf8(bytes, i, "Unicode artist name")?;
            let song_title = read_string_utf8(bytes, i, "non-Unicode song title")?;
            let song_title_unicode = read_string_utf8(bytes, i, "Unicode song title")?;
            let creator_name = read_string_utf8(bytes, i, "creator name")?;
            let difficulty = read_string_utf8(bytes, i, "difficulty")?;
            let audio_file_name = read_string_utf8(bytes, i, "audio file name")?;
            let md5_beatmap_hash = read_md5_hash(bytes, i)?;
            let dotosu_file_name = read_string_utf8(bytes, i, "corresponding .osu file name")?;
            let ranked_status = RankedStatus::read_from_bytes(bytes, i)?;
            let number_of_hitcircles = read_short(bytes, i)?;
            let number_of_sliders = read_short(bytes, i)?;
            let number_of_spinners = read_short(bytes, i)?;
            let last_modification_time = read_datetime(bytes, i)?;
            let approach_rate = ModernWithEntrySize::read_arcshpod(bytes, i)?;
            let circle_size = ModernWithEntrySize::read_arcshpod(bytes, i)?;
            let hp_drain = ModernWithEntrySize::read_arcshpod(bytes, i)?;
            let overall_difficulty = ModernWithEntrySize::read_arcshpod(bytes, i)?;
            let slider_velocity = read_double(bytes, i)?;
            let (num_mcsr_standard, mcsr_standard) =
                ModernWithEntrySize::read_mod_combo_star_ratings(bytes, i)?;
            let (num_mcsr_taiko, mcsr_taiko) =
                ModernWithEntrySize::read_mod_combo_star_ratings(bytes, i)?;
            let (num_mcsr_ctb, mcsr_ctb) =
                ModernWithEntrySize::read_mod_combo_star_ratings(bytes, i)?;
            let (num_mcsr_mania, mcsr_mania) =
                ModernWithEntrySize::read_mod_combo_star_ratings(bytes, i)?;
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
            let unknown_short = ModernWithEntrySize::read_unknown_short(bytes, i)?;
            let offset_from_song_start_in_editor_ms = read_int(bytes, i)?;
            let mania_scroll_speed = read_byte(bytes, i)?;
            beatmaps.push((
                num,
                Beatmap {
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
                    mania_scroll_speed,
                },
            ));
        }
    })
}
