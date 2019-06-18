use crate::masks::mask::Mask;

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
    pub fn new(entry_size: bool, artist_name: bool, artist_name_unicode: bool, song_title: bool,
        song_title_unicode: bool, creator_name: bool, difficulty: bool, audio_file_name: bool,
        md5_beatmap_hash: bool, dotosu_file_name: bool, ranked_status: bool,
        number_of_hitcircles: bool, number_of_sliders: bool, number_of_spinners: bool,
        last_modification_time: bool, approach_rate: bool, circle_size: bool, hp_drain: bool,
        overall_difficulty: bool, slider_velocity: bool, num_mod_combo_star_ratings_standard: bool,
        mod_combo_star_ratings_standard: bool, num_mod_combo_star_ratings_taiko: bool,
        mod_combo_star_ratings_taiko: bool, num_mod_combo_star_ratings_ctb: bool,
        mod_combo_star_ratings_ctb: bool, num_mod_combo_star_ratings_mania: bool,
        mod_combo_star_ratings_mania: bool, drain_time: bool, total_time: bool,
        preview_offset_from_start_ms: bool, num_timing_points: bool, timing_points: bool,
        beatmap_id: bool, beatmap_set_id: bool, thread_id: bool, standard_grade: bool,
        taiko_grade: bool, ctb_grade: bool, mania_grade: bool, local_offset: bool,
        stack_leniency: bool, gameplay_mode: bool, song_source: bool, song_tags: bool,
        online_offset: bool, font_used_for_song_title: bool, unplayed: bool, last_played: bool,
        is_osz2: bool, beatmap_folder_name: bool, last_checked_against_repo: bool,
        ignore_beatmap_sound: bool, ignore_beatmap_skin: bool, disable_storyboard: bool,
        disable_video: bool, visual_override: bool, unknown_short: bool,
        offset_from_song_start_in_editor_ms: bool, mania_scroll_speed: bool) -> Self {
        BeatmapMask {
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
            num_mod_combo_star_ratings_standard,
            mod_combo_star_ratings_standard,
            num_mod_combo_star_ratings_taiko,
            mod_combo_star_ratings_taiko,
            num_mod_combo_star_ratings_ctb,
            mod_combo_star_ratings_ctb,
            num_mod_combo_star_ratings_mania,
            mod_combo_star_ratings_mania,
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
        }
    }
}

impl Mask for BeatmapMask {
    fn is_complete(&self) -> bool {
        self.entry_size && self.artist_name && self.artist_name_unicode && self.song_title
            && self.song_title_unicode && self.creator_name && self.difficulty
            && self.audio_file_name && self.md5_beatmap_hash && self.dotosu_file_name
            && self.ranked_status && self.number_of_hitcircles && self.number_of_sliders
            && self.number_of_spinners && self.last_modification_time && self.approach_rate
            && self.circle_size && self.hp_drain && self.overall_difficulty && self.slider_velocity
            && self.num_mod_combo_star_ratings_standard && self.mod_combo_star_ratings_standard
            && self.num_mod_combo_star_ratings_taiko && self.mod_combo_star_ratings_taiko
            && self.num_mod_combo_star_ratings_ctb && self.mod_combo_star_ratings_ctb
            && self.num_mod_combo_star_ratings_mania && self.mod_combo_star_ratings_mania
            && self.drain_time && self.total_time && self.preview_offset_from_start_ms
            && self.num_timing_points && self.timing_points && self.beatmap_id
            && self.beatmap_set_id && self.thread_id && self.standard_grade && self.taiko_grade
            && self.ctb_grade && self.mania_grade && self.local_offset && self.stack_leniency
            && self.gameplay_mode && self.song_source && self.song_tags && self.online_offset
            && self.font_used_for_song_title && self.unplayed && self.last_played && self.is_osz2
            && self.beatmap_folder_name && self.last_checked_against_repo
            && self.ignore_beatmap_sound && self.ignore_beatmap_skin && self.disable_storyboard
            && self.disable_video && self.visual_override && self.unknown_short
            && self.offset_from_song_start_in_editor_ms && self.mania_scroll_speed
    }

    fn from_show_and_query(show: Self, query: Self) -> Self {
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

impl Default for BeatmapMask {
    fn default() -> Self {
        BeatmapMask {
            entry_size: true,
            artist_name: true,
            artist_name_unicode: true,
            song_title: true,
            song_title_unicode: true,
            creator_name: true,
            difficulty: true,
            audio_file_name: true,
            md5_beatmap_hash: true,
            dotosu_file_name: true,
            ranked_status: true,
            number_of_hitcircles: true,
            number_of_sliders: true,
            number_of_spinners: true,
            last_modification_time: true,
            approach_rate: true,
            circle_size: true,
            hp_drain: true,
            overall_difficulty: true,
            slider_velocity: true,
            num_mod_combo_star_ratings_standard: true,
            mod_combo_star_ratings_standard: true,
            num_mod_combo_star_ratings_taiko: true,
            mod_combo_star_ratings_taiko: true,
            num_mod_combo_star_ratings_ctb: true,
            mod_combo_star_ratings_ctb: true,
            num_mod_combo_star_ratings_mania: true,
            mod_combo_star_ratings_mania: true,
            drain_time: true,
            total_time: true,
            preview_offset_from_start_ms: true,
            num_timing_points: true,
            timing_points: true,
            beatmap_id: true,
            beatmap_set_id: true,
            thread_id: true,
            standard_grade: true,
            taiko_grade: true,
            ctb_grade: true,
            mania_grade: true,
            local_offset: true,
            stack_leniency: true,
            gameplay_mode: true,
            song_source: true,
            song_tags: true,
            online_offset: true,
            font_used_for_song_title: true,
            unplayed: true,
            last_played: true,
            is_osz2: true,
            beatmap_folder_name: true,
            last_checked_against_repo: true,
            ignore_beatmap_sound: true,
            ignore_beatmap_skin: true,
            disable_storyboard: true,
            disable_video: true,
            visual_override: true,
            unknown_short: true,
            offset_from_song_start_in_editor_ms: true,
            mania_scroll_speed: true
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct OsuDbMask {
    pub version: bool,
    pub folder_count: bool,
    pub account_unlocked: bool,
    pub account_unlock_date: bool,
    pub player_name: bool,
    pub number_of_beatmaps: bool,
    pub beatmap_mask: Option<BeatmapMask>,
    pub unknown_int: bool
}

impl OsuDbMask {
    pub fn new(version: bool, folder_count: bool, account_unlocked: bool, account_unlock_date: bool,
        player_name: bool, number_of_beatmaps: bool, beatmap_mask: Option<BeatmapMask>,
        unknown_int: bool) -> Self {
        OsuDbMask {
            version,
            folder_count,
            account_unlocked,
            account_unlock_date,
            player_name,
            number_of_beatmaps,
            beatmap_mask,
            unknown_int
        }
    }
}

impl Mask for OsuDbMask {
    fn is_complete(&self) -> bool {
        if let Some(beatmap_mask) = self.beatmap_mask {
            beatmap_mask.is_complete() && self.version && self.folder_count && self.account_unlocked
                && self.account_unlock_date && self.player_name && self.number_of_beatmaps
                && self.unknown_int
        } else {
            false
        }
    }

    fn from_show_and_query(show: Self, query: Self) -> Self {
        OsuDbMask {
            version: show.version || query.version,
            folder_count: show.folder_count || query.folder_count,
            account_unlocked: show.account_unlocked || query.account_unlocked,
            account_unlock_date: show.account_unlock_date || query.account_unlock_date,
            player_name: show.player_name || query.player_name,
            number_of_beatmaps: show.number_of_beatmaps || query.number_of_beatmaps,
            beatmap_mask: {
                match (show.beatmap_mask, query.beatmap_mask) {
                    (Some(show_mask), Some(query_mask)) => {
                        Some(BeatmapMask::from_show_and_query(show_mask, query_mask))
                    },
                    (Some(show_mask), None) => Some(show_mask),
                    (None, Some(query_mask)) => Some(query_mask),
                    (None, None) => None
                }
            },
            unknown_int: show.unknown_int || query.unknown_int
        }
    }
}

impl Default for OsuDbMask {
    fn default() -> Self {
        OsuDbMask {
            version: true,
            folder_count: true,
            account_unlocked: true,
            account_unlock_date: true,
            player_name: true,
            number_of_beatmaps: true,
            beatmap_mask: Some(BeatmapMask::default()),
            unknown_int: true
        }
    }
}