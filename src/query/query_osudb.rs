use crate::databases::osu::{OsuDb, Beatmap, TimingPoint, RankedStatus, ByteSingle, GameplayMode};
use crate::query::query::{QueryStruct, PartialDb, Query};
use std::time::SystemTime;
use std::path::Path;
use std::io::{Result as IoResult, Error as IoError, ErrorKind::InvalidData};
use std::fs::File;

pub struct OsuDbQuery {
    pub version: bool,
    pub folder_count: bool,
    pub account_unlocked: bool,
    pub account_unlock_date: bool,
    pub player_name: bool,
    pub number_of_beatmaps: bool,
    pub beatmap: Option<BeatmapQuery>,
    pub unknown_int: bool
}

impl QueryStruct for OsuDbQuery {}

impl OsuDbQuery {
    pub fn new(version: bool, folder_count: bool, account_unlocked: bool, account_unlock_date: bool,
        player_name: bool, number_of_beatmaps: bool, beatmap: Option<BeatmapQuery>,
        unknown_int: bool) -> Self {
        OsuDbQuery {
            version,
            folder_count,
            account_unlocked,
            account_unlock_date,
            player_name,
            number_of_beatmaps,
            beatmap,
            unknown_int
        }
    }
}

macro_rules! query {
    ($item:tt, $query:ident) => {
        if $query.$item {
            Some(self.$item)
        } else {
            None
        }
    }
}

impl Query for OsuDb {
    fn query_loaded(&self, query: OsuDbQuery) -> PartialOsuDb {
        let version = query!(version, query);
        let folder_count = query!(folder_count, query);
        let account_unlocked = query!(account_unlocked, query);
        let account_unlock_date = query!(account_unlock_date, query);
        let player_name = query!(player_name, query);
        let number_of_beatmaps = query!(number_of_beatmaps, query);
        let beatmaps = query!(beatmaps, query);
        let unknown_int = query!(unknown_int, query);
    }

    fn load_and_query<P: Into<Path>>(path: P, query: OsuDbQuery) -> PartialOsuDb {

    }
}

pub struct BeatmapQuery {
    entry_size: bool,
    artist_name: bool,
    artist_name_unicode: bool,
    song_title: bool,
    song_title_unicode: bool,
    creator_name: bool,
    difficulty: bool,
    audio_file_name: bool,
    md5_beatmap_hash: bool,
    dotosu_file_name: bool,
    ranked_status: bool,
    number_of_hitcircles: bool,
    number_of_sliders: bool,
    number_of_spinners: bool,
    last_modification_time: bool,
    approach_rate: bool,
    circle_size: bool,
    hp_drain: bool,
    overall_difficulty: bool,
    slider_velocity: bool,
    num_mod_combo_star_ratings_standard: bool,
    mod_combo_star_ratings_standard: bool,
    num_mod_combo_star_ratings_taiko: bool,
    mod_combo_star_ratings_taiko: bool,
    num_mod_combo_star_ratings_ctb: bool,
    mod_combo_star_ratings_ctb: bool,
    num_mod_combo_star_ratings_mania: bool,
    mod_combo_star_ratings_mania: bool,
    drain_time: bool,
    total_time: bool,
    preview_offset_from_start_ms: bool,
    num_timing_points: bool,
    timing_points: bool,
    beatmap_id: bool,
    beatmap_set_id: bool,
    thread_id: bool,
    standard_grade: bool,
    taiko_grade: bool,
    ctb_grade: bool,
    mania_grade: bool,
    local_offset: bool,
    stack_leniency: bool,
    gameplay_mode: bool,
    song_source: bool,
    song_tags: bool,
    online_offset: bool,
    font_used_for_song_title: bool,
    unplayed: bool,
    last_played: bool,
    is_osz2: bool,
    beatmap_folder_name: bool,
    last_checked_against_repo: bool,
    ignore_beatmap_sound: bool,
    ignore_beatmap_skin: bool,
    disable_storyboard: bool,
    disable_video: bool,
    visual_override: bool,
    unknown_short: bool,
    offset_from_song_start_in_editor_ms: bool,
    mania_scroll_speed: bool
}