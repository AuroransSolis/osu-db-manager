use std::fs::File;
use std::io::{Result as IoResult, Error as IoError, ErrorKind::InvalidData};
use std::time::{Duration, SystemTime};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::string::FromUtf8Error;
use std::mem::size_of_val;
use byteorder::ReadBytesExt;
use crate::deserialize_primitives::*;

const BOOLEAN: usize = 1;
const BYTE: usize = 1;
const SHORT: usize = 2;
const INT: usize = 4;
const LONG: usize = 8;
const SINGLE: usize = 4;
const DOUBLE: usize = 8;
const INT_DOUBLE_PAIR: usize = 14;
const TIMING_POINT: usize = 17;
const DATETIME: usize = 8;

// Deserializing osu!.db-specific data types

fn read_int_double_pair(file: &mut File) -> IoResult<(i32, f64)> {
    let _ = file.read_u8()?;
    let int = read_int(file)?;
    let _ = file.read_u8()?;
    let double = read_double(file)?;
    Ok((int, double))
}

#[derive(Copy, Clone, Debug)]
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
    let duration_since_epoch = Duration::from_micros(ticks as u64 / 10);
    Ok(SystemTime::UNIX_EPOCH + duration_since_epoch)
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
enum ByteSingle {
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
    fn check_entry_size(beatmap: &Beatmap) -> Option<bool>;
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

    fn check_entry_size(_beatmap: &Beatmap) -> Option<bool> {
        None
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

    fn check_entry_size(_beatmap: &Beatmap) -> Option<bool> {
        None
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

    fn check_entry_size(beatmap: &Beatmap) -> Option<bool> {
        let mut total_size = beatmap.artist_name.as_bytes().len()
            + beatmap.artist_name_unicode.as_bytes().len()
            + beatmap.song_title.as_bytes().len()
            + beatmap.song_title_unicode.as_bytes().len()
            + beatmap.creator_name.as_bytes().len()
            + beatmap.difficulty.as_bytes().len()
            + beatmap.audio_file_name.as_bytes().len()
            + beatmap.md5_beatmap_hash.as_bytes().len()
            + beatmap.dotosu_file_name.as_bytes().len()
            + BYTE // beatmap.ranked_status
            + SHORT // beatmap.number_of_hitcircles
            + SHORT // beatmap.number_of_sliders
            + SHORT // beatmap.number_of_spinners
            + DATETIME // beatmap.last_modification_time
            + SINGLE // beatmap.approach_rate
            + SINGLE // beatmap.circle_size
            + SINGLE // beatmap.hp_drain
            + SINGLE // beatmap.overall_difficulty
            + DOUBLE // beatmap.slider_velocity
            + INT_DOUBLE_PAIR // beatmap.num_mod_combo_star_ratings_standard
            + beatmap.mod_combo_star_ratings_standard.as_ref().unwrap().len() * INT_DOUBLE_PAIR
            + INT_DOUBLE_PAIR // beatmap.num_mod_combo_star_ratings_taiko
            + beatmap.mod_combo_star_ratings_taiko.as_ref().unwrap().len() * INT_DOUBLE_PAIR
            + INT_DOUBLE_PAIR // beatmap.num_mod_combo_star_ratings_ctb
            + beatmap.mod_combo_star_ratings_ctb.as_ref().unwrap().len() * INT_DOUBLE_PAIR
            + INT_DOUBLE_PAIR // beatmap.num_mod_combo_star_ratings_mania
            + beatmap.mod_combo_star_ratings_mania.as_ref().unwrap().len() * INT_DOUBLE_PAIR
            + INT // beatmap.drain_time
            + INT // beatmap.total_time
            + INT // beatmap.preview_offset_from_start_ms
            + INT // beatmap.num_timing_points
            + beatmap.timing_points.len() * TIMING_POINT
            + INT // beatmap.beatmap_id
            + INT // beatmap.beatmap_set_id
            + INT // beatmap.thread_id
            + BYTE // beatmap.standard_grade
            + BYTE // beatmap.taiko_grade
            + BYTE // beatmap.ctb_grade
            + BYTE // beatmap.mania_grade
            + SHORT // beatmap.local_offset
            + SINGLE // beatmap.stack_leniency
            + BYTE // beatmap.gameplay_mode
            + beatmap.song_source.as_bytes().len()
            + beatmap.song_tags.as_bytes().len()
            + SHORT // beatmap.online_offset
            + beatmap.font_used_for_song_title.as_bytes().len()
            + BOOLEAN // beatmap.unplayed
            + LONG // beatmap.last_played
            + BOOLEAN // beatmap.is_osz2
            + beatmap.beatmap_folder_name.as_bytes().len()
            + LONG // beatmap.last_checked_against_repo
            + BOOLEAN // beatmap.ignore_beatmap_sound
            + BOOLEAN // beatmap.ignore_beatmap_skin
            + BOOLEAN // beatmap.disable_storyboard
            + BOOLEAN // beatmap.disable_video
            + BOOLEAN //beatmap.visual_override
            + INT // beatmap.offset_from_song_start_in_editor_ms
            + BYTE; // beatmap.mania_scroll_speed
        println!("    Comparing {} (size of loaded beatmap) and {} (specified entry size)", total_size, beatmap.entry_size.unwrap());
        Some(total_size == beatmap.entry_size.unwrap() as usize)
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
        println!("    Loading entry_size...");
        let entry_size = T::read_entry_size(file)?;
        println!("        ...loaded {:?}", entry_size);
        println!("    Loading artist_name...");
        let artist_name = fromutf8_to_ioresult(read_string_utf8(file)?, "non-Unicode artist name")?;
        println!("        ...loaded {}", artist_name);
        println!("    Loading artist_name_unicode...");
        let artist_name_unicode = fromutf8_to_ioresult(read_string_utf8(file)?,
            "Unicode artist name")?;
        println!("        ...loaded {}", artist_name_unicode);
        println!("    Loading song_title...");
        let song_title = fromutf8_to_ioresult(read_string_utf8(file)?, "non-Unicode song title")?;
        println!("        ...loaded {}", song_title);
        let song_title_unicode = fromutf8_to_ioresult(read_string_utf8(file)?,
            "Unicode song title")?;
        println!("    Loading creator_name...");
        let creator_name = fromutf8_to_ioresult(read_string_utf8(file)?, "creator name")?;
        println!("        ...loaded {}", creator_name);
        println!("    Loading difficulty...");
        let difficulty = fromutf8_to_ioresult(read_string_utf8(file)?, "difficulty")?;
        println!("        ...loaded {}", difficulty);
        println!("    Loading audio_file_name...");
        let audio_file_name = fromutf8_to_ioresult(read_string_utf8(file)?, "audio file name")?;
        println!("        ...loaded {}", audio_file_name);
        println!("    Loading md5_beatmap_hash...");
        let md5_beatmap_hash = fromutf8_to_ioresult(read_string_utf8(file)?, "MD5 beatmap hash")?;
        println!("        ...loaded {}", md5_beatmap_hash);
        println!("    Loading dotosu_file_name...");
        let dotosu_file_name = fromutf8_to_ioresult(read_string_utf8(file)?,
            "corresponding .osu file name")?;
        println!("        ...loaded {}", dotosu_file_name);
        println!("    Loading ranked_status...");
        let ranked_status = RankedStatus::read_from_file(file)?;
        println!("        ...loaded {}", ranked_status);
        println!("    Loading number_of_hitcircles...");
        let number_of_hitcircles = read_short(file)?;
        println!("        ...loaded {}", number_of_hitcircles);
        println!("    Loading number_of_sliders...");
        let number_of_sliders = read_short(file)?;
        println!("        ...loaded {}", number_of_sliders);
        println!("    Loading number_of_spinners...");
        let number_of_spinners = read_short(file)?;
        println!("        ...loaded {}", number_of_spinners);
        println!("    Loading last_modification_time...");
        let last_modification_time = read_datetime(file)?;
        println!("        ...loaded {:?}", last_modification_time);
        println!("    Loading approach_rate...");
        let approach_rate = T::read_arcshpod(file)?;
        println!("        ...loaded {}", approach_rate);
        println!("    Loading circle_size...");
        let circle_size = T::read_arcshpod(file)?;
        println!("        ...loaded {}", circle_size);
        println!("    Loading hp_drain...");
        let hp_drain = T::read_arcshpod(file)?;
        println!("        ...loaded {}", hp_drain);
        println!("    Loading overall_difficulty...");
        let overall_difficulty = T::read_arcshpod(file)?;
        println!("        ...loaded {}", overall_difficulty);
        println!("    Loading slider_velocity...");
        let slider_velocity = read_double(file)?;
        println!("        ...loaded {}", slider_velocity);
        println!("    Loading (num_mcsr_standard, mcsr_standard)...");
        let (num_mcsr_standard, mcsr_standard) = T::read_mod_combo_star_ratings(file)?;
        println!("        ...loaded {:?}", num_mcsr_standard);
        println!("    Loading (num_mcsr_taiko, mcsr_taiko)...");
        let (num_mcsr_taiko, mcsr_taiko) = T::read_mod_combo_star_ratings(file)?;
        println!("        ...loaded {:?}", num_mcsr_taiko);
        println!("    Loading (num_mcsr_ctb, mcsr_ctb)...");
        let (num_mcsr_ctb, mcsr_ctb) = T::read_mod_combo_star_ratings(file)?;
        println!("        ...loaded {:?}", num_mcsr_ctb);
        println!("    Loading (num_mcsr_mania, mcsr_mania)...");
        let (num_mcsr_mania, mcsr_mania) = T::read_mod_combo_star_ratings(file)?;
        println!("        ...loaded {:?}", num_mcsr_mania);
        println!("    Loading drain_time...");
        let drain_time = read_int(file)?;
        println!("        ...loaded {}", drain_time);
        println!("    Loading total_time...");
        let total_time = read_int(file)?;
        println!("        ...loaded {}", total_time);
        println!("    Loading preview_offset_from_start_ms...");
        let preview_offset_from_start_ms = read_int(file)?;
        println!("        ...loaded {}", preview_offset_from_start_ms);
        println!("    Loading num_timing_points...");
        let num_timing_points = read_int(file)?;
        println!("        ...loaded {}", num_timing_points);
        println!("    Loading timing_points...");
        let mut timing_points = Vec::with_capacity(num_timing_points as usize);
        for _ in 0..num_timing_points {
            timing_points.push(TimingPoint::read_from_file(file)?);
        }
        println!("        ...loaded.");
        println!("    Loading beatmap_id...");
        let beatmap_id = read_int(file)?;
        println!("        ...loaded {}", beatmap_id);
        println!("    Loading beatmap_set_id...");
        let beatmap_set_id = read_int(file)?;
        println!("        ...loaded {}", beatmap_set_id);
        println!("    Loading thread_id...");
        let thread_id = read_int(file)?;
        println!("        ...loaded {}", thread_id);
        println!("    Loading standard_grade...");
        let standard_grade = file.read_u8()?;
        println!("        ...loaded {}", standard_grade);
        println!("    Loading taiko_grade...");
        let taiko_grade = file.read_u8()?;
        println!("        ...loaded {}", taiko_grade);
        println!("    Loading ctb_grade...");
        let ctb_grade = file.read_u8()?;
        println!("        ...loaded {}", ctb_grade);
        println!("    Loading mania_grade...");
        let mania_grade = file.read_u8()?;
        println!("        ...loaded {}", mania_grade);
        println!("    Loading local_offset...");
        let local_offset = read_short(file)?;
        println!("        ...loaded {}", local_offset);
        println!("    Loading stack_leniency...");
        let stack_leniency = read_single(file)?;
        println!("        ...loaded {}", stack_leniency);
        println!("    Loading gameplay_mode...");
        let gameplay_mode = GameplayMode::read_gameplay_mode(file)?;
        println!("        ...loaded {}", gameplay_mode);
        println!("    Loading song_source...");
        let song_source = fromutf8_to_ioresult(read_string_utf8(file)?, "song source")?;
        println!("        ...loaded {}", song_source);
        println!("    Loading song_tags...");
        let song_tags = fromutf8_to_ioresult(read_string_utf8(file)?, "song tags")?;
        println!("        ...loaded {}", song_tags);
        println!("    Loading online_offset...");
        let online_offset = read_short(file)?;
        println!("        ...loaded {}", online_offset);
        println!("    Loading font_used_for_song_title...");
        let font_used_for_song_title = fromutf8_to_ioresult(read_string_utf8(file)?,
            "font used for song title")?;
        println!("        ...loaded {}", font_used_for_song_title);
        println!("    Loading unplayed...");
        let unplayed = read_boolean(file)?;
        println!("        ...loaded {}", unplayed);
        println!("    Loading last_played...");
        let last_played = read_datetime(file)?;
        println!("        ...loaded {:?}", last_played);
        println!("    Loading is_osz2...");
        let is_osz2 = read_boolean(file)?;
        println!("        ...loaded {}", is_osz2);
        println!("    Loading beatmap_folder_name...");
        let beatmap_folder_name = fromutf8_to_ioresult(read_string_utf8(file)?, "folder name")?;
        println!("        ...loaded {}", beatmap_folder_name);
        println!("    Loading last_checked_against_repo...");
        let last_checked_against_repo = read_datetime(file)?;
        println!("        ...loaded {:?}", last_checked_against_repo);
        println!("    Loading ignore_beatmap_sound...");
        let ignore_beatmap_sound = read_boolean(file)?;
        println!("        ...loaded {}", ignore_beatmap_sound);
        println!("    Loading ignore_beatmap_skin...");
        let ignore_beatmap_skin = read_boolean(file)?;
        println!("        ...loaded {}", ignore_beatmap_skin);
        println!("    Loading disable_storyboard...");
        let disable_storyboard = read_boolean(file)?;
        println!("        ...loaded {}", disable_storyboard);
        println!("    Loading disable_video...");
        let disable_video = read_boolean(file)?;
        println!("        ...loaded {}", disable_video);
        println!("    Loading visual_override...");
        let visual_override = read_boolean(file)?;
        println!("        ...loaded {}", visual_override);
        println!("    Loading unknown_short...");
        let unknown_short = T::read_unknown_short(file)?;
        println!("        ...loaded {:?}", unknown_short);
        println!("    Loading offset_from_song_start_in_editor_ms...");
        let offset_from_song_start_in_editor_ms = read_int(file)?;
        println!("        ...loaded {}", offset_from_song_start_in_editor_ms);
        println!("    Loading mania_scroll_speed...");
        let mania_scroll_speed = file.read_u8()?;
        println!("        ...loaded {}", mania_scroll_speed);
        let beatmap = Beatmap {
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
        };
        if let Some(false) = T::check_entry_size(&beatmap) {
            Err(IoError::new(InvalidData, "Entry size and size of beatmap do not match!"))
        } else {
            Ok(beatmap)
        }
    }
}

fn fromutf8_to_ioresult(r: Result<String, FromUtf8Error>, field: &str) -> IoResult<String> {
    match r {
        Ok(string) => Ok(string),
        Err(e) => {
            let err_msg = format!("Error reading {} ({})", field, e);
            Err(IoError::new(InvalidData, err_msg.as_str()))
        }
    }
}

#[derive(Debug, Clone)]
pub struct OsuDb {
    version: i32,
    folder_count: i32,
    account_unlocked: bool,
    account_unlock_date: Option<SystemTime>,
    player_name: String,
    number_of_beatmaps: i32,
    beatmaps: Vec<Beatmap>,
    unknown_int: i32
}

impl OsuDb {
    pub fn read_from_file(file: &mut File) -> IoResult<Self> {
        println!("Loading version...");
        let version = read_int(file)?;
        println!("    ...loaded {}", version);
        println!("Loading folder_count...");
        let folder_count = read_int(file)?;
        println!("    ...loaded {}", folder_count);
        println!("Loading account_unlocked...");
        let account_unlocked = read_boolean(file)?;
        println!("    ...loaded {}", account_unlocked);
        println!("Loading account_unlock_date...");
        let account_unlock_date = if !account_unlocked {
            Some(read_datetime(file)?)
        } else {
            let _ = read_long(file)?;
            None
        };
        println!("    ...loaded {:?}", account_unlock_date);
        println!("Loading player_name...");
        let player_name = fromutf8_to_ioresult(read_string_utf8(file)?, "player name")?;
        println!("    ...loaded {}", player_name);
        println!("Loading num_beatmaps...");
        let num_beatmaps = read_int(file)?;
        println!("    ...loaded {}", num_beatmaps);
        println!("Loading beatmaps...");
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
        println!("    ...loaded.");
        println!("Loading unknown_int...");
        let unknown_int = read_int(file)?;
        println!("    ...loaded {}", unknown_int);
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