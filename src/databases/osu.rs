use std::io::{Result as IoResult, Error as IoError, ErrorKind};
use std::time::SystemTime;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use crate::deserialize_primitives::*;
use crate::databases::load::Load;
use crate::read_error::{ParseFileResult, DbFileParseError, ParseErrorKind::*};

// Deserializing osu!.db-specific data types

const INT_DOUBLE_PAIR_ERR: &str = "Failed to read extraneous byte in int-double pair.";
const RANKED_STATUS_ERR: &str = "Failed to read byte for ranked status.";
const GAMEPLAY_MODE_ERR: &str = "Failed to read byte for gameplay mode specifier.";

macro_rules! primitive {
    ($msg:ident) => {{
        DbFileParseError::new(PrimitiveError, $msg)
    }};
}

#[inline]
pub fn read_int_double_pair(bytes: &[u8], i: &mut usize) -> ParseFileResult<(i32, f64)> {
    *i += 1;
    let int = read_int(bytes, i)?;
    *i += 1;
    let double = read_double(bytes, i)?;
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
    fn read_from_bytes(bytes: &[u8], i: &mut usize) -> ParseFileResult<Self> {
        Ok(TimingPoint {
            bpm: read_double(bytes, i)?,
            offset: read_double(bytes, i)?,
            inherited: read_boolean(bytes, i)?
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
    pub fn read_from_bytes(bytes: &[u8], i: &mut usize) -> ParseFileResult<Self> {
        match read_byte(bytes, i).map_err(|_| primitive!(RANKED_STATUS_ERR))? {
            0 => Ok(Unknown),
            1 => Ok(Unsubmitted),
            2 => Ok(PendingWIPGraveyard),
            3 => Ok(Unused),
            4 => Ok(Ranked),
            5 => Ok(Approved),
            6 => Ok(Qualified),
            7 => Ok(Loved),
            b @ _ => {
                let err_msg = format!("Found invalid ranked status value ({})", b);
                Err(DbFileParseError::new(PrimitiveError, err_msg.as_str()))
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
    fn read_entry_size(bytes: &[u8], i: &mut usize) -> ParseFileResult<Option<i32>>;
    fn read_arcshpod(bytes: &[u8], i: &mut usize) -> ParseFileResult<ByteSingle>;
    fn read_mod_combo_star_ratings(bytes: &[u8], i: &mut usize)
        -> ParseFileResult<(Option<i32>, Option<Vec<(i32, f64)>>)>;
    fn read_unknown_short(bytes: &[u8], i: &mut usize) -> ParseFileResult<Option<i16>>;
}

impl ReadVersionSpecificData for Legacy {
    #[inline]
    fn read_entry_size(_bytes: &[u8], _i: &mut usize) -> ParseFileResult<Option<i32>> {
        Ok(None)
    }

    #[inline]
    fn read_arcshpod(bytes: &[u8], i: &mut usize) -> ParseFileResult<ByteSingle> {
        Ok(Byte(read_byte(bytes, i)?))
    }

    #[inline]
    fn read_mod_combo_star_ratings(_bytes: &[u8], _i: &mut usize)
        -> ParseFileResult<(Option<i32>, Option<Vec<(i32, f64)>>)> {
        Ok((None, None))
    }

    #[inline]
    fn read_unknown_short(bytes: &[u8], i: &mut usize) -> ParseFileResult<Option<i16>> {
        Ok(Some(read_short(bytes, i)?))
    }
}

impl ReadVersionSpecificData for Modern {
    #[inline]
    fn read_entry_size(_bytes: &[u8], _i: &mut usize) -> ParseFileResult<Option<i32>> {
        Ok(None)
    }

    #[inline]
    fn read_arcshpod(bytes: &[u8], i: &mut usize) -> ParseFileResult<ByteSingle> {
        Ok(Single(read_single(bytes, i)?))
    }

    #[inline]
    fn read_mod_combo_star_ratings(bytes: &[u8], i: &mut usize)
        -> ParseFileResult<(Option<i32>, Option<Vec<(i32, f64)>>)> {
        let num_int_doubles = read_int(bytes, i)?;
        let mut int_double_pairs = Vec::with_capacity(num_int_doubles as usize);
        for _ in 0..num_int_doubles {
            int_double_pairs.push(read_int_double_pair(bytes, i)?);
        }
        Ok((Some(num_int_doubles), Some(int_double_pairs)))
    }

    #[inline]
    fn read_unknown_short(_bytes: &[u8], _i: &mut usize) -> ParseFileResult<Option<i16>> {
        Ok(None)
    }
}

impl ReadVersionSpecificData for ModernWithEntrySize {
    #[inline]
    fn read_entry_size(bytes: &[u8], i: &mut usize) -> ParseFileResult<Option<i32>> {
        Ok(Some(read_int(bytes, i)?))
    }

    #[inline]
    fn read_arcshpod(bytes: &[u8], i: &mut usize) -> ParseFileResult<ByteSingle> {
        Ok(Single(read_single(bytes, i)?))
    }

    #[inline]
    fn read_mod_combo_star_ratings(bytes: &[u8], i: &mut usize)
        -> ParseFileResult<(Option<i32>, Option<Vec<(i32, f64)>>)> {
        let num_int_doubles = read_int(bytes, i)?;
        let mut int_double_pairs = Vec::with_capacity(num_int_doubles as usize);
        for _ in 0..num_int_doubles {
            int_double_pairs.push(read_int_double_pair(bytes, i)?);
        }
        Ok((Some(num_int_doubles), Some(int_double_pairs)))
    }

    #[inline]
    fn read_unknown_short(_bytes: &[u8], _i: &mut usize) -> ParseFileResult<Option<i16>> {
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
use core::borrow::BorrowMut;

impl GameplayMode {
    #[inline]
    pub fn read_from_bytes(bytes: &[u8], i: &mut usize) -> ParseFileResult<Self> {
        let b = read_byte(bytes, i).map_err(|_| primitive!(GAMEPLAY_MODE_ERR))?;
        match b {
            0 => Ok(Standard),
            1 => Ok(Taiko),
            2 => Ok(Ctb),
            3 => Ok(Mania),
            _ => {
                let err_msg = format!("Read invalid gamemode specifier ({})", b);
                Err(DbFileParseError::new(PrimitiveError, err_msg.as_str()))
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
    pub ranked_status: RankedStatus,
    pub number_of_hitcircles: i16,
    pub number_of_sliders: i16,
    pub number_of_spinners: i16,
    pub last_modification_time: SystemTime,
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
    pub song_source: Option<String>,
    pub song_tags: Option<String>,
    pub online_offset: i16,
    pub font_used_for_song_title: Option<String>,
    pub unplayed: bool,
    pub last_played: SystemTime,
    pub is_osz2: bool,
    pub beatmap_folder_name: Option<String>,
    pub last_checked_against_repo: SystemTime,
    pub ignore_beatmap_sound: bool,
    pub ignore_beatmap_skin: bool,
    pub disable_storyboard: bool,
    pub disable_video: bool,
    pub visual_override: bool,
    pub unknown_short: Option<i16>,
    pub offset_from_song_start_in_editor_ms: i32,
    pub mania_scroll_speed: u8
}

impl Beatmap {
    fn read_from_bytes<T: ReadVersionSpecificData>(bytes: &[u8], i: &mut usize)
        -> ParseFileResult<Self> {
        let entry_size = T::read_entry_size(bytes, i)?;
        let artist_name = read_string_utf8(bytes, i, "non-Unicode artist name")?;
        let artist_name_unicode = read_string_utf8(bytes, i, "Unicode artist name")?;
        let song_title = read_string_utf8(bytes, i, "non-Unicode song title")?;
        let song_title_unicode = read_string_utf8(bytes, i, "Unicode song title")?;
        let creator_name = read_player_name(bytes, i).map_err(|_| {
            let msg = format!("Error reading creator name.");
            DbFileParseError::new(PrimitiveError, msg.as_str())
        })?;
        let difficulty = read_string_utf8(bytes, i, "difficulty")?;
        let audio_file_name = read_string_utf8(bytes, i, "audio file name")?;
        let md5_beatmap_hash = read_md5_hash(bytes, i)?;
        let dotosu_file_name = read_string_utf8(bytes, i, "corresponding .osu file name")?;
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
    pub player_name: Option<String>,
    pub number_of_beatmaps: i32,
    pub beatmaps: Vec<Beatmap>,
    pub unknown_int: i32
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
        if version < 20140609 {
            for _ in 0..num_beatmaps {
                beatmaps.push(Beatmap::read_from_bytes::<Legacy>(&bytes, &mut index)?);
            }
        } else if version >= 20140609 && version < 20160408 {
            for _ in 0..num_beatmaps {
                beatmaps.push(Beatmap::read_from_bytes::<Modern>(&bytes, &mut index)?);
            }
        } else if version >= 20160408 {
            for _ in 0..num_beatmaps {
                beatmaps.push(Beatmap::read_from_bytes::<ModernWithEntrySize>(&bytes, &mut index)?);
            }
        } else {
            let err_msg = format!("Read version with no associated beatmap loading method {}",
                version);
            return Err(DbFileParseError::new(OsuDbError, err_msg.as_str()));
        }
        let unknown_int = read_int(&bytes, &mut index)?;
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

    fn read_multi_thread(jobs: usize, bytes: Vec<u8>) -> ParseFileResult<Self> {
        let (version, folder_count, account_unlocked, account_unlock_date, player_name,
            mut bytes_used) = {
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
            let (player_name_len, player_name) = read_player_name_with_len(&bytes, i)?;
            // version: 4
            // folder_count: 4
            // account_unlocked: 1
            // account_unlock_date: 8
            // player_name_string: player_name_len
            let bytes_used = 4 + 4 + 1 + 8 + player_name_len;
            (version, folder_count, account_unlocked, account_unlock_date, player_name, bytes_used)
        };
        let num_beatmaps = read_int(&bytes, &mut bytes_used)?;
        let counter = Arc::new(Mutex::new(0));
        let start = Arc::new(Mutex::new(bytes_used + 4));
        let beatmaps = if version >= 20160408 {
            let threads = (0..jobs)
                .map(|_| spawn_beatmap_loader_thread(num_beatmaps as usize, counter.clone(),
                    start.clone(), &bytes)).collect::<Vec<_>>();
            let mut results = threads.into_iter().map(|joinhandle| joinhandle.join().unwrap())
                .collect::<Vec<_>>();
            let mut beatmaps = results.pop().unwrap()?;
            for beatmap_result in results {
                beatmaps.append(&mut beatmap_result?);
            }
            beatmaps.sort_by(|(a, _), (b, _)| a.cmp(b));
            beatmaps.into_iter().map(|(_, beatmap)| beatmap).collect::<Vec<_>>()
        } else if version / 1000 <= 2016 && version / 1000 >= 2007 { // catch valid versions
            return Err(DbFileParseError::new(OsuDbError, "osu!.db versions older than 20160408 do \
                not support multithreaded loading."));
        } else {
            let err_msg = format!("Read version with no associated beatmap loading method: {}",
                version);
            return Err(DbFileParseError::new(OsuDbError, err_msg.as_str()));
        };
        let unknown_int = read_int(&bytes, &mut *start.lock().unwrap())?;
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
fn spawn_beatmap_loader_thread(number: usize, counter: Arc<Mutex<usize>>, start: Arc<Mutex<usize>>,
    bytes_pointer: *const Vec<u8>) -> JoinHandle<ParseFileResult<Vec<(usize, Beatmap)>>> {
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
                *s += 4;
                println!("{} => {}", start_at, *s);
                (entry_size, start_at, *ctr - 1)
            };
            let i = &mut start;
            let artist_name = read_string_utf8(bytes, i, "non-Unicode artist name")?;
            println!("{} | {}: {:?}", num, stringify!(artist_name), artist_name);
            let artist_name_unicode = read_string_utf8(bytes, i, "Unicode artist name")?;
            println!("{} | {}: {:?}", num, stringify!(artist_name_unicode), artist_name_unicode);
            let song_title = read_string_utf8(bytes, i, "non-Unicode song title")?;
            println!("{} | {}: {:?}", num, stringify!(song_title), song_title);
            let song_title_unicode = read_string_utf8(bytes, i, "Unicode song title")?;
            println!("{} | {}: {:?}", num, stringify!(song_title_unicode), song_title_unicode);
            let creator_name = read_string_utf8(bytes, i, "creator name")?;
            println!("{} | {}: {:?}", num, stringify!(creator_name), creator_name);
            let difficulty = read_string_utf8(bytes, i, "difficulty")?;
            println!("{} | {}: {:?}", num, stringify!(difficulty), difficulty);
            let audio_file_name = read_string_utf8(bytes, i, "audio file name")?;
            println!("{} | {}: {:?}", num, stringify!(audio_file_name), audio_file_name);
            let md5_beatmap_hash = read_md5_hash(bytes, i)?;
            println!("{} | {}: {:?}", num, stringify!(md5_beatmap_hash), md5_beatmap_hash);
            let dotosu_file_name = read_string_utf8(bytes, i, "corresponding .osu file name")?;
            println!("{} | {}: {:?}", num, stringify!(dotosu_file_name), dotosu_file_name);
            let ranked_status = RankedStatus::read_from_bytes(bytes, i)?;
            println!("{} | {}: {}", num, stringify!(ranked_status), ranked_status);
            let number_of_hitcircles = read_short(bytes, i)?;
            println!("{} | {}: {}", num, stringify!(number_of_hitcircles), number_of_hitcircles);
            let number_of_sliders = read_short(bytes, i)?;
            println!("{} | {}: {}", num, stringify!(number_of_sliders), number_of_sliders);
            let number_of_spinners = read_short(bytes, i)?;
            println!("{} | {}: {}", num, stringify!(number_of_spinners), number_of_spinners);
            let last_modification_time = read_datetime(bytes, i)?;
            println!("{} | {}: {:?}", num, stringify!(last_modification_time), last_modification_time);
            let approach_rate = ModernWithEntrySize::read_arcshpod(bytes, i)?;
            println!("{} | {}: {}", num, stringify!(approach_rate), approach_rate);
            let circle_size = ModernWithEntrySize::read_arcshpod(bytes, i)?;
            println!("{} | {}: {}", num, stringify!(circle_size), circle_size);
            let hp_drain = ModernWithEntrySize::read_arcshpod(bytes, i)?;
            println!("{} | {}: {}", num, stringify!(hp_drain), hp_drain);
            let overall_difficulty = ModernWithEntrySize::read_arcshpod(bytes, i)?;
            println!("{} | {}: {}", num, stringify!(overall_difficulty), overall_difficulty);
            let slider_velocity = read_double(bytes, i)?;
            println!("{} | {}: {}", num, stringify!(slider_velocity), slider_velocity);
            let (num_mcsr_standard, mcsr_standard)
                = ModernWithEntrySize::read_mod_combo_star_ratings(bytes, i)?;
            println!("{} | {}: {:?}", num, stringify!(num_mcsr_standard), num_mcsr_standard);
            let (num_mcsr_taiko, mcsr_taiko)
                = ModernWithEntrySize::read_mod_combo_star_ratings(bytes, i)?;
            println!("{} | {}: {:?}", num, stringify!(num_mcsr_taiko), num_mcsr_taiko);
            let (num_mcsr_ctb, mcsr_ctb)
                = ModernWithEntrySize::read_mod_combo_star_ratings(bytes, i)?;
            println!("{} | {}: {:?}", num, stringify!(num_mcsr_ctb), num_mcsr_ctb);
            let (num_mcsr_mania, mcsr_mania)
                = ModernWithEntrySize::read_mod_combo_star_ratings(bytes, i)?;
            println!("{} | {}: {:?}", num, stringify!(num_mcsr_mania), num_mcsr_mania);
            let drain_time = read_int(bytes, i)?;
            println!("{} | {}: {}", num, stringify!(drain_time), drain_time);
            let total_time = read_int(bytes, i)?;
            println!("{} | {}: {}", num, stringify!(total_time), total_time);
            let preview_offset_from_start_ms = read_int(bytes, i)?;
            println!("{} | {}: {}", num, stringify!(preview_offset_from_start_ms), preview_offset_from_start_ms);
            let num_timing_points = read_int(bytes, i)?;
            println!("{} | {}: {}", num, stringify!(num_timing_points), num_timing_points);
            println!("{}: {} => {}?", num, i, *i + num_timing_points as usize * 17);
            let mut timing_points = Vec::with_capacity(num_timing_points as usize);
            for _ in 0..num_timing_points {
                timing_points.push(TimingPoint::read_from_bytes(bytes, i)?);
            }
            println!("{}: {}", num, i);
            let beatmap_id = read_int(bytes, i)?;
            println!("{} | {}: {}", num, stringify!(beatmap_id), beatmap_id);
            let beatmap_set_id = read_int(bytes, i)?;
            println!("{} | {}: {}", num, stringify!(beatmap_set_id), beatmap_set_id);
            let thread_id = read_int(bytes, i)?;
            println!("{} | {}: {}", num, stringify!(thread_id), thread_id);
            let standard_grade = read_byte(bytes, i)?;
            println!("{} | {}: {}", num, stringify!(standard_grade), standard_grade);
            let taiko_grade = read_byte(bytes, i)?;
            println!("{} | {}: {}", num, stringify!(taiko_grade), taiko_grade);
            let ctb_grade = read_byte(bytes, i)?;
            println!("{} | {}: {}", num, stringify!(ctb_grade), ctb_grade);
            let mania_grade = read_byte(bytes, i)?;
            println!("{} | {}: {}", num, stringify!(mania_grade), mania_grade);
            let local_offset = read_short(bytes, i)?;
            println!("{} | {}: {}", num, stringify!(local_offset), local_offset);
            let stack_leniency = read_single(bytes, i)?;
            println!("{} | {}: {}", num, stringify!(stack_leniency), stack_leniency);
            let gameplay_mode = GameplayMode::read_from_bytes(bytes, i)?;
            println!("{} | {}: {}", num, stringify!(gameplay_mode), gameplay_mode);
            let song_source = read_string_utf8(bytes, i, "song source")?;
            println!("{} | {}: {:?}", num, stringify!(song_source), song_source);
            let song_tags = read_string_utf8(bytes, i, "song tags")?;
            println!("{} | {}: {:?}", num, stringify!(song_tags), song_tags);
            let online_offset = read_short(bytes, i)?;
            println!("{} | {}: {}", num, stringify!(online_offset), online_offset);
            let font_used_for_song_title = read_string_utf8(bytes, i, "font used for song title")?;
            println!("{} | {}: {:?}", num, stringify!(font_used_for_song_title), font_used_for_song_title);
            let unplayed = read_boolean(bytes, i)?;
            println!("{} | {}: {}", num, stringify!(unplayed), unplayed);
            let last_played = read_datetime(bytes, i)?;
            println!("{} | {}: {:?}", num, stringify!(last_played), last_played);
            let is_osz2 = read_boolean(bytes, i)?;
            println!("{} | {}: {}", num, stringify!(is_osz2), is_osz2);
            let beatmap_folder_name = read_string_utf8(bytes, i, "folder name")?;
            println!("{} | {}: {:?}", num, stringify!(beatmap_folder_name), beatmap_folder_name);
            let last_checked_against_repo = read_datetime(bytes, i)?;
            println!("{} | {}: {:?}", num, stringify!(last_checked_against_repo), last_checked_against_repo);
            let ignore_beatmap_sound = read_boolean(bytes, i)?;
            println!("{} | {}: {}", num, stringify!(ignore_beatmap_sound), ignore_beatmap_sound);
            let ignore_beatmap_skin = read_boolean(bytes, i)?;
            println!("{} | {}: {}", num, stringify!(ignore_beatmap_skin), ignore_beatmap_skin);
            let disable_storyboard = read_boolean(bytes, i)?;
            println!("{} | {}: {}", num, stringify!(disable_storyboard), disable_storyboard);
            let disable_video = read_boolean(bytes, i)?;
            println!("{} | {}: {}", num, stringify!(disable_video), disable_video);
            let visual_override = read_boolean(bytes, i)?;
            println!("{} | {}: {}", num, stringify!(visual_override), visual_override);
            let unknown_short = ModernWithEntrySize::read_unknown_short(bytes, i)?;
            println!("{} | {}: {:?}", num, stringify!(unknown_short), unknown_short);
            let offset_from_song_start_in_editor_ms = read_int(bytes, i)?;
            println!("{} | {}: {}", num, stringify!(offset_from_song_start_in_editor_ms), offset_from_song_start_in_editor_ms);
            let mania_scroll_speed = read_byte(bytes, i)?;
            println!("{} | {}: {}", num, stringify!(mania_scroll_speed), mania_scroll_speed);
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
