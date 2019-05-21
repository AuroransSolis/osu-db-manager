use std::fs::File;
use std::io::{Result as IoResult, Error as IoError, ErrorKind, Cursor, Seek, SeekFrom, Read};
use std::time::{Duration, SystemTime};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::sync::{Arc, Mutex, atomic::{AtomicUsize, AtomicU64, Ordering}};
use std::thread::{self, JoinHandle};
use byteorder::ReadBytesExt;
use crate::deserialize_primitives::*;
use crate::databases::load::{Load, LoadSettings};
use crate::clone_file;

// Deserializing osu!.db-specific data types

fn read_int_double_pair(file: &mut File) -> IoResult<(i32, f64)> {
    let _ = file.read_u8()?;
    let int = read_int(file)?;
    let _ = file.read_u8()?;
    let double = read_double(file)?;
    Ok((int, double))
}

#[derive(Copy, Clone, Debug)]
pub struct TimingPoint {
    bpm: f64,
    offset: f64,
    inherited: bool
}

impl TimingPoint {
    fn read_from_file(file: &mut File) -> IoResult<Self> {
        Ok(TimingPoint {
            bpm: read_double(file)?,
            offset: read_double(file)?,
            inherited: read_boolean(file)?
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
    pub fn read_from_file(file: &mut File) -> IoResult<Self> {
        let b = file.read_u8()?;
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
    fn read_entry_size(file: &mut File) -> IoResult<Option<i32>>;
    fn read_arcshpod(file: &mut File) -> IoResult<ByteSingle>;
    fn read_mod_combo_star_ratings(file: &mut File)
        -> IoResult<(Option<i32>, Option<Vec<(i32, f64)>>)>;
    fn read_unknown_short(file: &mut File) -> IoResult<Option<i16>>;
}

impl ReadVersionSpecificData for Legacy {
    fn read_entry_size(_file: &mut File) -> IoResult<Option<i32>> {
        Ok(None)
    }

    fn read_arcshpod(file: &mut File) -> IoResult<ByteSingle> {
        Ok(Byte(file.read_u8()?))
    }

    fn read_mod_combo_star_ratings(_file: &mut File)
        -> IoResult<(Option<i32>, Option<Vec<(i32, f64)>>)> {
        Ok((None, None))
    }

    fn read_unknown_short(file: &mut File) -> IoResult<Option<i16>> {
        Ok(Some(read_short(file)?))
    }
}

impl ReadVersionSpecificData for Modern {
    fn read_entry_size(_file: &mut File) -> IoResult<Option<i32>> {
        Ok(None)
    }

    fn read_arcshpod(file: &mut File) -> IoResult<ByteSingle> {
        Ok(Single(read_single(file)?))
    }

    fn read_mod_combo_star_ratings(file: &mut File)
        -> IoResult<(Option<i32>, Option<Vec<(i32, f64)>>)> {
        let num_int_doubles = read_int(file)?;
        let mut int_double_pairs = Vec::new();
        for _ in 0..num_int_doubles {
            int_double_pairs.push(read_int_double_pair(file)?);
        }
        Ok((Some(num_int_doubles), Some(int_double_pairs)))
    }

    fn read_unknown_short(_file: &mut File) -> IoResult<Option<i16>> {
        Ok(None)
    }
}

impl ReadVersionSpecificData for ModernWithEntrySize {
    fn read_entry_size(file: &mut File) -> IoResult<Option<i32>> {
        Ok(Some(read_int(file)?))
    }

    fn read_arcshpod(file: &mut File) -> IoResult<ByteSingle> {
        Ok(Single(read_single(file)?))
    }

    fn read_mod_combo_star_ratings(file: &mut File)
        -> IoResult<(Option<i32>, Option<Vec<(i32, f64)>>)> {
        let num_int_doubles = read_int(file)?;
        let mut int_double_pairs = Vec::new();
        for _ in 0..num_int_doubles {
            int_double_pairs.push(read_int_double_pair(file)?);
        }
        Ok((Some(num_int_doubles), Some(int_double_pairs)))
    }

    fn read_unknown_short(_file: &mut File) -> IoResult<Option<i16>> {
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
    pub fn read_from_file(file: &mut File) -> IoResult<Self> {
        let b = file.read_u8()?;
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
    fn read_from_file<T: ReadVersionSpecificData>(file: &mut File) -> IoResult<Self> {
        let entry_size = T::read_entry_size(file)?;
        let artist_name = fromutf8_to_ioresult(read_string_utf8(file)?, "non-Unicode artist name")?;
        let artist_name_unicode = fromutf8_to_ioresult(read_string_utf8(file)?,
            "Unicode artist name")?;
        let song_title = fromutf8_to_ioresult(read_string_utf8(file)?, "non-Unicode song title")?;
        let song_title_unicode = fromutf8_to_ioresult(read_string_utf8(file)?,
            "Unicode song title")?;
        let creator_name = fromutf8_to_ioresult(read_string_utf8(file)?, "creator name")?;
        let difficulty = fromutf8_to_ioresult(read_string_utf8(file)?, "difficulty")?;
        let audio_file_name = fromutf8_to_ioresult(read_string_utf8(file)?, "audio file name")?;
        let md5_beatmap_hash = fromutf8_to_ioresult(read_string_utf8(file)?, "MD5 beatmap hash")?;
        let dotosu_file_name = fromutf8_to_ioresult(read_string_utf8(file)?,
            "corresponding .osu file name")?;
        let ranked_status = RankedStatus::read_from_file(file)?;
        let number_of_hitcircles = read_short(file)?;
        let number_of_sliders = read_short(file)?;
        let number_of_spinners = read_short(file)?;
        let last_modification_time = read_datetime(file)?;
        let approach_rate = T::read_arcshpod(file)?;
        let circle_size = T::read_arcshpod(file)?;
        let hp_drain = T::read_arcshpod(file)?;
        let overall_difficulty = T::read_arcshpod(file)?;
        let slider_velocity = read_double(file)?;
        let (num_mcsr_standard, mcsr_standard) = T::read_mod_combo_star_ratings(file)?;
        let (num_mcsr_taiko, mcsr_taiko) = T::read_mod_combo_star_ratings(file)?;
        let (num_mcsr_ctb, mcsr_ctb) = T::read_mod_combo_star_ratings(file)?;
        let (num_mcsr_mania, mcsr_mania) = T::read_mod_combo_star_ratings(file)?;
        let drain_time = read_int(file)?;
        let total_time = read_int(file)?;
        let preview_offset_from_start_ms = read_int(file)?;
        let num_timing_points = read_int(file)?;
        let mut timing_points = Vec::with_capacity(num_timing_points as usize);
        for _ in 0..num_timing_points {
            timing_points.push(TimingPoint::read_from_file(file)?);
        }
        let beatmap_id = read_int(file)?;
        let beatmap_set_id = read_int(file)?;
        let thread_id = read_int(file)?;
        let standard_grade = file.read_u8()?;
        let taiko_grade = file.read_u8()?;
        let ctb_grade = file.read_u8()?;
        let mania_grade = file.read_u8()?;
        let local_offset = read_short(file)?;
        let stack_leniency = read_single(file)?;
        let gameplay_mode = GameplayMode::read_from_file(file)?;
        let song_source = fromutf8_to_ioresult(read_string_utf8(file)?, "song source")?;
        let song_tags = fromutf8_to_ioresult(read_string_utf8(file)?, "song tags")?;
        let online_offset = read_short(file)?;
        let font_used_for_song_title = fromutf8_to_ioresult(read_string_utf8(file)?,
            "font used for song title")?;
        let unplayed = read_boolean(file)?;
        let last_played = read_datetime(file)?;
        let is_osz2 = read_boolean(file)?;
        let beatmap_folder_name = fromutf8_to_ioresult(read_string_utf8(file)?, "folder name")?;
        let last_checked_against_repo = read_datetime(file)?;
        let ignore_beatmap_sound = read_boolean(file)?;
        let ignore_beatmap_skin = read_boolean(file)?;
        let disable_storyboard = read_boolean(file)?;
        let disable_video = read_boolean(file)?;
        let visual_override = read_boolean(file)?;
        let unknown_short = T::read_unknown_short(file)?;
        let offset_from_song_start_in_editor_ms = read_int(file)?;
        let mania_scroll_speed = file.read_u8()?;
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

#[derive(Copy, Clone)]
pub struct BeatmapLoadSettings {
    pub chunks: usize // how many beatmaps to load at a time
}

impl LoadSettings for BeatmapLoadSettings {}

#[derive(Copy, Clone)]
pub struct OsuDbLoadSettings {
    pub jobs: usize,
    pub beatmap_settings: BeatmapLoadSettings
}

impl LoadSettings for OsuDbLoadSettings {}

impl Load for OsuDb {
    fn read_from_file(settings: OsuDbLoadSettings, mut file: File) -> IoResult<Self> {
        if settings.jobs == 1 {
            Self::read_single_thread(settings, file)
        } else {
            Self::read_multi_thread(settings, file)
        }
    }

    fn read_single_thread(settings: OsuDbLoadSettings, mut file: File) -> IoResult<Self> {
        let version = read_int(&mut file)?;
        let folder_count = read_int(&mut file)?;
        let account_unlocked = read_boolean(&mut file)?;
        let account_unlock_date = if !account_unlocked {
            Some(read_datetime(&mut file)?)
        } else {
            let _ = read_long(&mut file)?;
            None
        };
        let player_name = fromutf8_to_ioresult(read_string_utf8(&mut file)?, "player name")?;
        let num_beatmaps = read_int(&mut file)?;
        let mut beatmaps = Vec::with_capacity(num_beatmaps as usize);
        if version < 20140609 {
            for _ in 0..num_beatmaps {
                beatmaps.push(Beatmap::read_from_file::<Legacy>(&mut file)?);
            }
        } else if version >= 20140609 && version < 20160408 {
            for _ in 0..num_beatmaps {
                beatmaps.push(Beatmap::read_from_file::<Modern>(&mut file)?);
            }
        } else if version >= 20160408 {
            for _ in 0..num_beatmaps {
                beatmaps.push(Beatmap::read_from_file::<ModernWithEntrySize>(&mut file)?);
            }
        } else {
            let err_msg = format!("Read version with no associated beatmap loading method {}",
                version);
            return Err(IoError::new(ErrorKind::InvalidData, err_msg.as_str()));
        }
        let unknown_int = read_int(&mut file)?;
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

    fn read_multi_thread(settings: OsuDbLoadSettings, mut file: File) -> IoResult<Self> {
        let version = read_int(&mut file)?;
        let folder_count = read_int(&mut file)?;
        let account_unlocked = read_boolean(&mut file)?;
        let account_unlock_date = if !account_unlocked {
            Some(read_datetime(&mut file)?)
        } else {
            let _ = read_long(&mut file)?;
            None
        };
        let player_name = fromutf8_to_ioresult(read_string_utf8(&mut file)?, "player name")?;
        let num_beatmaps = read_int(&mut file)?;
        if version >= 20160408 {
            let seek_locations = fetch_seek_locations(clone_file(&file))?;

            for _ in 0..num_beatmaps {
                beatmaps.push(Beatmap::read_from_file::<ModernWithEntrySize>(file)?);
            }
        } else if version / 1000 <= 2016 && version / 1000 >= 2007 { // catch valid versions
            return Err(IoError::new(ErrorKind::Other, "osu!.db versions older than 20160408 do not \
                support multithreaded loading."));
        } else {
            let err_msg = format!("Read version with no associated beatmap loading method: {}",
                version);
            return Err(IoError::new(ErrorKind::InvalidData, err_msg.as_str()));
        }
        let unknown_int = read_int(file)?;
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

fn spawn_beatmap_loader_seek_locations(mut file: File, entry_lengths: Arc<Mutex<Vec<usize>>>)
    -> JoinHandle<IoResult<Vec<(usize, Beatmap)>>> {
    let mut current_position = 0;
    let mut maps = Vec::new();
    thread::spawn(move || {
        let beatmap_number = {
            let cnt = counter.get_mut();
            let current = cnt;
            if current == amt {
                return maps;
            }
            *cnt += 1;
            current
        };
        let (offset_from_start, entry_size) = {
            let atomic_current_offset = start_offset.get_mut();
            let ret = *atomic_current_offset;
            let entry_size = read_int(&mut file)?;
            *atomic_current_offset += entry_size + 4;
            (ret, entry_size)
        };
        let current_position = file.seek(SeekFrom::Current((offset_from_start as u64
            - current_position) as i64))?;
    })
}

fn fetch_seek_locations(mut file: File) -> IoResult<Vec<usize>> {
    let _ = file.seek(SeekFrom::Start(17))?; // go to start and skip the first four fields
    let player_name_len = read_uleb128(&mut file)?;
    let _ = file.seek(SeekFrom::Current(player_name_len as i64))?; // skip over player name
    let number_of_beatmaps = read_int(&mut file)?;
    let seek_from_start = 17 + player_name_len + 4;
    let mut seek_locations = Vec::with_capacity(number_of_beatmaps as usize);
    seek_locations.push(file.seek(SeekFrom::Current(0))? as usize);
    for _ in 0..number_of_beatmaps {
        let entry_length = read_int(&mut file)? as usize;
        seek_locations.push(file.seek(SeekFrom::Current(entry_length as i64))? as usize
            + seek_from_start);
    }
    Ok(entry_lengths)
}