use std::fs::File;
use std::io::{Result as IoResult, Error as IoError, ErrorKind::InvalidData, Cursor};
use std::time::{Duration, SystemTime};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::sync::{Arc, Mutex, atomic::{AtomicUsize, Ordering}};
use byteorder::ReadBytesExt;
use crate::deserialize_primitives::*;
use crate::databases::load::{Load, LoadSettings};

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
                Err(IoError::new(InvalidData, err_msg.as_str()))
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
pub struct Pre20140609;

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

impl ReadVersionSpecificData for Pre20140609 {
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
                Err(IoError::new(InvalidData, err_msg.as_str()))
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
    }
}

impl Load for Beatmap {
    fn read_from_file(settings: BeatmapLoadSettings, stream: &mut File) -> IoResult<Self> {
        if settings.jobs == 1 {
            Self::read_single_thread(settings, stream)
        } else {
            Self::read_multi_thread(settings, stream)
        }
    }

    fn read_single_thread(settings: BeatmapLoadSettings, stream: Arc<Mutex>)
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
    pub jobs: usize,
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
    fn read_from_file(settings: OsuDbLoadSettings, file: &mut File) -> IoResult<Self> {
        if settings.jobs == 1 {
            Self::read_single_thread(settings, file)
        } else {
            Self::read_multi_thread(settings, file)
        }
    }

    fn read_single_thread(settings: OsuDbLoadSettings, file: &mut File) -> IoResult<Self> {
        let version = read_int(file)?;
        let folder_count = read_int(file)?;
        let account_unlocked = read_boolean(file)?;
        let account_unlock_date = if !account_unlocked {
            Some(read_datetime(file)?)
        } else {
            let _ = read_long(file)?;
            None
        };
        let player_name = fromutf8_to_ioresult(read_string_utf8(file)?, "player name")?;
        let num_beatmaps = read_int(file)?;
        let mut beatmaps = Vec::with_capacity(num_beatmaps as usize);
        if version < 20140609 {
            for _ in 0..num_beatmaps {
                beatmaps.push(Beatmap::read_from_file::<Pre20140609>(file)?);
            }
        } else if version >= 20140609 && version < 20160408 {
            for _ in 0..num_beatmaps {
                beatmaps.push(Beatmap::read_from_file::<Modern>(file)?);
            }
        } else if version >= 20160408 {
            for _ in 0..num_beatmaps {
                beatmaps.push(Beatmap::read_from_file::<ModernWithEntrySize>(file)?);
            }
        } else {
            let err_msg = format!("Read version with no associated beatmap loading method {}",
                version);
            return Err(IoError::new(InvalidData, err_msg.as_str()));
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

    fn read_multi_thread(settings: OsuDbLoadSettings, file: &mut File) -> IoResult<Self> {
        let version = read_int(file)?;
        let folder_count = read_int(file)?;
        let account_unlocked = read_boolean(file)?;
        let account_unlock_date = if !account_unlocked {
            Some(read_datetime(file)?)
        } else {
            let _ = read_long(file)?;
            None
        };
        let player_name = fromutf8_to_ioresult(read_string_utf8(file)?, "player name")?;
        let num_beatmaps = read_int(file)?;
        let mut bytes = Vec::new();
        while let Ok(byte) = file.read_u8() {
            bytes.push(byte);
        }
        let mut beatmap_number = AtomicUsize::new(num_beatmaps as usize);
        let mut arc_cursor = Arc::new(Mutex::new(Cursor::new(bytes)));
        let mut current_start = AtomicUsize::new(0);
        let mut beatmaps = Vec::with_capacity(num_beatmaps as usize);
        if version < 20140609 {
            for _ in 0..num_beatmaps {
                beatmaps.push(Beatmap::read_from_file::<Pre20140609>(file)?);
            }
        } else if version >= 20140609 && version < 20160408 {
            for _ in 0..num_beatmaps {
                beatmaps.push(Beatmap::read_from_file::<Modern>(file)?);
            }
        } else if version >= 20160408 {
            for _ in 0..num_beatmaps {
                beatmaps.push(Beatmap::read_from_file::<ModernWithEntrySize>(file)?);
            }
        } else {
            let err_msg = format!("Read version with no associated beatmap loading method {}",
                version);
            return Err(IoError::new(InvalidData, err_msg.as_str()));
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