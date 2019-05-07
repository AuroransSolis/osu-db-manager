use std::fs::File;
use std::io::{Result as IoResult, Error as IoError, ErrorKind::InvalidData};
use std::time::{Duration, SystemTime};
use std::fmt::{Display, Formatter, Result as FmtResult};
use byteorder::LittleEndian;
use deserialize_primitives::*;

// Deserializing osu!.db-specific data types

fn read_int_double_pair(file: &mut File) -> IoResult<(i32, f64)> {
    Ok((read_int(file)?, read_double(file)?))
}

struct TimingPoint {
    bpm: f64,
    offset: f64,
    inherited: bool
}

fn read_timing_point(file: &mut File) -> IoResult<TimingPoint> {
    Ok(TimingPoint {
        bpm: read_double(file)?,
        offset: read_double(file)?,
        inherited: read_boolean(file)?
    })
}

fn read_datetime(file: &mut File) -> IoResult<SystemTime> {
    let ticks = read_long(file)?;
    let duration_since_epoch = Duration::from_nanos(ticks as u64 * 100);
    Ok(SystemTime::UNIX_EPOCH.add(duration_since_epoch))
}

#[derive(Copy, Clone, Debug)]
enum RankedStatus {
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
enum ByteSingle {
    Byte(u8),
    Single(f32)
}

use self::ByteSingle::*;

impl Display for ByteSingle {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", match self {
            Byte(&b) => b,
            Single(&s) => s
        })
    }
}

#[derive(Copy, Clone, Debug)]
enum GameplayMode {
    Standard,
    Taiko,
    Ctb,
    Mania
}

use self::GameplayMode::*;

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

pub struct Beatmap {
    entry_size: i32,
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
    local_beatmap_offset: i16,
    stack_leniency: f32,
    gameplay_mode: GameplayMode,
    song_source: String,
    song_tags: String,
    online_offset: i16,
    font_used_for_song_title: String,
    unplayed: bool,
    last_played: SystemTime,
    ignore_beatmap_sound: bool,
    ignore_beatmap_skin: bool,
    disable_storyboard: bool,
    disable_video: bool,
    visual_override: bool,
    maybe_version_indicator: i16,
    maybe_last_modification_time: i32,
    mania_scroll_speed: u8
}

impl Beatmap {
    fn read_from_file(file: &mut File) -> IoResult<Self> {
        let entry_size = read_int(file)?;
        let artist_name = read_string_utf8(file)?;
        let artist_name = if let Ok(string) = artist_name {
            string
        } else if let Err(e) = artist_name {
            let err_msg = format!("Error reading non-Unicode artist name ({})", e);
            return Err(IoError::new(InvalidData, err_msg.as_str()));
        };
        let artist_name_unicode = read_string_utf8(file)?;
        let artist_name_unicode = if let Ok(string) = artist_name_unicode {
            string
        } else if let Err(e) = artist_name_unicode {
            let err_msg = format!("Error reading Unicode artist name ({})", e);
            return Err(IoError::new(InvalidData, err_msg.as_str()));
        };
        let song_title = read_string_utf8(file)?;
        let song_title = if let Ok(string) = song_title {
            string
        } else if let Err(e) = song_title {
            let err_msg = format!("Error reading non-Unicode artist name ({})", e);
            return Err(IoError::new(InvalidData, err_msg.as_str()));
        };
        let song_title_unicode = read_string_utf8(file)?;
        let song_title_unicode = if let Ok(string) = song_title_unicode {
            string
        } else if let Err(e) = song_title_unicode {
            let err_msg = format!("Error reading Unicode artist name ({})", e);
            return Err(IoError::new(InvalidData, err_msg.as_str()));
        };

    }
}