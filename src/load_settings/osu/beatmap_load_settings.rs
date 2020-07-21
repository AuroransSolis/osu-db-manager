use crate::databases::osu::primitives::{ByteSingle, GameplayMode, RankedStatus};
use crate::load_settings::{EqualClone, EqualCopy, Relational};
use crate::masks::osu_mask::BeatmapMask;
use chrono::naive::NaiveDate;
use structopt::StructOpt;

#[derive(Clone, StructOpt)]
pub struct BeatmapLoadSettings {
    #[structopt(
        name = "entry size",
        long = "entry-size",
        value_name = "RELATIONAL",
        help = "Size of the beatmap entry in bytes",
        default_value,
        parse(try_from_str)
    )]
    pub entry_size: Relational<i32>,
    #[structopt(
        name = "artist name",
        long = "artist-name",
        value_name = "EQ",
        default_value,
        parse(try_from_str)
    )]
    pub artist_name: EqualClone<String>,
    #[structopt(
        name = "artist name unicode",
        long = "artist-name-unicode",
        value_name = "EQ",
        default_value,
        parse(try_from_str)
    )]
    pub artist_name_unicode: EqualClone<String>,
    #[structopt(
        name = "song title",
        long = "song-title",
        value_name = "EQ",
        default_value,
        parse(try_from_str)
    )]
    pub song_title: EqualClone<String>,
    #[structopt(
        name = "song title unicode",
        long = "song-title-unicode",
        value_name = "EQ",
        default_value,
        parse(try_from_str)
    )]
    pub song_title_unicode: EqualClone<String>,
    #[structopt(
        name = "creator name",
        long = "creator-name",
        value_name = "EQ",
        default_value,
        help = "Name of the creator of the beatmap",
        parse(try_from_str)
    )]
    pub creator_name: EqualClone<String>,
    #[structopt(
        name = "difficulty",
        long = "difficulty",
        value_name = "EQ",
        help = "Name of the difficulty of this map in its mapset",
        default_value,
        parse(try_from_str)
    )]
    pub difficulty: EqualClone<String>,
    #[structopt(
        name = "audio file name",
        long = "audio-file-name",
        value_name = "EQ",
        default_value,
        parse(try_from_str)
    )]
    pub audio_file_name: EqualClone<String>,
    #[structopt(
        name = "MD5 beatmap hash",
        long = "md5-beatmap-hash",
        value_name = "EQ",
        default_value,
        parse(try_from_str)
    )]
    pub md5_beatmap_hash: EqualClone<String>,
    #[structopt(
        name = ".osu file name",
        long = "dotosu-file-name",
        value_name = "EQ",
        default_value,
        parse(try_from_str)
    )]
    pub dotosu_file_name: EqualClone<String>,
    #[structopt(
        name = "ranked status",
        long = "ranked-status",
        value_name = "EQ",
        default_value,
        possible_values(&[
            "unknown",
            "unsubmitted",
            "pending",
            "wip",
            "graveyard",
            "unused",
            "ranked",
            "approved",
            "qualified",
            "loved"
        ]),
        long_help = "Possible values: unknown, unsubmitted, pending, wip, graveyard, unused, \
            ranked, approved, qualified, loved",
        parse(try_from_str)
    )]
    pub ranked_status: EqualCopy<RankedStatus>,
    #[structopt(
        name = "number of hitcircles",
        long = "number-of-hitcircles",
        value_name = "RELATIONAL",
        default_value,
        parse(try_from_str)
    )]
    pub number_of_hitcircles: Relational<i16>,
    #[structopt(
        name = "number of sliders",
        long = "number-of-sliders",
        value_name = "RELATIONAL",
        default_value,
        parse(try_from_str)
    )]
    pub number_of_sliders: Relational<i16>,
    #[structopt(
        name = "number of spinners",
        long = "number-of-spinners",
        value_name = "RELATIONAL",
        default_value,
        parse(try_from_str)
    )]
    pub number_of_spinners: Relational<i16>,
    #[structopt(
        name = "last modification time",
        long = "last-modification-time",
        value_name = "RELATIONAL-DATE",
        default_value,
        parse(try_from_str)
    )]
    pub last_modification_time: Relational<NaiveDate>,
    #[structopt(
        name = "approach rate",
        alias = "ar",
        long = "approach-rate",
        value_name = "RELATIONAL",
        default_value,
        parse(try_from_str)
    )]
    pub approach_rate: Relational<ByteSingle>,
    #[structopt(
        name = "circle size",
        alias = "cs",
        long = "circle-size",
        value_name = "RELATIONAL",
        default_value,
        parse(try_from_str)
    )]
    pub circle_size: Relational<ByteSingle>,
    #[structopt(
        name = "hp drain",
        alias = "hp",
        long = "hp-drain",
        value_name = "RELATIONAL",
        default_value,
        parse(try_from_str)
    )]
    pub hp_drain: Relational<ByteSingle>,
    #[structopt(
        name = "overall difficulty",
        alias = "od",
        long = "overall-difficulty",
        value_name = "RELATIONAL",
        default_value,
        parse(try_from_str)
    )]
    pub overall_difficulty: Relational<ByteSingle>,
    #[structopt(
        name = "slider velocity",
        long = "slider-velocity",
        value_name = "RELATIONAL",
        default_value,
        parse(try_from_str)
    )]
    pub slider_velocity: Relational<f64>,
    #[structopt(skip)]
    pub num_mod_combo_star_ratings_standard: bool,
    #[structopt(skip)]
    pub mod_combo_star_ratings_standard: bool,
    #[structopt(skip)]
    pub num_mod_combo_star_ratings_taiko: bool,
    #[structopt(skip)]
    pub mod_combo_star_ratings_taiko: bool,
    #[structopt(skip)]
    pub num_mod_combo_star_ratings_ctb: bool,
    #[structopt(skip)]
    pub mod_combo_star_ratings_ctb: bool,
    #[structopt(skip)]
    pub num_mod_combo_star_ratings_mania: bool,
    #[structopt(skip)]
    pub mod_combo_star_ratings_mania: bool,
    #[structopt(
        name = "drain time",
        long = "drain-time",
        value_name = "RELATIONAL",
        default_value,
        parse(try_from_str)
    )]
    pub drain_time: Relational<i32>,
    #[structopt(
        name = "total time",
        long = "total-time",
        value_name = "RELATIONAL",
        default_value,
        parse(try_from_str)
    )]
    pub total_time: Relational<i32>,
    #[structopt(
        name = "preview offset from start ms",
        long = "preview-offset-from-start-ms",
        value_name = "RELATIONAL",
        default_value,
        parse(try_from_str)
    )]
    pub preview_offset_from_start_ms: Relational<i32>,
    #[structopt(
        name = "num timing points",
        long = "num-timing-points",
        value_name = "RELATIONAL",
        default_value,
        parse(try_from_str)
    )]
    pub num_timing_points: Relational<i32>,
    #[structopt(skip)]
    pub timing_points: bool,
    #[structopt(
        name = "beatmap id",
        long = "beatmap-id",
        value_name = "RELATIONAL",
        help = "Beatmap ID - e.g. 992022 in https://osu.ppy.sh/beatmapsets/439396#osu/992022",
        default_value,
        parse(try_from_str)
    )]
    pub beatmap_id: Relational<i32>,
    #[structopt(
        name = "beatmap set id",
        long = "beatmap-set-id",
        value_name = "RELATIONAL",
        help = "Beatmap set ID - e.g. 439396 in https://osu.ppy.sh/beatmapsets/439396#osu/992022",
        default_value,
        parse(try_from_str)
    )]
    pub beatmap_set_id: Relational<i32>,
    #[structopt(
        name = "thread id",
        long = "thread-id",
        value_name = "RELATIONAL",
        help = "Thread ID - e.g. 440377 in https://osu.ppy.sh/community/forums/topics/440377",
        default_value,
        parse(try_from_str)
    )]
    pub thread_id: Relational<i32>,
    #[structopt(
        name = "standard grade",
        long = "standard-grade",
        value_name = "RELATIONAL",
        default_value,
        parse(try_from_str)
    )]
    pub standard_grade: Relational<u8>,
    #[structopt(
        name = "taiko grade",
        long = "taiko-grade",
        value_name = "RELATIONAL",
        default_value,
        parse(try_from_str)
    )]
    pub taiko_grade: Relational<u8>,
    #[structopt(
        name = "ctb grade",
        long = "ctb-grade",
        value_name = "RELATIONAL",
        default_value,
        parse(try_from_str)
    )]
    pub ctb_grade: Relational<u8>,
    #[structopt(
        name = "mania grade",
        long = "mania-grade",
        value_name = "RELATIONAL",
        default_value,
        parse(try_from_str)
    )]
    pub mania_grade: Relational<u8>,
    #[structopt(
        name = "local offset",
        long = "local-offset",
        value_name = "RELATIONAL",
        default_value,
        parse(try_from_str)
    )]
    pub local_offset: Relational<i16>,
    #[structopt(
        name = "stack leniency",
        long = "stack-leniency",
        value_name = "RELATIONAL",
        default_value,
        parse(try_from_str)
    )]
    pub stack_leniency: Relational<f32>,
    #[structopt(
        name = "gameplay mode",
        long = "gameplay-mode",
        value_name = "EQ",
        possible_values(&[
            "osu!",
            "osu",
            "osu!standard",
            "standard",
            "osu!taiko",
            "taiko",
            "osu!ctb",
            "ctb",
            "catch-the-beat",
            "osu!mania",
            "mania",
        ]),
        long_help = "Possible values: osu!, osu, osu!standard, standard, osu!taiko, taiko, \
            osu!ctb, ctb, catch-the-beat, osu!mania, mania",
        default_value,
        parse(try_from_str)
    )]
    pub gameplay_mode: EqualCopy<GameplayMode>,
    #[structopt(
        name = "song source",
        long = "song-source",
        value_name = "EQ",
        default_value,
        parse(try_from_str)
    )]
    pub song_source: EqualClone<String>,
    #[structopt(
        name = "song tags",
        long = "song-tags",
        value_name = "EQ-COMMA-SEPARATED",
        default_value,
        parse(try_from_str)
    )]
    pub song_tags: EqualClone<String>,
    #[structopt(
        name = "online offset",
        long = "online-offset",
        value_name = "RELATIONAL",
        default_value,
        parse(try_from_str)
    )]
    pub online_offset: Relational<i16>,
    #[structopt(
        name = "font used for song title",
        long = "font-used-for-song-title",
        value_name = "EQ",
        default_value,
        parse(try_from_str)
    )]
    pub font_used_for_song_title: EqualClone<String>,
    #[structopt(
        name = "unplayed",
        long = "unplayed",
        value_name = "EQ-BOOL",
        possible_values(&["t", "true", "y", "yes", "1", "f", "false", "n", "no", "0"]),
        default_value,
        parse(try_from_str)
    )]
    pub unplayed: EqualCopy<bool>,
    #[structopt(
        name = "last played",
        long = "last-played",
        value_name = "RELATIONAL",
        default_value,
        parse(try_from_str)
    )]
    pub last_played: Relational<NaiveDate>,
    #[structopt(
        name = "is OSZ2",
        long = "is-osz2",
        value_name = "EQ-BOOL",
        possible_values(&["t", "true", "y", "yes", "1", "f", "false", "n", "no", "0"]),
        default_value,
        parse(try_from_str)
    )]
    pub is_osz2: EqualCopy<bool>,
    #[structopt(
        name = "beatmap folder name",
        long = "beatmap-folder-name",
        value_name = "EQ",
        default_value,
        parse(try_from_str)
    )]
    pub beatmap_folder_name: EqualClone<String>,
    #[structopt(
        name = "last checked against repo",
        long = "last-checked-against-repo",
        value_name = "RELATIONAL-DATE",
        default_value,
        parse(try_from_str)
    )]
    pub last_checked_against_repo: Relational<NaiveDate>,
    #[structopt(
        name = "ignore beatmap sound",
        long = "ignore-beatmap-sound",
        value_name = "EQ-BOOL",
        possible_values(&["t", "true", "y", "yes", "1", "f", "false", "n", "no", "0"]),
        default_value,
        parse(try_from_str)
    )]
    pub ignore_beatmap_sound: EqualCopy<bool>,
    #[structopt(
        name = "ignore beatmap skin",
        long = "ignore-beatmap-skin",
        value_name = "EQ-BOOL",
        possible_values(&["t", "true", "y", "yes", "1", "f", "false", "n", "no", "0"]),
        default_value,
        parse(try_from_str)
    )]
    pub ignore_beatmap_skin: EqualCopy<bool>,
    #[structopt(
        name = "disable storyboard",
        long = "disable-storyboard",
        value_name = "EQ-BOOL",
        possible_values(&["t", "true", "y", "yes", "1", "f", "false", "n", "no", "0"]),
        default_value,
        parse(try_from_str)
    )]
    pub disable_storyboard: EqualCopy<bool>,
    #[structopt(
        name = "disable video",
        long = "disable-video",
        value_name = "EQ-BOOL",
        possible_values(&["t", "true", "y", "yes", "1", "f", "false", "n", "no", "0"]),
        default_value,
        parse(try_from_str)
    )]
    pub disable_video: EqualCopy<bool>,
    #[structopt(
        name = "visual override",
        long = "visual-override",
        value_name = "EQ-BOOL",
        possible_values(&["t", "true", "y", "yes", "1", "f", "false", "n", "no", "0"]),
        default_value,
        parse(try_from_str)
    )]
    pub visual_override: EqualCopy<bool>,
    #[structopt(skip)]
    pub unknown_short: bool,
    #[structopt(
        name = "offset from song start in editor ms",
        long = "offset-from-song-start-in-editor-ms",
        value_name = "RELATIONAL",
        default_value,
        parse(try_from_str)
    )]
    pub offset_from_song_start_in_editor_ms: Relational<i32>,
    #[structopt(
        name = "mania scroll speed",
        long = "mania-scroll-speed",
        value_name = "RELATIONAL",
        default_value,
        parse(try_from_str)
    )]
    pub mania_scroll_speed: Relational<u8>,
}

impl BeatmapLoadSettings {
    pub fn load_all(&self) -> bool {
        self.entry_size.is_load()
            && self.artist_name.is_load()
            && self.artist_name_unicode.is_load()
            && self.song_title.is_load()
            && self.song_title_unicode.is_load()
            && self.creator_name.is_load()
            && self.difficulty.is_load()
            && self.audio_file_name.is_load()
            && self.md5_beatmap_hash.is_load()
            && self.dotosu_file_name.is_load()
            && self.ranked_status.is_load()
            && self.number_of_hitcircles.is_load()
            && self.number_of_sliders.is_load()
            && self.number_of_spinners.is_load()
            && self.last_modification_time.is_load()
            && self.approach_rate.is_load()
            && self.circle_size.is_load()
            && self.hp_drain.is_load()
            && self.overall_difficulty.is_load()
            && self.slider_velocity.is_load()
            && self.num_mod_combo_star_ratings_standard
            && self.mod_combo_star_ratings_standard
            && self.num_mod_combo_star_ratings_taiko
            && self.mod_combo_star_ratings_taiko
            && self.num_mod_combo_star_ratings_ctb
            && self.mod_combo_star_ratings_ctb
            && self.num_mod_combo_star_ratings_mania
            && self.mod_combo_star_ratings_mania
            && self.drain_time.is_load()
            && self.total_time.is_load()
            && self.preview_offset_from_start_ms.is_load()
            && self.num_timing_points.is_load()
            && self.timing_points
            && self.beatmap_id.is_load()
            && self.beatmap_set_id.is_load()
            && self.thread_id.is_load()
            && self.standard_grade.is_load()
            && self.taiko_grade.is_load()
            && self.ctb_grade.is_load()
            && self.mania_grade.is_load()
            && self.local_offset.is_load()
            && self.stack_leniency.is_load()
            && self.gameplay_mode.is_load()
            && self.song_source.is_load()
            && self.song_tags.is_load()
            && self.online_offset.is_load()
            && self.font_used_for_song_title.is_load()
            && self.unplayed.is_load()
            && self.last_played.is_load()
            && self.is_osz2.is_load()
            && self.beatmap_folder_name.is_load()
            && self.last_checked_against_repo.is_load()
            && self.ignore_beatmap_sound.is_load()
            && self.ignore_beatmap_skin.is_load()
            && self.disable_storyboard.is_load()
            && self.disable_video.is_load()
            && self.visual_override.is_load()
            && self.unknown_short
            && self.offset_from_song_start_in_editor_ms.is_load()
            && self.mania_scroll_speed.is_load()
    }

    pub fn ignore_all(&self) -> bool {
        self.entry_size.is_ignore()
            && self.artist_name.is_ignore()
            && self.artist_name_unicode.is_ignore()
            && self.song_title.is_ignore()
            && self.song_title_unicode.is_ignore()
            && self.creator_name.is_ignore()
            && self.difficulty.is_ignore()
            && self.audio_file_name.is_ignore()
            && self.md5_beatmap_hash.is_ignore()
            && self.dotosu_file_name.is_ignore()
            && self.ranked_status.is_ignore()
            && self.number_of_hitcircles.is_ignore()
            && self.number_of_sliders.is_ignore()
            && self.number_of_spinners.is_ignore()
            && self.last_modification_time.is_ignore()
            && self.approach_rate.is_ignore()
            && self.circle_size.is_ignore()
            && self.hp_drain.is_ignore()
            && self.overall_difficulty.is_ignore()
            && self.slider_velocity.is_ignore()
            && !self.num_mod_combo_star_ratings_standard
            && !self.mod_combo_star_ratings_standard
            && !self.num_mod_combo_star_ratings_taiko
            && !self.mod_combo_star_ratings_taiko
            && !self.num_mod_combo_star_ratings_ctb
            && !self.mod_combo_star_ratings_ctb
            && !self.num_mod_combo_star_ratings_mania
            && !self.mod_combo_star_ratings_mania
            && self.drain_time.is_ignore()
            && self.total_time.is_ignore()
            && self.preview_offset_from_start_ms.is_ignore()
            && self.num_timing_points.is_ignore()
            && !self.timing_points
            && self.beatmap_id.is_ignore()
            && self.beatmap_set_id.is_ignore()
            && self.thread_id.is_ignore()
            && self.standard_grade.is_ignore()
            && self.taiko_grade.is_ignore()
            && self.ctb_grade.is_ignore()
            && self.mania_grade.is_ignore()
            && self.local_offset.is_ignore()
            && self.stack_leniency.is_ignore()
            && self.gameplay_mode.is_ignore()
            && self.song_source.is_ignore()
            && self.song_tags.is_ignore()
            && self.online_offset.is_ignore()
            && self.font_used_for_song_title.is_ignore()
            && self.unplayed.is_ignore()
            && self.last_played.is_ignore()
            && self.is_osz2.is_ignore()
            && self.beatmap_folder_name.is_ignore()
            && self.last_checked_against_repo.is_ignore()
            && self.ignore_beatmap_sound.is_ignore()
            && self.ignore_beatmap_skin.is_ignore()
            && self.disable_storyboard.is_ignore()
            && self.disable_video.is_ignore()
            && self.visual_override.is_ignore()
            && !self.unknown_short
            && self.offset_from_song_start_in_editor_ms.is_ignore()
            && self.mania_scroll_speed.is_ignore()
    }

    pub fn is_partial(&self) -> bool {
        self.entry_size.is_ignore()
            || self.artist_name.is_ignore()
            || self.artist_name_unicode.is_ignore()
            || self.song_title.is_ignore()
            || self.song_title_unicode.is_ignore()
            || self.creator_name.is_ignore()
            || self.difficulty.is_ignore()
            || self.audio_file_name.is_ignore()
            || self.md5_beatmap_hash.is_ignore()
            || self.dotosu_file_name.is_ignore()
            || self.ranked_status.is_ignore()
            || self.number_of_hitcircles.is_ignore()
            || self.number_of_sliders.is_ignore()
            || self.number_of_spinners.is_ignore()
            || self.last_modification_time.is_ignore()
            || self.approach_rate.is_ignore()
            || self.circle_size.is_ignore()
            || self.hp_drain.is_ignore()
            || self.overall_difficulty.is_ignore()
            || self.slider_velocity.is_ignore()
            || !self.num_mod_combo_star_ratings_standard
            || !self.mod_combo_star_ratings_standard
            || !self.num_mod_combo_star_ratings_taiko
            || !self.mod_combo_star_ratings_taiko
            || !self.num_mod_combo_star_ratings_ctb
            || !self.mod_combo_star_ratings_ctb
            || !self.num_mod_combo_star_ratings_mania
            || !self.mod_combo_star_ratings_mania
            || self.drain_time.is_ignore()
            || self.total_time.is_ignore()
            || self.preview_offset_from_start_ms.is_ignore()
            || self.num_timing_points.is_ignore()
            || !self.timing_points
            || self.beatmap_id.is_ignore()
            || self.beatmap_set_id.is_ignore()
            || self.thread_id.is_ignore()
            || self.standard_grade.is_ignore()
            || self.taiko_grade.is_ignore()
            || self.ctb_grade.is_ignore()
            || self.mania_grade.is_ignore()
            || self.local_offset.is_ignore()
            || self.stack_leniency.is_ignore()
            || self.gameplay_mode.is_ignore()
            || self.song_source.is_ignore()
            || self.song_tags.is_ignore()
            || self.online_offset.is_ignore()
            || self.font_used_for_song_title.is_ignore()
            || self.unplayed.is_ignore()
            || self.last_played.is_ignore()
            || self.is_osz2.is_ignore()
            || self.beatmap_folder_name.is_ignore()
            || self.last_checked_against_repo.is_ignore()
            || self.ignore_beatmap_sound.is_ignore()
            || self.ignore_beatmap_skin.is_ignore()
            || self.disable_storyboard.is_ignore()
            || self.disable_video.is_ignore()
            || self.visual_override.is_ignore()
            || !self.unknown_short
            || self.offset_from_song_start_in_editor_ms.is_ignore()
            || self.mania_scroll_speed.is_ignore()
    }

    pub fn set_from_mask(&mut self, mask: &BeatmapMask) {
        self.entry_size.apply_mask(mask.entry_size);
        self.artist_name.apply_mask(mask.artist_name);
        self.artist_name_unicode
            .apply_mask(mask.artist_name_unicode);
        self.song_title.apply_mask(mask.song_title);
        self.song_title_unicode.apply_mask(mask.song_title_unicode);
        self.creator_name.apply_mask(mask.creator_name);
        self.difficulty.apply_mask(mask.difficulty);
        self.audio_file_name.apply_mask(mask.audio_file_name);
        self.md5_beatmap_hash.apply_mask(mask.md5_beatmap_hash);
        self.dotosu_file_name.apply_mask(mask.dotosu_file_name);
        self.ranked_status.apply_mask(mask.ranked_status);
        self.number_of_hitcircles
            .apply_mask(mask.number_of_hitcircles);
        self.number_of_sliders.apply_mask(mask.number_of_sliders);
        self.number_of_spinners.apply_mask(mask.number_of_spinners);
        self.last_modification_time
            .apply_mask(mask.last_modification_time);
        self.approach_rate.apply_mask(mask.approach_rate);
        self.circle_size.apply_mask(mask.circle_size);
        self.hp_drain.apply_mask(mask.hp_drain);
        self.overall_difficulty.apply_mask(mask.overall_difficulty);
        self.slider_velocity.apply_mask(mask.slider_velocity);
        self.num_mod_combo_star_ratings_standard |= mask.num_mod_combo_star_ratings_standard;
        self.mod_combo_star_ratings_standard |= mask.mod_combo_star_ratings_standard;
        self.num_mod_combo_star_ratings_taiko |= mask.num_mod_combo_star_ratings_taiko;
        self.mod_combo_star_ratings_taiko |= mask.mod_combo_star_ratings_taiko;
        self.num_mod_combo_star_ratings_ctb |= mask.num_mod_combo_star_ratings_ctb;
        self.mod_combo_star_ratings_ctb |= mask.mod_combo_star_ratings_ctb;
        self.num_mod_combo_star_ratings_mania |= mask.num_mod_combo_star_ratings_mania;
        self.mod_combo_star_ratings_mania |= mask.mod_combo_star_ratings_mania;
        self.drain_time.apply_mask(mask.drain_time);
        self.total_time.apply_mask(mask.total_time);
        self.preview_offset_from_start_ms
            .apply_mask(mask.preview_offset_from_start_ms);
        self.num_timing_points.apply_mask(mask.num_timing_points);
        self.timing_points |= mask.timing_points;
        self.beatmap_id.apply_mask(mask.beatmap_id);
        self.beatmap_set_id.apply_mask(mask.beatmap_set_id);
        self.thread_id.apply_mask(mask.thread_id);
        self.standard_grade.apply_mask(mask.standard_grade);
        self.taiko_grade.apply_mask(mask.taiko_grade);
        self.ctb_grade.apply_mask(mask.ctb_grade);
        self.mania_grade.apply_mask(mask.mania_grade);
        self.local_offset.apply_mask(mask.local_offset);
        self.stack_leniency.apply_mask(mask.stack_leniency);
        self.gameplay_mode.apply_mask(mask.gameplay_mode);
        self.song_source.apply_mask(mask.song_source);
        self.song_tags.apply_mask(mask.song_tags);
        self.online_offset.apply_mask(mask.online_offset);
        self.font_used_for_song_title
            .apply_mask(mask.font_used_for_song_title);
        self.unplayed.apply_mask(mask.unplayed);
        self.last_played.apply_mask(mask.last_played);
        self.is_osz2.apply_mask(mask.is_osz2);
        self.beatmap_folder_name
            .apply_mask(mask.beatmap_folder_name);
        self.last_checked_against_repo
            .apply_mask(mask.last_checked_against_repo);
        self.ignore_beatmap_sound
            .apply_mask(mask.ignore_beatmap_sound);
        self.ignore_beatmap_skin
            .apply_mask(mask.ignore_beatmap_skin);
        self.disable_storyboard.apply_mask(mask.disable_storyboard);
        self.disable_video.apply_mask(mask.disable_video);
        self.visual_override.apply_mask(mask.visual_override);
        self.unknown_short |= mask.unknown_short;
        self.offset_from_song_start_in_editor_ms
            .apply_mask(mask.offset_from_song_start_in_editor_ms);
        self.mania_scroll_speed.apply_mask(mask.mania_scroll_speed);
    }
}
