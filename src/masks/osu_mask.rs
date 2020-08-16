use structopt::StructOpt;

#[derive(Copy, Clone, Debug, StructOpt)]
pub struct BeatmapMask {
    #[structopt(name = "show-entry-size", long = "show-entry-size")]
    pub entry_size: bool,
    #[structopt(name = "show-artist-name", long = "show-artist-name")]
    pub artist_name: bool,
    #[structopt(name = "show-artist-name-unicode", long = "show-artist-name-unicode")]
    pub artist_name_unicode: bool,
    #[structopt(name = "show-song-title", long = "show-song-title")]
    pub song_title: bool,
    #[structopt(name = "show-song-title-unicode", long = "show-song-title-unicode")]
    pub song_title_unicode: bool,
    #[structopt(name = "show-creator-name", long = "show-creator-name")]
    pub creator_name: bool,
    #[structopt(name = "show-difficulty", long = "show-difficulty")]
    pub difficulty: bool,
    #[structopt(name = "show-audio-file-name", long = "show-audio-file-name")]
    pub audio_file_name: bool,
    #[structopt(name = "show-md5-beatmap-hash", long = "show-md5-beatmap-hash")]
    pub md5_beatmap_hash: bool,
    #[structopt(name = "show-dotosu-file-name", long = "show-dotosu-file-name")]
    pub dotosu_file_name: bool,
    #[structopt(name = "show-ranked-status", long = "show-ranked-status")]
    pub ranked_status: bool,
    #[structopt(name = "show-number-of-hitcircles", long = "show-number-of-hitcircles")]
    pub number_of_hitcircles: bool,
    #[structopt(name = "show-number-of-sliders", long = "show-number-of-sliders")]
    pub number_of_sliders: bool,
    #[structopt(name = "show-number-of-spinners", long = "show-number-of-spinners")]
    pub number_of_spinners: bool,
    #[structopt(
        name = "show-last-modification-time",
        long = "show-last-modification-time"
    )]
    pub last_modification_time: bool,
    #[structopt(name = "show-approach-rate", long = "show-approach-rate")]
    pub approach_rate: bool,
    #[structopt(name = "show-circle-size", long = "show-circle-size")]
    pub circle_size: bool,
    #[structopt(name = "show-hp-drain", long = "show-hp-drain")]
    pub hp_drain: bool,
    #[structopt(name = "show-overall-difficulty", long = "show-overall-difficulty")]
    pub overall_difficulty: bool,
    #[structopt(name = "show-slider-velocity", long = "show-slider-velocity")]
    pub slider_velocity: bool,
    #[structopt(
        name = "show-num-mod-combo-star-ratings-standard",
        long = "show-num-mod-combo-star-ratings-standard"
    )]
    pub num_mod_combo_star_ratings_standard: bool,
    #[structopt(
        name = "show-mod-combo-star-ratings-standard",
        long = "show-mod-combo-star-ratings-standard"
    )]
    pub mod_combo_star_ratings_standard: bool,
    #[structopt(
        name = "show-num-mod-combo-star-ratings-taiko",
        long = "show-num-mod-combo-star-ratings-taiko"
    )]
    pub num_mod_combo_star_ratings_taiko: bool,
    #[structopt(
        name = "show-mod-combo-star-ratings-taiko",
        long = "show-mod-combo-star-ratings-taiko"
    )]
    pub mod_combo_star_ratings_taiko: bool,
    #[structopt(
        name = "show-num-mod-combo-star-ratings-ctb",
        long = "show-num-mod-combo-star-ratings-ctb"
    )]
    pub num_mod_combo_star_ratings_ctb: bool,
    #[structopt(
        name = "show-mod-combo-star-ratings-ctb",
        long = "show-mod-combo-star-ratings-ctb"
    )]
    pub mod_combo_star_ratings_ctb: bool,
    #[structopt(
        name = "show-num-mod-combo-star-ratings-mania",
        long = "show-num-mod-combo-star-ratings-mania"
    )]
    pub num_mod_combo_star_ratings_mania: bool,
    #[structopt(
        name = "show-mod-combo-star-ratings-mania",
        long = "show-mod-combo-star-ratings-mania"
    )]
    pub mod_combo_star_ratings_mania: bool,
    #[structopt(name = "show-drain-time", long = "show-drain-time")]
    pub drain_time: bool,
    #[structopt(name = "show-total-time", long = "show-total-time")]
    pub total_time: bool,
    #[structopt(
        name = "show-preview-offset-from-start-ms",
        long = "show-preview-offset-from-start-ms"
    )]
    pub preview_offset_from_start_ms: bool,
    #[structopt(name = "show-num-timing-points", long = "show-num-timing-points")]
    pub num_timing_points: bool,
    #[structopt(name = "show-timing-points", long = "show-timing-points")]
    pub timing_points: bool,
    #[structopt(name = "show-beatmap-id", long = "show-beatmap-id")]
    pub beatmap_id: bool,
    #[structopt(name = "show-beatmap-set-id", long = "show-beatmap-set-id")]
    pub beatmap_set_id: bool,
    #[structopt(name = "show-thread-id", long = "show-thread-id")]
    pub thread_id: bool,
    #[structopt(name = "show-standard-grade", long = "show-standard-grade")]
    pub standard_grade: bool,
    #[structopt(name = "show-taiko-grade", long = "show-taiko-grade")]
    pub taiko_grade: bool,
    #[structopt(name = "show-ctb-grade", long = "show-ctb-grade")]
    pub ctb_grade: bool,
    #[structopt(name = "show-mania-grade", long = "show-mania-grade")]
    pub mania_grade: bool,
    #[structopt(name = "show-local-offset", long = "show-local-offset")]
    pub local_offset: bool,
    #[structopt(name = "show-stack-leniency", long = "show-stack-leniency")]
    pub stack_leniency: bool,
    #[structopt(name = "show-gameplay-mode", long = "show-gameplay-mode")]
    pub gameplay_mode: bool,
    #[structopt(name = "show-song-source", long = "show-song-source")]
    pub song_source: bool,
    #[structopt(name = "show-song-tags", long = "show-song-tags")]
    pub song_tags: bool,
    #[structopt(name = "show-online-offset", long = "show-online-offset")]
    pub online_offset: bool,
    #[structopt(
        name = "show-font-used-for-song-title",
        long = "show-font-used-for-song-title"
    )]
    pub font_used_for_song_title: bool,
    #[structopt(name = "show-unplayed", long = "show-unplayed")]
    pub unplayed: bool,
    #[structopt(name = "show-last-played", long = "show-last-played")]
    pub last_played: bool,
    #[structopt(name = "show-is-osz2", long = "show-is-osz2")]
    pub is_osz2: bool,
    #[structopt(name = "show-beatmap-folder-name", long = "show-beatmap-folder-name")]
    pub beatmap_folder_name: bool,
    #[structopt(
        name = "show-last-checked-against-repo",
        long = "show-last-checked-against-repo"
    )]
    pub last_checked_against_repo: bool,
    #[structopt(name = "show-ignore-beatmap-sound", long = "show-ignore-beatmap-sound")]
    pub ignore_beatmap_sound: bool,
    #[structopt(name = "show-ignore-beatmap-skin", long = "show-ignore-beatmap-skin")]
    pub ignore_beatmap_skin: bool,
    #[structopt(name = "show-disable-storyboard", long = "show-disable-storyboard")]
    pub disable_storyboard: bool,
    #[structopt(name = "show-disable-video", long = "show-disable-video")]
    pub disable_video: bool,
    #[structopt(name = "show-visual-override", long = "show-visual-override")]
    pub visual_override: bool,
    #[structopt(name = "show-unknown-short", long = "show-unknown-short")]
    pub unknown_short: bool,
    #[structopt(
        name = "show-offset-from-song-start-in-editor-ms",
        long = "show-offset-from-song-start-in-editor-ms"
    )]
    pub offset_from_song_start_in_editor_ms: bool,
    #[structopt(name = "show-mania-scroll-speed", long = "show-mania-scroll-speed")]
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
}

#[derive(Copy, Clone, Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct OsuDbMask {
    #[structopt(name = "show-version", long = "show-version")]
    pub version: bool,
    #[structopt(name = "show-folder-count", long = "show-folder-count")]
    pub folder_count: bool,
    #[structopt(name = "show-account-unlocked", long = "show-account-unlocked")]
    pub account_unlocked: bool,
    #[structopt(name = "show-account-unlock-date", long = "show-account-unlock-date")]
    pub account_unlock_date: bool,
    #[structopt(name = "show-player-name", long = "show-player-name")]
    pub player_name: bool,
    #[structopt(name = "show-number-of-beatmaps", long = "show-number-of-beatmaps")]
    pub number_of_beatmaps: bool,
    #[structopt(flatten)]
    pub beatmap_mask: BeatmapMask,
    #[structopt(
        name = "show-unknown-short-or-permissions",
        long = "show-unknown-short-or-permissions"
    )]
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
}
