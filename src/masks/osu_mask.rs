use structopt::StructOpt;

#[derive(Copy, Clone, Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct BeatmapMask {
    #[structopt(long)]
    pub entry_size: bool,
    #[structopt(long)]
    pub artist_name: bool,
    #[structopt(long)]
    pub artist_name_unicode: bool,
    #[structopt(long)]
    pub song_title: bool,
    #[structopt(long)]
    pub song_title_unicode: bool,
    #[structopt(long)]
    pub creator_name: bool,
    #[structopt(long)]
    pub difficulty: bool,
    #[structopt(long)]
    pub audio_file_name: bool,
    #[structopt(long)]
    pub md5_beatmap_hash: bool,
    #[structopt(long)]
    pub dotosu_file_name: bool,
    #[structopt(long)]
    pub ranked_status: bool,
    #[structopt(long)]
    pub number_of_hitcircles: bool,
    #[structopt(long)]
    pub number_of_sliders: bool,
    #[structopt(long)]
    pub number_of_spinners: bool,
    #[structopt(long)]
    pub last_modification_time: bool,
    #[structopt(long)]
    pub approach_rate: bool,
    #[structopt(long)]
    pub circle_size: bool,
    #[structopt(long)]
    pub hp_drain: bool,
    #[structopt(long)]
    pub overall_difficulty: bool,
    #[structopt(long)]
    pub slider_velocity: bool,
    #[structopt(long)]
    pub num_mod_combo_star_ratings_standard: bool,
    #[structopt(long)]
    pub mod_combo_star_ratings_standard: bool,
    #[structopt(long)]
    pub num_mod_combo_star_ratings_taiko: bool,
    #[structopt(long)]
    pub mod_combo_star_ratings_taiko: bool,
    #[structopt(long)]
    pub num_mod_combo_star_ratings_ctb: bool,
    #[structopt(long)]
    pub mod_combo_star_ratings_ctb: bool,
    #[structopt(long)]
    pub num_mod_combo_star_ratings_mania: bool,
    #[structopt(long)]
    pub mod_combo_star_ratings_mania: bool,
    #[structopt(long)]
    pub drain_time: bool,
    #[structopt(long)]
    pub total_time: bool,
    #[structopt(long)]
    pub preview_offset_from_start_ms: bool,
    #[structopt(long)]
    pub num_timing_points: bool,
    #[structopt(long)]
    pub timing_points: bool,
    #[structopt(long)]
    pub beatmap_id: bool,
    #[structopt(long)]
    pub beatmap_set_id: bool,
    #[structopt(long)]
    pub thread_id: bool,
    #[structopt(long)]
    pub standard_grade: bool,
    #[structopt(long)]
    pub taiko_grade: bool,
    #[structopt(long)]
    pub ctb_grade: bool,
    #[structopt(long)]
    pub mania_grade: bool,
    #[structopt(long)]
    pub local_offset: bool,
    #[structopt(long)]
    pub stack_leniency: bool,
    #[structopt(long)]
    pub gameplay_mode: bool,
    #[structopt(long)]
    pub song_source: bool,
    #[structopt(long)]
    pub song_tags: bool,
    #[structopt(long)]
    pub online_offset: bool,
    #[structopt(long)]
    pub font_used_for_song_title: bool,
    #[structopt(long)]
    pub unplayed: bool,
    #[structopt(long)]
    pub last_played: bool,
    #[structopt(long)]
    pub is_osz2: bool,
    #[structopt(long)]
    pub beatmap_folder_name: bool,
    #[structopt(long)]
    pub last_checked_against_repo: bool,
    #[structopt(long)]
    pub ignore_beatmap_sound: bool,
    #[structopt(long)]
    pub ignore_beatmap_skin: bool,
    #[structopt(long)]
    pub disable_storyboard: bool,
    #[structopt(long)]
    pub disable_video: bool,
    #[structopt(long)]
    pub visual_override: bool,
    #[structopt(long)]
    pub unknown_short: bool,
    #[structopt(long)]
    pub offset_from_song_start_in_editor_ms: bool,
    #[structopt(long)]
    pub mania_scroll_speed: bool,
}

impl BeatmapMask {
    fn ignore_all(&self) -> bool {
        !self.entry_size
            && !self.artist_name
            && !self.artist_name_unicode
            && !self.song_title
            && !self.song_title_unicode
            && !self.creator_name
            && !self.difficulty
            && !self.audio_file_name
            && !self.md5_beatmap_hash
            && !self.dotosu_file_name
            && !self.ranked_status
            && !self.number_of_hitcircles
            && !self.number_of_sliders
            && !self.number_of_spinners
            && !self.last_modification_time
            && !self.approach_rate
            && !self.circle_size
            && !self.hp_drain
            && !self.overall_difficulty
            && !self.slider_velocity
            && !self.num_mod_combo_star_ratings_standard
            && !self.mod_combo_star_ratings_standard
            && !self.num_mod_combo_star_ratings_taiko
            && !self.mod_combo_star_ratings_taiko
            && !self.num_mod_combo_star_ratings_ctb
            && !self.mod_combo_star_ratings_ctb
            && !self.num_mod_combo_star_ratings_mania
            && !self.mod_combo_star_ratings_mania
            && !self.drain_time
            && !self.total_time
            && !self.preview_offset_from_start_ms
            && !self.num_timing_points
            && !self.timing_points
            && !self.beatmap_id
            && !self.beatmap_set_id
            && !self.thread_id
            && !self.standard_grade
            && !self.taiko_grade
            && !self.ctb_grade
            && !self.mania_grade
            && !self.local_offset
            && !self.stack_leniency
            && !self.gameplay_mode
            && !self.song_source
            && !self.song_tags
            && !self.online_offset
            && !self.font_used_for_song_title
            && !self.unplayed
            && !self.last_played
            && !self.is_osz2
            && !self.beatmap_folder_name
            && !self.last_checked_against_repo
            && !self.ignore_beatmap_sound
            && !self.ignore_beatmap_skin
            && !self.disable_storyboard
            && !self.disable_video
            && !self.visual_override
            && !self.unknown_short
            && !self.offset_from_song_start_in_editor_ms
            && !self.mania_scroll_speed
    }

    fn is_complete(&self) -> bool {
        self.entry_size
            && self.artist_name
            && self.artist_name_unicode
            && self.song_title
            && self.song_title_unicode
            && self.creator_name
            && self.difficulty
            && self.audio_file_name
            && self.md5_beatmap_hash
            && self.dotosu_file_name
            && self.ranked_status
            && self.number_of_hitcircles
            && self.number_of_sliders
            && self.number_of_spinners
            && self.last_modification_time
            && self.approach_rate
            && self.circle_size
            && self.hp_drain
            && self.overall_difficulty
            && self.slider_velocity
            && self.num_mod_combo_star_ratings_standard
            && self.mod_combo_star_ratings_standard
            && self.num_mod_combo_star_ratings_taiko
            && self.mod_combo_star_ratings_taiko
            && self.num_mod_combo_star_ratings_ctb
            && self.mod_combo_star_ratings_ctb
            && self.num_mod_combo_star_ratings_mania
            && self.mod_combo_star_ratings_mania
            && self.drain_time
            && self.total_time
            && self.preview_offset_from_start_ms
            && self.num_timing_points
            && self.timing_points
            && self.beatmap_id
            && self.beatmap_set_id
            && self.thread_id
            && self.standard_grade
            && self.taiko_grade
            && self.ctb_grade
            && self.mania_grade
            && self.local_offset
            && self.stack_leniency
            && self.gameplay_mode
            && self.song_source
            && self.song_tags
            && self.online_offset
            && self.font_used_for_song_title
            && self.unplayed
            && self.last_played
            && self.is_osz2
            && self.beatmap_folder_name
            && self.last_checked_against_repo
            && self.ignore_beatmap_sound
            && self.ignore_beatmap_skin
            && self.disable_storyboard
            && self.disable_video
            && self.visual_override
            && self.unknown_short
            && self.offset_from_song_start_in_editor_ms
            && self.mania_scroll_speed
    }
}

#[derive(Copy, Clone, Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct OsuDbMask {
    #[structopt(long)]
    pub version: bool,
    #[structopt(long)]
    pub folder_count: bool,
    #[structopt(long)]
    pub account_unlocked: bool,
    #[structopt(long)]
    pub account_unlock_date: bool,
    #[structopt(long)]
    pub player_name: bool,
    #[structopt(long)]
    pub number_of_beatmaps: bool,
    #[structopt(flatten)]
    pub beatmap_mask: BeatmapMask,
    #[structopt(long)]
    pub unknown_short_or_permissions: bool,
}

impl OsuDbMask {
    fn ignore_all(&self) -> bool {
        !self.version
            && !self.folder_count
            && !self.account_unlocked
            && !self.account_unlock_date
            && !self.player_name
            && !self.number_of_beatmaps
            && !self.unknown_short_or_permissions
            && self.beatmap_mask.ignore_all()
    }

    fn is_complete(&self) -> bool {
        self.version
            && self.folder_count
            && self.account_unlocked
            && self.account_unlock_date
            && self.player_name
            && self.number_of_beatmaps
            && self.unknown_short_or_permissions
            && self.beatmap_mask.is_complete()
    }
}
