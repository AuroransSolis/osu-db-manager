use crate::query::query::PartialDb;
use crate::databases::osu::{Beatmap, ByteSingle, RankedStatus, GameplayMode, TimingPoint,
    Pre20140609, Modern, ModernWithEntrySize};
use crate::query::query_osudb::OsuDbQuery;
use crate::deserialize_primitives::*;
use std::fs::File;
use std::time::SystemTime;
use std::io::{Result as IoResult, Error as IoError, ErrorKind::InvalidData, Cursor};

pub struct PartialOsuDb {
    pub version: Option<i32>,
    pub folder_count: Option<i32>,
    pub account_unlocked: Option<bool>,
    pub account_unlock_date: Option<SystemTime>,
    pub player_name: Option<String>,
    pub number_of_beatmaps: Option<i32>,
    pub beatmaps: Option<Vec<Beatmap>>,
    pub unknown_int: Option<i32>
}

impl PartialDb for PartialOsuDb {}

pub trait ReadPartialVersionSpecificData {

}

macro_rules! read_or_advance {
    ($cursor:ident, $advance:literal, $condition:expr, $load:expr) => {
        if $condition {
            Some($load)
        } else {
            let current_pos = $cursor.position();
            $cursor.set_position(current_pos + $advance);
            None
        }
    }
}

impl PartialOsuDb {
    pub fn load_from_query<V: ReadPartialVersionSpecificData>(file: &mut File, jobs: usize,
        query: &OsuDbQuery) -> IoResult<Self> {
        let mut cursor = Cursor::new(file);
        let version = read_or_advance!(cursor, 4, query.version, read_int(cursor)?);
        let folder_count = read_or_advance!(cursor, 4, query.folder_count, read_int(cursor)?);
        let account_unlocked = read_or_advance!(cursor, 1, query.account_unlocked,
            read_boolean(cursor)?);
        let account_unlock_date = read_or_advance!(cursor, 8, query.account_unlock_date,
            read_datetime(cursor)?);
        let player_name = if query.player_name {
            Some(fromutf8_to_ioresult(read_string_utf8(cursor)?, "player name")?)
        } else {
            let new_pos = read_uleb128(cursor)? as u64 + cursor.position();
            cursor.set_position(new_pos);
            None
        };
        let number_of_beatmaps = read_or_advance!(cursor, 4, query.number_of_beatmaps,
            read_int(cursor));
        let beatmaps = if let Some = query.beatmap {
            if let Some = number_of_beatmaps {

            } else {
                return Err(IoError::new(InvalidData, "Attempted to load in beatmaps \
                    without loading in number of beatmaps!"));
            }
        } else {

        };
    }
}

pub struct PartialBeatmap {
    entry_size: Option<i32>,
    artist_name: Option<String>,
    artist_name_unicode: Option<String>,
    song_title: Option<String>,
    song_title_unicode: Option<String>,
    creator_name: Option<String>,
    difficulty: Option<String>,
    audio_file_name: Option<String>,
    md5_beatmap_hash: Option<String>,
    dotosu_file_name: Option<String>,
    ranked_status: Option<RankedStatus>,
    number_of_hitcircles: Option<i16>,
    number_of_sliders: Option<i16>,
    number_of_spinners: Option<i16>,
    last_modification_time: Option<SystemTime>,
    approach_rate: Option<ByteSingle>,
    circle_size: Option<ByteSingle>,
    hp_drain: Option<ByteSingle>,
    overall_difficulty: Option<ByteSingle>,
    slider_velocity: Option<f64>,
    num_mod_combo_star_ratings_standard: Option<i32>,
    mod_combo_star_ratings_standard: Option<Vec<(i32, f64)>>,
    num_mod_combo_star_ratings_taiko: Option<i32>,
    mod_combo_star_ratings_taiko: Option<Vec<(i32, f64)>>,
    num_mod_combo_star_ratings_ctb: Option<i32>,
    mod_combo_star_ratings_ctb: Option<Vec<(i32, f64)>>,
    num_mod_combo_star_ratings_mania: Option<i32>,
    mod_combo_star_ratings_mania: Option<Vec<(i32, f64)>>,
    drain_time: Option<i32>,
    total_time: Option<i32>,
    preview_offset_from_start_ms: Option<i32>,
    num_timing_points: Option<i32>,
    timing_points: Option<Vec<TimingPoint>>,
    beatmap_id: Option<i32>,
    beatmap_set_id: Option<i32>,
    thread_id: Option<i32>,
    standard_grade: Option<u8>,
    taiko_grade: Option<u8>,
    ctb_grade: Option<u8>,
    mania_grade: Option<u8>,
    local_offset: Option<i16>,
    stack_leniency: Option<f32>,
    gameplay_mode: Option<GameplayMode>,
    song_source: Option<String>,
    song_tags: Option<String>,
    online_offset: Option<i16>,
    font_used_for_song_title: Option<String>,
    unplayed: Option<bool>,
    last_played: Option<SystemTime>,
    is_osz2: Option<bool>,
    beatmap_folder_name: Option<String>,
    last_checked_against_repo: Option<SystemTime>,
    ignore_beatmap_sound: Option<bool>,
    ignore_beatmap_skin: Option<bool>,
    disable_storyboard: Option<bool>,
    disable_video: Option<bool>,
    visual_override: Option<bool>,
    unknown_short: Option<i16>,
    offset_from_song_start_in_editor_ms: Option<i32>,
    mania_scroll_speed: Option<u8>
}