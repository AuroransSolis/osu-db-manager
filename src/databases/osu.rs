use std::io::{Result as IoResult, Error as IoError, ErrorKind};
use std::time::SystemTime;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use crate::deserialize_primitives::*;
use crate::databases::load::Load;

// Deserializing osu!.db-specific data types

#[inline]
fn read_int_double_pair<I: Iterator<Item = u8>>(i: &mut I) -> IoResult<(i32, f64)> {
    let int_double_pair_err = IoError::new(ErrorKind::Other,
        "Failed to read extraneous byte in int-double pair.");
    let _ = i.next().ok_or_else(|| int_double_pair_err)?;
    let int = read_int(i)?;
    let int_double_pair_err = IoError::new(ErrorKind::Other,
        "Failed to read extraneous byte in int-double pair.");
    let _ = i.next().ok_or_else(|| int_double_pair_err)?;
    let double = read_double(i)?;
    Ok((int, double))
}

#[derive(Copy, Clone, Debug)]
pub struct TimingPoint {
    bpm: f64,
    offset: f64,
    inherited: bool
}

impl TimingPoint {
    #[inline]
    fn read_from_bytes<I: Iterator<Item = u8>>(i: &mut I) -> IoResult<Self> {
        Ok(TimingPoint {
            bpm: read_double(i)?,
            offset: read_double(i)?,
            inherited: read_boolean(i)?
        })
    }
}

#[derive(Copy, Clone, Debug)]
pub enum RankedStatus {
    Unknown,
    Unsubmitted,
    PendingWIPGraveyard,
    Unused,
    Ranked,
    Approved,
    Qualified,
    Loved
}

use self::RankedStatus::*;

impl RankedStatus {
    #[inline]
    pub fn read_from_bytes<I: Iterator<Item = u8>>(i: &mut I) -> IoResult<Self> {
        let ranked_status_err = IoError::new(ErrorKind::Other,
            "Failed to read byte for ranked status.");
        let b = i.next().ok_or_else(|| ranked_status_err)?;
        match b {
            0 => Ok(Unknown),
            1 => Ok(Unsubmitted),
            2 => Ok(PendingWIPGraveyard),
            3 => Ok(Unused),
            4 => Ok(Ranked),
            5 => Ok(Approved),
            6 => Ok(Qualified),
            7 => Ok(Loved),
            _ => {
                let err_msg = format!("Found invalid ranked status value ({})", b);
                Err(IoError::new(ErrorKind::InvalidData, err_msg.as_str()))
            }
        }
    }
}

impl Display for RankedStatus {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", match self {
            Unknown => "Unknown",
            Unsubmitted => "Unsubmitted",
            PendingWIPGraveyard => "Pending/WIP/Graveyard",
            Unused => "Unused",
            Ranked => "Ranked",
            Approved => "Approved",
            Qualified => "Qualified",
            Loved => "Loved"
        })
    }
}

#[derive(Copy, Clone, Debug)]
pub enum ByteSingle {
    Byte(u8),
    Single(f32)
}

use self::ByteSingle::*;

impl Display for ByteSingle {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", match self {
            Byte(b) => format!("{}", b),
            Single(s) => format!("{}", s)
        })
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Legacy;

#[derive(Copy, Clone, Debug)]
pub struct Modern;

#[derive(Copy, Clone, Debug)]
pub struct ModernWithEntrySize;

trait ReadVersionSpecificData {
    fn read_entry_size<I: Iterator<Item = u8>>(i: &mut I) -> IoResult<Option<i32>>;
    fn read_arcshpod<I: Iterator<Item = u8>>(i: &mut I) -> IoResult<ByteSingle>;
    fn read_mod_combo_star_ratings<I: Iterator<Item = u8>>(i: &mut I)
        -> IoResult<(Option<i32>, Option<Vec<(i32, f64)>>)>;
    fn read_unknown_short<I: Iterator<Item = u8>>(i: &mut I) -> IoResult<Option<i16>>;
}

impl ReadVersionSpecificData for Legacy {
    #[inline]
    fn read_entry_size<I: Iterator<Item = u8>>(_i: &mut I) -> IoResult<Option<i32>> {
        Ok(None)
    }

    #[inline]
    fn read_arcshpod<I: Iterator<Item = u8>>(i: &mut I) -> IoResult<ByteSingle> {
        Ok(Byte(i.next().ok_or_else(|| IoError::new(ErrorKind::Other,
            "Failed to get byte for AR/CS/HP/OD"))?))
    }

    #[inline]
    fn read_mod_combo_star_ratings<I: Iterator<Item = u8>>(_i: &mut I)
        -> IoResult<(Option<i32>, Option<Vec<(i32, f64)>>)> {
        Ok((None, None))
    }

    #[inline]
    fn read_unknown_short<I: Iterator<Item = u8>>(i: &mut I) -> IoResult<Option<i16>> {
        Ok(Some(read_short(i)?))
    }
}

impl ReadVersionSpecificData for Modern {
    #[inline]
    fn read_entry_size<I: Iterator<Item = u8>>(_i: &mut I) -> IoResult<Option<i32>> {
        Ok(None)
    }

    #[inline]
    fn read_arcshpod<I: Iterator<Item = u8>>(i: &mut I) -> IoResult<ByteSingle> {
        Ok(Single(read_single(i)?))
    }

    #[inline]
    fn read_mod_combo_star_ratings<I: Iterator<Item = u8>>(i: &mut I)
        -> IoResult<(Option<i32>, Option<Vec<(i32, f64)>>)> {
        let num_int_doubles = read_int(i)?;
        let mut int_double_pairs = Vec::with_capacity(num_int_doubles as usize);
        for _ in 0..num_int_doubles {
            int_double_pairs.push(read_int_double_pair(i)?);
        }
        Ok((Some(num_int_doubles), Some(int_double_pairs)))
    }

    #[inline]
    fn read_unknown_short<I: Iterator<Item = u8>>(_i: &mut I) -> IoResult<Option<i16>> {
        Ok(None)
    }
}

impl ReadVersionSpecificData for ModernWithEntrySize {
    #[inline]
    fn read_entry_size<I: Iterator<Item = u8>>(i: &mut I) -> IoResult<Option<i32>> {
        Ok(Some(read_int(i)?))
    }

    #[inline]
    fn read_arcshpod<I: Iterator<Item = u8>>(i: &mut I) -> IoResult<ByteSingle> {
        Ok(Single(read_single(i)?))
    }

    #[inline]
    fn read_mod_combo_star_ratings<I: Iterator<Item = u8>>(i: &mut I)
        -> IoResult<(Option<i32>, Option<Vec<(i32, f64)>>)> {
        let num_int_doubles = read_int(i)?;
        let mut int_double_pairs = Vec::with_capacity(num_int_doubles as usize);
        for _ in 0..num_int_doubles {
            int_double_pairs.push(read_int_double_pair(i)?);
        }
        Ok((Some(num_int_doubles), Some(int_double_pairs)))
    }

    #[inline]
    fn read_unknown_short<I: Iterator<Item = u8>>(_i: &mut I) -> IoResult<Option<i16>> {
        Ok(None)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum GameplayMode {
    Standard,
    Taiko,
    Ctb,
    Mania
}

use self::GameplayMode::*;

impl GameplayMode {
    #[inline]
    pub fn read_from_bytes<I: Iterator<Item = u8>>(i: &mut I) -> IoResult<Self> {
        let gameplay_mode_err = IoError::new(ErrorKind::Other,
            "Failed to read byte for gameplay mode specifier.");
        let b = i.next().ok_or_else(|| gameplay_mode_err)?;
        match b {
            0 => Ok(Standard),
            1 => Ok(Taiko),
            2 => Ok(Ctb),
            3 => Ok(Mania),
            _ => {
                let err_msg = format!("Read invalid gamemode specifier ({})", b);
                Err(IoError::new(ErrorKind::InvalidData, err_msg.as_str()))
            }
        }
    }
}

impl Display for GameplayMode {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", match self {
            Standard => "osu!standard",
            Taiko => "Taiko",
            Ctb => "CTB",
            Mania => "osu!mania"
        })
    }
}

#[derive(Clone, Debug)]
pub struct Beatmap {
    entry_size: Option<i32>,
    artist_name: String,
    artist_name_unicode: String,
    song_title: String,
    song_title_unicode: String,
    creator_name: String,
    difficulty: String,
    audio_file_name: String,
    md5_beatmap_hash: String,
    dotosu_file_name: String,
    ranked_status: RankedStatus,
    number_of_hitcircles: i16,
    number_of_sliders: i16,
    number_of_spinners: i16,
    last_modification_time: SystemTime,
    approach_rate: ByteSingle,
    circle_size: ByteSingle,
    hp_drain: ByteSingle,
    overall_difficulty: ByteSingle,
    slider_velocity: f64,
    num_mod_combo_star_ratings_standard: Option<i32>,
    mod_combo_star_ratings_standard: Option<Vec<(i32, f64)>>,
    num_mod_combo_star_ratings_taiko: Option<i32>,
    mod_combo_star_ratings_taiko: Option<Vec<(i32, f64)>>,
    num_mod_combo_star_ratings_ctb: Option<i32>,
    mod_combo_star_ratings_ctb: Option<Vec<(i32, f64)>>,
    num_mod_combo_star_ratings_mania: Option<i32>,
    mod_combo_star_ratings_mania: Option<Vec<(i32, f64)>>,
    drain_time: i32,
    total_time: i32,
    preview_offset_from_start_ms: i32,
    num_timing_points: i32,
    timing_points: Vec<TimingPoint>,
    beatmap_id: i32,
    beatmap_set_id: i32,
    thread_id: i32,
    standard_grade: u8,
    taiko_grade: u8,
    ctb_grade: u8,
    mania_grade: u8,
    local_offset: i16,
    stack_leniency: f32,
    gameplay_mode: GameplayMode,
    song_source: String,
    song_tags: String,
    online_offset: i16,
    font_used_for_song_title: String,
    unplayed: bool,
    last_played: SystemTime,
    is_osz2: bool,
    beatmap_folder_name: String,
    last_checked_against_repo: SystemTime,
    ignore_beatmap_sound: bool,
    ignore_beatmap_skin: bool,
    disable_storyboard: bool,
    disable_video: bool,
    visual_override: bool,
    unknown_short: Option<i16>,
    offset_from_song_start_in_editor_ms: i32,
    mania_scroll_speed: u8
}

impl Beatmap {
    fn read_from_bytes<T: ReadVersionSpecificData, I: Iterator<Item = u8>>(i: &mut I) -> IoResult<Self> {
        let entry_size = T::read_entry_size(i)?;
        let artist_name = fromutf8_to_ioresult(read_string_utf8(i)?, "non-Unicode artist name")?;
        let artist_name_unicode = fromutf8_to_ioresult(read_string_utf8(i)?,
            "Unicode artist name")?;
        let song_title = fromutf8_to_ioresult(read_string_utf8(i)?, "non-Unicode song title")?;
        let song_title_unicode = fromutf8_to_ioresult(read_string_utf8(i)?,
            "Unicode song title")?;
        let creator_name = fromutf8_to_ioresult(read_string_utf8(i)?, "creator name")?;
        let difficulty = fromutf8_to_ioresult(read_string_utf8(i)?, "difficulty")?;
        let audio_file_name = fromutf8_to_ioresult(read_string_utf8(i)?, "audio file name")?;
        let md5_beatmap_hash = read_md5_hash(i).map_err(|e| {
            let msg = format!("Error reading MD5 beatmap hash: {}", e);
            IoError::new(ErrorKind::Other, msg.as_str())
        })?;
        let dotosu_file_name = fromutf8_to_ioresult(read_string_utf8(i)?,
            "corresponding .osu file name")?;
        let ranked_status = RankedStatus::read_from_bytes(i)?;
        let number_of_hitcircles = read_short(i)?;
        let number_of_sliders = read_short(i)?;
        let number_of_spinners = read_short(i)?;
        let last_modification_time = read_datetime(i)?;
        let approach_rate = T::read_arcshpod(i)?;
        let circle_size = T::read_arcshpod(i)?;
        let hp_drain = T::read_arcshpod(i)?;
        let overall_difficulty = T::read_arcshpod(i)?;
        let slider_velocity = read_double(i)?;
        let (num_mcsr_standard, mcsr_standard) = T::read_mod_combo_star_ratings(i)?;
        let (num_mcsr_taiko, mcsr_taiko) = T::read_mod_combo_star_ratings(i)?;
        let (num_mcsr_ctb, mcsr_ctb) = T::read_mod_combo_star_ratings(i)?;
        let (num_mcsr_mania, mcsr_mania) = T::read_mod_combo_star_ratings(i)?;
        let drain_time = read_int(i)?;
        let total_time = read_int(i)?;
        let preview_offset_from_start_ms = read_int(i)?;
        let num_timing_points = read_int(i)?;
        let mut timing_points = Vec::with_capacity(num_timing_points as usize);
        for _ in 0..num_timing_points {
            timing_points.push(TimingPoint::read_from_bytes(i)?);
        }
        let beatmap_id = read_int(i)?;
        let beatmap_set_id = read_int(i)?;
        let thread_id = read_int(i)?;
        let standard_grade = read_byte(i)?;
        let taiko_grade = read_byte(i)?;
        let ctb_grade = read_byte(i)?;
        let mania_grade = read_byte(i)?;
        let local_offset = read_short(i)?;
        let stack_leniency = read_single(i)?;
        let gameplay_mode = GameplayMode::read_from_bytes(i)?;
        let song_source = fromutf8_to_ioresult(read_string_utf8(i)?, "song source")?;
        let song_tags = fromutf8_to_ioresult(read_string_utf8(i)?, "song tags")?;
        let online_offset = read_short(i)?;
        let font_used_for_song_title = fromutf8_to_ioresult(read_string_utf8(i)?,
            "font used for song title")?;
        let unplayed = read_boolean(i)?;
        let last_played = read_datetime(i)?;
        let is_osz2 = read_boolean(i)?;
        let beatmap_folder_name = fromutf8_to_ioresult(read_string_utf8(i)?,
            "folder name")?;
        let last_checked_against_repo = read_datetime(i)?;
        let ignore_beatmap_sound = read_boolean(i)?;
        let ignore_beatmap_skin = read_boolean(i)?;
        let disable_storyboard = read_boolean(i)?;
        let disable_video = read_boolean(i)?;
        let visual_override = read_boolean(i)?;
        let unknown_short = T::read_unknown_short(i)?;
        let offset_from_song_start_in_editor_ms = read_int(i)?;
        let mania_scroll_speed = read_byte(i)?;
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
            mania_scroll_speed
        })
    }
}

#[derive(Debug, Clone)]
pub struct OsuDb {
    pub version: i32,
    pub folder_count: i32,
    pub account_unlocked: bool,
    pub account_unlock_date: Option<SystemTime>,
    pub player_name: String,
    pub number_of_beatmaps: i32,
    pub beatmaps: Vec<Beatmap>,
    pub unknown_int: i32
}

impl Load for OsuDb {
    fn read_single_thread(bytes: Vec<u8>) -> IoResult<Self> {
        let mut bytes_iter = bytes.into_iter();
        let version = read_int(&mut bytes_iter)?;
        let folder_count = read_int(&mut bytes_iter)?;
        let account_unlocked = read_boolean(&mut bytes_iter)?;
        let account_unlock_date = if !account_unlocked {
            Some(read_datetime(&mut bytes_iter)?)
        } else {
            let _ = read_datetime(&mut bytes_iter)?;
            None
        };
        let player_name = fromutf8_to_ioresult(read_string_utf8(&mut bytes_iter)?, "player name")?;
        let num_beatmaps = read_int(&mut bytes_iter)?;
        let mut beatmaps = Vec::with_capacity(num_beatmaps as usize);
        if version < 20140609 {
            for _ in 0..num_beatmaps {
                beatmaps.push(Beatmap::read_from_bytes::<Legacy, _>(&mut bytes_iter)?);
            }
        } else if version >= 20140609 && version < 20160408 {
            for _ in 0..num_beatmaps {
                beatmaps.push(Beatmap::read_from_bytes::<Modern, _>(&mut bytes_iter)?);
            }
        } else if version >= 20160408 {
            for _ in 0..num_beatmaps {
                beatmaps.push(Beatmap::read_from_bytes::<ModernWithEntrySize, _>(&mut bytes_iter)?);
            }
        } else {
            let err_msg = format!("Read version with no associated beatmap loading method {}",
                version);
            return Err(IoError::new(ErrorKind::InvalidData, err_msg.as_str()));
        }
        let unknown_int = read_int(&mut bytes_iter)?;
        Ok(OsuDb {
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

    fn read_multi_thread(jobs: usize, bytes: Vec<u8>) -> IoResult<Self> {
        let (version, folder_count, account_unlocked, account_unlock_date, player_name,
            bytes_used) = {
            let mut bytes_iter = bytes.iter().cloned();
            let version = read_int(&mut bytes_iter)?;
            let folder_count = read_int(&mut bytes_iter)?;
            let account_unlocked = read_boolean(&mut bytes_iter)?;
            let account_unlock_date = if !account_unlocked {
                Some(read_datetime(&mut bytes_iter)?)
            } else {
                let _ = read_datetime(&mut bytes_iter)?;
                None
            };
            let (player_name_len, player_name) = read_player_name_with_len(&mut bytes_iter)?;
            //println!("        Loaded : {}", );
            // version: 4
            // folder_count: 4
            // account_unlocked: 1
            // account_unlock_date: 8
            // player_name_string: player_name_len
            let bytes_used = 4 + 4 + 1 + 8 + player_name_len;
            (version, folder_count, account_unlocked, account_unlock_date, player_name, bytes_used)
        };
        let num_beatmaps = read_int(&mut bytes.iter().skip(bytes_used).cloned())?;
        let beatmaps = if version >= 20160408 {
            let counter = Arc::new(Mutex::new(0));
            let start_read = Arc::new(Mutex::new(bytes_used + 4));
            let threads = (0..jobs)
                .map(|_| spawn_beatmap_loader_thread(num_beatmaps as usize, counter.clone(),
                    start_read.clone(), &bytes)).collect::<Vec<_>>();
            let mut results = threads.into_iter().map(|joinhandle| joinhandle.join().unwrap())
                .collect::<Vec<_>>();
            let mut beatmaps = results.pop().unwrap()?;
            for beatmap_result in results {
                beatmaps.append(&mut beatmap_result?);
            }
            beatmaps.sort_by(|(a, _), (b, _)| a.cmp(b));
            beatmaps.into_iter().map(|(_, beatmap)| beatmap).collect::<Vec<_>>()
        } else if version / 1000 <= 2016 && version / 1000 >= 2007 { // catch valid versions
            return Err(IoError::new(ErrorKind::Other, "osu!.db versions older than 20160408 do not \
                support multithreaded loading."));
        } else {
            let err_msg = format!("Read version with no associated beatmap loading method: {}",
                version);
            return Err(IoError::new(ErrorKind::InvalidData, err_msg.as_str()));
        };
        let unknown_int = read_int(&mut (&bytes[bytes.len() - 4..bytes.len()]).iter().cloned())?;
        Ok(OsuDb {
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
fn spawn_beatmap_loader_thread(number: usize, counter: Arc<Mutex<usize>>,
    start_read: Arc<Mutex<usize>>, bytes_pointer: *const Vec<u8>)
    -> JoinHandle<IoResult<Vec<(usize, Beatmap)>>> {
    let tmp = bytes_pointer as usize;
    thread::spawn(move || {
        let bytes = unsafe { &*(tmp as *const Vec<u8>) };
        let mut beatmaps = Vec::new();
        loop {
            let (entry_size, start, num) = {
                let mut ctr = counter.lock().unwrap();
                if *ctr >= number {
                    return Ok(beatmaps);
                } else {
                    *ctr += 1;
                }
                let mut start = start_read.lock().unwrap();
                let start_at = *start + 4;
                let entry_size = read_int(&mut (&bytes[*start..*start + 4]).iter().cloned())?;
                *start += entry_size as usize + 4;
                (entry_size, start_at, *ctr - 1)
            };
            let mut i = bytes[start..start + entry_size as usize].iter().cloned();
            let artist_name = fromutf8_to_ioresult(read_string_utf8(&mut i)?, "non-Unicode artist name")?;
            let artist_name_unicode = fromutf8_to_ioresult(read_string_utf8(&mut i)?,
                "Unicode artist name")?;
            let song_title = fromutf8_to_ioresult(read_string_utf8(&mut i)?, "non-Unicode song title")?;
            let song_title_unicode = fromutf8_to_ioresult(read_string_utf8(&mut i)?,
                "Unicode song title")?;
            let creator_name = fromutf8_to_ioresult(read_string_utf8(&mut i)?, "creator name")?;
            let difficulty = fromutf8_to_ioresult(read_string_utf8(&mut i)?, "difficulty")?;
            let audio_file_name = fromutf8_to_ioresult(read_string_utf8(&mut i)?, "audio file name")?;
            let md5_beatmap_hash = read_md5_hash(&mut i).map_err(|e| {
                let msg = format!("Error reading MD5 beatmap hash: {}", e);
                IoError::new(ErrorKind::Other, msg.as_str())
            })?;
            let dotosu_file_name = fromutf8_to_ioresult(read_string_utf8(&mut i)?,
                "corresponding .osu file name")?;
            let ranked_status = RankedStatus::read_from_bytes(&mut i)?;
            let number_of_hitcircles = read_short(&mut i)?;
            let number_of_sliders = read_short(&mut i)?;
            let number_of_spinners = read_short(&mut i)?;
            let last_modification_time = read_datetime(&mut i)?;
            let approach_rate = ModernWithEntrySize::read_arcshpod(&mut i)?;
            let circle_size = ModernWithEntrySize::read_arcshpod(&mut i)?;
            let hp_drain = ModernWithEntrySize::read_arcshpod(&mut i)?;
            let overall_difficulty = ModernWithEntrySize::read_arcshpod(&mut i)?;
            let slider_velocity = read_double(&mut i)?;
            let (num_mcsr_standard, mcsr_standard) = ModernWithEntrySize::read_mod_combo_star_ratings(&mut i)?;
            let (num_mcsr_taiko, mcsr_taiko) = ModernWithEntrySize::read_mod_combo_star_ratings(&mut i)?;
            let (num_mcsr_ctb, mcsr_ctb) = ModernWithEntrySize::read_mod_combo_star_ratings(&mut i)?;
            let (num_mcsr_mania, mcsr_mania) = ModernWithEntrySize::read_mod_combo_star_ratings(&mut i)?;
            let drain_time = read_int(&mut i)?;
            let total_time = read_int(&mut i)?;
            let preview_offset_from_start_ms = read_int(&mut i)?;
            let num_timing_points = read_int(&mut i)?;
            let mut timing_points = Vec::with_capacity(num_timing_points as usize);
            for _ in 0..num_timing_points {
                timing_points.push(TimingPoint::read_from_bytes(&mut i)?);
            }
            let beatmap_id = read_int(&mut i)?;
            let beatmap_set_id = read_int(&mut i)?;
            let thread_id = read_int(&mut i)?;
            let standard_grade = read_byte(&mut i)?;
            let taiko_grade = read_byte(&mut i)?;
            let ctb_grade = read_byte(&mut i)?;
            let mania_grade = read_byte(&mut i)?;
            let local_offset = read_short(&mut i)?;
            let stack_leniency = read_single(&mut i)?;
            let gameplay_mode = GameplayMode::read_from_bytes(&mut i)?;
            let song_source = fromutf8_to_ioresult(read_string_utf8(&mut i)?, "song source")?;
            let song_tags = fromutf8_to_ioresult(read_string_utf8(&mut i)?, "song tags")?;
            let online_offset = read_short(&mut i)?;
            let font_used_for_song_title = fromutf8_to_ioresult(read_string_utf8(&mut i)?,
                "font used for song title")?;
            let unplayed = read_boolean(&mut i)?;
            let last_played = read_datetime(&mut i)?;
            let is_osz2 = read_boolean(&mut i)?;
            let beatmap_folder_name = fromutf8_to_ioresult(read_string_utf8(&mut i)?,
                "folder name")?;
            let last_checked_against_repo = read_datetime(&mut i)?;
            let ignore_beatmap_sound = read_boolean(&mut i)?;
            let ignore_beatmap_skin = read_boolean(&mut i)?;
            let disable_storyboard = read_boolean(&mut i)?;
            let disable_video = read_boolean(&mut i)?;
            let visual_override = read_boolean(&mut i)?;
            let unknown_short = ModernWithEntrySize::read_unknown_short(&mut i)?;
            let offset_from_song_start_in_editor_ms = read_int(&mut i)?;
            let mania_scroll_speed = read_byte(&mut i)?;
            beatmaps.push((num, Beatmap {
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
