use structopt::StructOpt;

#[derive(Copy, Clone, Debug, StructOpt)]
pub struct BeatmapMask {
    #[structopt(long = "show-entry-size")]
    pub entry_size: bool,
    #[structopt(long = "show-artist-name")]
    pub artist_name: bool,
    #[structopt(long = "show-artist-name-unicode")]
    pub artist_name_unicode: bool,
    #[structopt(long = "show-song-title")]
    pub song_title: bool,
    #[structopt(long = "show-song_title-unicode")]
    pub song_title_unicode: bool,
    #[structopt(long = "show-creator-name")]
    pub creator_name: bool,
    #[structopt(long = "show-difficulty")]
    pub difficulty: bool,
    #[structopt(long = "show-audio-file-name")]
    pub audio_file_name: bool,
    #[structopt(long = "show-md5-beatmap-hash")]
    pub md5_beatmap_hash: bool,
    #[structopt(long = "show-dotosu-file-name")]
    pub dotosu_file_name: bool,
    #[structopt(long = "show-ranked-status")]
    pub ranked_status: bool,
    #[structopt(long = "show-number-of-hitcircles")]
    pub number_of_hitcircles: bool,
    #[structopt(long = "show-number-of-sliders")]
    pub number_of_sliders: bool,
    #[structopt(long = "show-number-of-spinners")]
    pub number_of_spinners: bool,
    #[structopt(long = "show-last_modification-time")]
    pub last_modification_time: bool,
    #[structopt(long = "show-approach-rate")]
    pub approach_rate: bool,
    #[structopt(long = "show-circle-size")]
    pub circle_size: bool,
    #[structopt(long = "show-hp-drain")]
    pub hp_drain: bool,
    #[structopt(long = "show-overall-difficulty")]
    pub overall_difficulty: bool,
    #[structopt(long = "show-slider-velocity")]
    pub slider_velocity: bool,
    #[structopt(long = "show-num-mod-combo-star-ratings-standard")]
    pub num_mod_combo_star_ratings_standard: bool,
    #[structopt(long = "show-mod-combo-star-ratings-standard")]
    pub mod_combo_star_ratings_standard: bool,
    #[structopt(long = "show-num-mod-combo-star-ratings-taiko")]
    pub num_mod_combo_star_ratings_taiko: bool,
    #[structopt(long = "show-mod-combo-star-ratings-taiko")]
    pub mod_combo_star_ratings_taiko: bool,
    #[structopt(long = "show-num-mod-combo-star-ratings-ctb")]
    pub num_mod_combo_star_ratings_ctb: bool,
    #[structopt(long = "show-mod-combo-star-ratings-ctb")]
    pub mod_combo_star_ratings_ctb: bool,
    #[structopt(long = "show-num-mod-combo-star-ratings-mania")]
    pub num_mod_combo_star_ratings_mania: bool,
    #[structopt(long = "show-mod-combo-star-ratings-mania")]
    pub mod_combo_star_ratings_mania: bool,
    #[structopt(long = "show-drain-time")]
    pub drain_time: bool,
    #[structopt(long = "show-total-time")]
    pub total_time: bool,
    #[structopt(long = "show-preview-offset-from-start-ms")]
    pub preview_offset_from_start_ms: bool,
    #[structopt(long = "show-num-timing-points")]
    pub num_timing_points: bool,
    #[structopt(long = "show-timing-points")]
    pub timing_points: bool,
    #[structopt(long = "show-beatmap-id")]
    pub beatmap_id: bool,
    #[structopt(long = "show-beatmap-set-id")]
    pub beatmap_set_id: bool,
    #[structopt(long = "show-thread-id")]
    pub thread_id: bool,
    #[structopt(long = "show-standard-grade")]
    pub standard_grade: bool,
    #[structopt(long = "show-taiko-grade")]
    pub taiko_grade: bool,
    #[structopt(long = "show-ctb-grade")]
    pub ctb_grade: bool,
    #[structopt(long = "show-mania-grade")]
    pub mania_grade: bool,
    #[structopt(long = "show-local-offset")]
    pub local_offset: bool,
    #[structopt(long = "show-stack-leniency")]
    pub stack_leniency: bool,
    #[structopt(long = "show-gameplay-mode")]
    pub gameplay_mode: bool,
    #[structopt(long = "show-song-source")]
    pub song_source: bool,
    #[structopt(long = "show-song-tags")]
    pub song_tags: bool,
    #[structopt(long = "show-online-offset")]
    pub online_offset: bool,
    #[structopt(long = "show-font-used-for-song-title")]
    pub font_used_for_song_title: bool,
    #[structopt(long = "show-unplayed")]
    pub unplayed: bool,
    #[structopt(long = "show-last-played")]
    pub last_played: bool,
    #[structopt(long = "show-is-osz2")]
    pub is_osz2: bool,
    #[structopt(long = "show-beatmap-folder-name")]
    pub beatmap_folder_name: bool,
    #[structopt(long = "show-last-checked-against-repo")]
    pub last_checked_against_repo: bool,
    #[structopt(long = "show-ignore-beatmap-sound")]
    pub ignore_beatmap_sound: bool,
    #[structopt(long = "show-ignore-beatmap-skin")]
    pub ignore_beatmap_skin: bool,
    #[structopt(long = "show-disable-storyboard")]
    pub disable_storyboard: bool,
    #[structopt(long = "show-disable-video")]
    pub disable_video: bool,
    #[structopt(long = "show-visual-override")]
    pub visual_override: bool,
    #[structopt(long = "show-unknown-short")]
    pub unknown_short: bool,
    #[structopt(long = "show-offset-from-song-start-in-editor-ms")]
    pub offset_from_song_start_in_editor_ms: bool,
    #[structopt(long = "show-mania-scroll-speed")]
    pub mania_scroll_speed: bool,
}

impl BeatmapMask {
    pub fn ignore_all(&self) -> bool {
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

    pub fn is_complete(&self) -> bool {
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
    #[structopt(long = "show-version")]
    pub version: bool,
    #[structopt(long = "show-folder-count")]
    pub folder_count: bool,
    #[structopt(long = "show-account-unlocked")]
    pub account_unlocked: bool,
    #[structopt(long = "show-account-unlock-date")]
    pub account_unlock_date: bool,
    #[structopt(long = "show-player-name")]
    pub player_name: bool,
    #[structopt(long = "show-number-of-beatmaps")]
    pub number_of_beatmaps: bool,
    #[structopt(flatten)]
    pub beatmap_mask: BeatmapMask,
    #[structopt(long = "show-unknown-short-or-permissions")]
    pub unknown_short_or_permissions: bool,
}

impl OsuDbMask {
    pub fn ignore_all(&self) -> bool {
        !self.version
            && !self.folder_count
            && !self.account_unlocked
            && !self.account_unlock_date
            && !self.player_name
            && !self.number_of_beatmaps
            && !self.unknown_short_or_permissions
            && self.beatmap_mask.ignore_all()
    }

    pub fn is_complete(&self) -> bool {
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
