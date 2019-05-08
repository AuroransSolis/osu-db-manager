use std::fs::File;
use std::io::{Result as IoResult, Error as IoError, ErrorKind::InvalidData, Read, Seek, SeekFrom};
use std::time::{Duration, SystemTime};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::string::FromUtf8Error;
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

impl TimingPoint {
    fn read_from_file(file: &mut File) -> IoResult<Self> {
        Ok(TimingPoint {
            bpm: read_double(file)?,
            offset: read_double(file)?,
            inherited: read_boolean(file)?
        })
    }
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

impl RankedStatus {
    pub fn read_from_file(file: &mut File) -> IoResult<Self> {
        let b = file.read_u8()?;
        match b {
            0 => Ok(Ok(Unknown)),
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
pub struct Pre20140609;

#[derive(Copy, Clone, Debug)]
pub struct Modern;

trait ReadVersionSpecificData {
    fn read_arcshpod(file: &mut File) -> IoResult<ByteSingle>;
    fn read_mod_combo_star_ratings(file: &mut File) -> IoResult<Option<(i32, Vec<(i32, f64)>)>>;
    fn read_unknown_short(file: &mut File) -> IoResult<Option<i16>>;
}

impl ReadVersionSpecificData for Pre20140609 {
    fn read_arcshpod(file: &mut File) -> IoResult<ByteSingle> {
        Ok(Byte(file.read_u8()?))
    }

    fn read_mod_combo_star_ratings(file: &mut File) -> IoResult<Option<(i32, Vec<(i32, f64)>)>> {
        Ok(None)
    }

    fn read_unknown_short(file: &mut File) -> IoResult<Option<i16>> {
        Ok(Some(read_short(file)?))
    }
}

impl ReadVersionSpecificData for Modern {
    fn read_arcshpod(file: &mut File) -> IoResult<ByteSingle> {
        Ok(Single(read_single(file)?))
    }

    fn read_mod_combo_star_ratings(file: &mut File) -> IoResult<Option<(i32, Vec<(i32, f64)>)>> {
        let num_int_doubles = read_int(file)?;
        let mut int_double_pairs = Vec::new();
        for _ in 0..num_int_doubles {
            int_double_pairs.push(read_int_double_pair(file)?);
        }
        Ok(Some((num_int_doubles, int_double_pairs)))
    }

    fn read_unknown_short(file: &mut File) -> IoResult<Option<i16>> {
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
    fn read_gameplay_mode(file: &mut File) -> IoResult<Self> {
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
    local_offset: i16,
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
    unknown_short: Option<i16>,
    maybe_last_modification_time: i32,
    mania_scroll_speed: u8
}

impl Beatmap {
    fn read_from_file<T: ReadVersionSpecificData>(file: &mut File, version: T) -> IoResult<Self> {
        let entry_size = read_int(file)?;
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
        let mut timing_points = Vec::new();
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
        let gameplay_mode = GameplayMode::read_gameplay_mode(file)?;
        let song_source = fromutf8_to_ioresult(read_string_utf8(file)?, "song source")?;
        let song_tags = fromutf8_to_ioresult(read_string_utf8(file)?, "song tags")?;
        let online_offset = read_short(file)?;
        let font_used_for_song_title = fromutf8_to_ioresult(read_string_utf8(file)?,
            "font used for song title")?;
        let unplayed = read_boolean(file)?;
        let last_played = read_long(file)?;
        let is_osz2 = read_boolean(file)?;
        let folder_name = fromutf8_to_ioresult(read_string_utf8(file)?, "folder name")?;
        let last_checked_against_repo = read_long(file)?;
        let ignore_beatmap_sound = read_boolean(file)?;
        let ignore_beatmap_skin = read_boolean(file)?;
        let disable_storyboard = read_boolean(file)?;
        let disable_video = read_boolean(file)?;
        let visual_override = read_boolean(file)?;
        let unknown_short = T::read_unknown_short(file)?;
        let maybe_last_modification_time = read_int(file)?;
        let mania_scroll_speed = file.read_u8()?;
        Ok(Beatmap {

        })
    }
}

fn fromutf8_to_ioresult(r: Result<String, FromUtf8Error>, field: &str) -> IoResult<String> {
    if let Ok(string) = r {
        Ok(string)
    } else if let Err(e) = r {
        let err_msg = format!("Error reading {} ({})", field, e);
        Err(IoError::new(InvalidData, err_msg.as_str()))
    }
}