#[derive(Copy, Clone, Debug)]
pub struct BeatmapMask {
    pub entry_size: bool,
    pub artist_name: bool,
    pub artist_name_unicode: bool,
    pub song_title: bool,
    pub song_title_unicode: bool,
    pub creator_name: bool,
    pub difficulty: bool,
    pub audio_file_name: bool,
    pub md5_beatmap_hash: bool,
    pub dotosu_file_name: bool,
    pub ranked_status: bool,
    pub number_of_hitcircles: bool,
    pub number_of_sliders: bool,
    pub number_of_spinners: bool,
    pub last_modification_time: bool,
    pub approach_rate: bool,
    pub circle_size: bool,
    pub hp_drain: bool,
    pub overall_difficulty: bool,
    pub slider_velocity: bool,
    pub num_mod_combo_star_ratings_standard: bool,
    pub mod_combo_star_ratings_standard: bool,
    pub num_mod_combo_star_ratings_taiko: bool,
    pub mod_combo_star_ratings_taiko: bool,
    pub num_mod_combo_star_ratings_ctb: bool,
    pub mod_combo_star_ratings_ctb: bool,
    pub num_mod_combo_star_ratings_mania: bool,
    pub mod_combo_star_ratings_mania: bool,
    pub drain_time: bool,
    pub total_time: bool,
    pub preview_offset_from_start_ms: bool,
    pub num_timing_points: bool,
    pub timing_points: bool,
    pub beatmap_id: bool,
    pub beatmap_set_id: bool,
    pub thread_id: bool,
    pub standard_grade: bool,
    pub taiko_grade: bool,
    pub ctb_grade: bool,
    pub mania_grade: bool,
    pub local_offset: bool,
    pub stack_leniency: bool,
    pub gameplay_mode: bool,
    pub song_source: bool,
    pub song_tags: bool,
    pub online_offset: bool,
    pub font_used_for_song_title: bool,
    pub unplayed: bool,
    pub last_played: bool,
    pub is_osz2: bool,
    pub beatmap_folder_name: bool,
    pub last_checked_against_repo: bool,
    pub ignore_beatmap_sound: bool,
    pub ignore_beatmap_skin: bool,
    pub disable_storyboard: bool,
    pub disable_video: bool,
    pub visual_override: bool,
    pub unknown_short: bool,
    pub offset_from_song_start_in_editor_ms: bool,
    pub mania_scroll_speed: bool
}

impl BeatmapMask {
    pub fn from_show_and_query(show: &BeatmapMask, query: &BeatmapMask) -> Self {
        BeatmapMask {
            entry_size: show.entry_size || query.entry_size,
            artist_name: show.artist_name || query.artist_name,
            artist_name_unicode: show.artist_name_unicode || query.artist_name_unicode,
            song_title: show.song_title || query.song_title,
            song_title_unicode: show.song_title_unicode || query.song_title_unicode,
            creator_name: show.creator_name || query.creator_name,
            difficulty: show.difficulty || query.difficulty,
            audio_file_name: show.audio_file_name || query.audio_file_name,
            md5_beatmap_hash: show.md5_beatmap_hash || query.md5_beatmap_hash,
            dotosu_file_name: show.dotosu_file_name || query.dotosu_file_name,
            ranked_status: show.ranked_status || query.ranked_status,
            number_of_hitcircles: show.number_of_hitcircles || query.number_of_hitcircles,
            number_of_sliders: show.number_of_sliders || query.number_of_sliders,
            number_of_spinners: show.number_of_spinners || query.number_of_spinners,
            last_modification_time: show.last_modification_time || query.last_modification_time,
            approach_rate: show.approach_rate || query.approach_rate,
            circle_size: show.circle_size || query.circle_size,
            hp_drain: show.hp_drain || query.hp_drain,
            overall_difficulty: show.overall_difficulty || query.overall_difficulty,
            slider_velocity: show.slider_velocity || query.slider_velocity,
            num_mod_combo_star_ratings_standard: show.num_mod_combo_star_ratings_standard
                || query.num_mod_combo_star_ratings_standard,
            mod_combo_star_ratings_standard: show.mod_combo_star_ratings_standard
                || query.mod_combo_star_ratings_standard,
            num_mod_combo_star_ratings_taiko: show.num_mod_combo_star_ratings_taiko
                || query.num_mod_combo_star_ratings_taiko,
            mod_combo_star_ratings_taiko: show.mod_combo_star_ratings_taiko
                || query.mod_combo_star_ratings_taiko,
            num_mod_combo_star_ratings_ctb: show.num_mod_combo_star_ratings_ctb
                || query.num_mod_combo_star_ratings_ctb,
            mod_combo_star_ratings_ctb: show.mod_combo_star_ratings_ctb
                || query.mod_combo_star_ratings_ctb,
            num_mod_combo_star_ratings_mania: show.num_mod_combo_star_ratings_mania
                || query.num_mod_combo_star_ratings_mania,
            mod_combo_star_ratings_mania: show.mod_combo_star_ratings_mania
                || query.mod_combo_star_ratings_mania,
            drain_time: show.drain_time || query.drain_time,
            total_time: show.total_time || query.total_time,
            preview_offset_from_start_ms: show.preview_offset_from_start_ms
                || query.preview_offset_from_start_ms,
            num_timing_points: show.num_timing_points || query.num_timing_points,
            timing_points: show.timing_points || query.timing_points,
            beatmap_id: show.beatmap_id || query.beatmap_id,
            beatmap_set_id: show.beatmap_set_id || query.beatmap_set_id,
            thread_id: show.thread_id || query.thread_id,
            standard_grade: show.standard_grade || query.standard_grade,
            taiko_grade: show.taiko_grade || query.taiko_grade,
            ctb_grade: show.ctb_grade || query.ctb_grade,
            mania_grade: show.mania_grade || query.mania_grade,
            local_offset: show.local_offset || query.local_offset,
            stack_leniency: show.stack_leniency || query.stack_leniency,
            gameplay_mode: show.gameplay_mode || query.gameplay_mode,
            song_source: show.song_source || query.song_source,
            song_tags: show.song_tags || query.song_tags,
            online_offset: show.online_offset || query.online_offset,
            font_used_for_song_title: show.font_used_for_song_title
                || query.font_used_for_song_title,
            unplayed: show.unplayed || query.unplayed,
            last_played: show.last_played || query.last_played,
            is_osz2: show.is_osz2 || query.is_osz2,
            beatmap_folder_name: show.beatmap_folder_name || query.beatmap_folder_name,
            last_checked_against_repo: show.last_checked_against_repo
                || query.last_checked_against_repo,
            ignore_beatmap_sound: show.ignore_beatmap_sound || query.ignore_beatmap_sound,
            ignore_beatmap_skin: show.ignore_beatmap_skin || query.ignore_beatmap_skin,
            disable_storyboard: show.disable_storyboard || query.disable_storyboard,
            disable_video: show.disable_video || query.disable_video,
            visual_override: show.visual_override || query.visual_override,
            unknown_short: show.unknown_short || query.unknown_short,
            offset_from_song_start_in_editor_ms: show.offset_from_song_start_in_editor_ms
                || query.offset_from_song_start_in_editor_ms,
            mania_scroll_speed: show.mania_scroll_speed || query.mania_scroll_speed
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct OsuDb {
    pub version: bool,
    pub folder_count: bool,
    pub account_unlocked: bool,
    pub account_unlock_date: bool,
    pub player_name: bool,
    pub number_of_beatmaps: bool,
    pub beatmap_mask: BeatmapMask,
    pub unknown_int: bool
}

impl OsuDb {
    fn from_show_and_query(show: &Self, query: &Self) -> Self {
        OsuDb {
            version: show.version || query.version,
            folder_count: show.folder_count || query.folder_count,
            account_unlocked: show.account_unlocked || query.account_unlocked,
            account_unlock_date: show.account_unlock_date || query.account_unlock_date,
            player_name: show.player_name || query.player_name,
            number_of_beatmaps: show.number_of_beatmaps || query.number_of_beatmaps,
            beatmap_mask: BeatmapMask::from_show_and_query(show.beatmap_mask, query.beatmap_mask),
            unknown_int: show.unknown_int || query.unknown_int
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Collection {
    pub collection_name: bool,
    pub number_of_beatmaps: bool,
    pub md5_beatmap_hashes: bool
}

fn