#![rustfmt::skip]
use clap::{App, AppSettings, Arg, ArgGroup, ArgMatches, SubCommand};

use crate::masks::mask::Mask;

const BEATMAP_FIELDS: [&str; 60] = [
    "Entry size",
    "Artist name",
    "Artist name unicode",
    "Song title",
    "Song title unicode",
    "Creator name",
    "Difficulty",
    "Audio file name",
    "MD5 beatmap hash",
    ".osu file name",
    "Ranked status",
    "Number of hitcircles",
    "Number of sliders",
    "Number of spinners",
    "Last modification time",
    "Approach rate",
    "Circle size",
    "HP drain",
    "Overall difficulty",
    "Slider velocity",
    "Number of precalculated mod combo star ratings (standard)",
    "Precalculated mod combo star ratings (standard)",
    "Number of precalculated mod combo star ratings (taiko)",
    "precalculated mod combo star ratings (taiko)",
    "Number of precalculated mod combo star ratings (CTB)",
    "precalculated mod combo star ratings (CTB)",
    "Number of precalculated mod combo star ratings (mania)",
    "precalculated mod combo star ratings (mania)",
    "Drain time",
    "Total time",
    "Preview offset from start (ms)",
    "Number of timing points",
    "Timing points",
    "Beatmap ID",
    "Beatmap set ID",
    "Thread ID",
    "Standard grade",
    "Taiko grade",
    "CTB grade",
    "Mania grade",
    "Local offset",
    "Stack leniency",
    "Gameplay mode",
    "Song source",
    "Song tags",
    "Online offset",
    "Font used for song title",
    "Unplayed",
    "Last played",
    "Is OSZ2",
    "Beatmap folder name",
    "Last checked against repo",
    "Ignore beatmap sound",
    "Ignore beatmap skin",
    "Disable storyboard",
    "Disable video",
    "Visual override",
    "Unknown short",
    "Offset from song start in editor (ms)",
    "Mania scroll speed",
];

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

    fn from_input(input: &str) -> Self {
        let matches = App::new("osu!.db beatmap entry show options parsing")
            .version("1.0.0")
            .author("Aurorans Solis")
            .about("Parser for show options for entries in osu!.db (beatmaps).")
            .arg(
                Arg::with_name("All")
                    .long("all")
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show all fields"),
            )
            .arg(
                Arg::with_name("None")
                    .long("none")
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show no fields"),
            )
            .arg(
                Arg::with_name("Entry size")
                    .long("entry-size")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show the size of each database entry"),
            )
            .arg(
                Arg::with_name("Artist name (ASCII)")
                    .long("artist-name")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show the artist name (ASCII)"),
            )
            .arg(
                Arg::with_name("Artist name (Unicode)")
                    .long("artist-name-unicode")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show the artist name (Unicode)"),
            )
            .arg(
                Arg::with_name("Song title (ASCII)")
                    .long("song-title")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show the song title (ASCII)"),
            )
            .arg(
                Arg::with_name("Song title (Unicode)")
                    .long("song-title-unicode")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show the song title (Unicode)"),
            )
            .arg(
                Arg::with_name("Creator name")
                    .long("creator-name")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show creator name"),
            )
            .arg(
                Arg::with_name("Difficulty")
                    .long("difficulty")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show the difficulty name"),
            )
            .arg(
                Arg::with_name("Audio file name")
                    .long("audio-file-name")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show the audio file name"),
            )
            .arg(
                Arg::with_name("MD5 beatmap hash")
                    .long("md5-beatmap-hash")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show MD5 beatmap hash"),
            )
            .arg(
                Arg::with_name(".osu file name")
                    .long("dotosu-file-name")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show .osu file name"),
            )
            .arg(
                Arg::with_name("Ranked status")
                    .long("ranked-status")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show ranked status"),
            )
            .arg(
                Arg::with_name("Number of hitcircles")
                    .long("number-of-hitcircles")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show number of hitcircles"),
            )
            .arg(
                Arg::with_name("Number of sliders")
                    .long("number-of-sliders")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show number of sliders"),
            )
            .arg(
                Arg::with_name("Number of spinners")
                    .long("number-of-spinners")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show number of spinners"),
            )
            .arg(
                Arg::with_name("Last modification time")
                    .long("last-modification-time")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show last modification time"),
            )
            .arg(
                Arg::with_name("Approach rate")
                    .short("ar")
                    .long("approach-rate")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show approach rate"),
            )
            .arg(
                Arg::with_name("Circle size")
                    .short("cs")
                    .long("circle-size")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show circle size"),
            )
            .arg(
                Arg::with_name("HP drain")
                    .short("hp")
                    .long("hp-drain")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show HP drain"),
            )
            .arg(
                Arg::with_name("Overall difficulty")
                    .short("od")
                    .long("overall-difficulty")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show overall difficulty"),
            )
            .arg(
                Arg::with_name("Slider velocity")
                    .long("slider-velocity")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show slider velocity"),
            )
            .arg(
                Arg::with_name("Number of precalculated mod combo star ratings (standard)")
                    .long("num-mod-combo-star-ratings-standard")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show number of precalculated mod combo star ratings (standard)"),
            )
            .arg(
                Arg::with_name("Precalculated mod combo star ratings (standard)")
                    .long("mod-combo-star-ratings-standard")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show precalculated mod combo star ratings (standard)"),
            )
            .arg(
                Arg::with_name("Number of precalculated mod combo star ratings (taiko)")
                    .long("num-mod-combo-star-ratings-taiko")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show number of precalculated mod combo star ratings (taiko)"),
            )
            .arg(
                Arg::with_name("Precalculated mod combo star ratings (taiko)")
                    .long("mod-combo-star-ratings-taiko")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show precalculated mod combo star ratings (taiko)"),
            )
            .arg(
                Arg::with_name("Number of precalculated mod combo star ratings (CTB)")
                    .long("num-mod-combo-star-ratings-ctb")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show number of precalculated mod combo star ratings (CTB)"),
            )
            .arg(
                Arg::with_name("Precalculated mod combo star ratings (CTB)")
                    .long("mod-combo-star-ratings-ctb")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show precalculated mod combo star ratings (CTB)"),
            )
            .arg(
                Arg::with_name("Number of precalculated mod combo star ratings (mania)")
                    .long("num-mod-combo-star-ratings-mania")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show number of precalculated mod combo star ratings (mania)"),
            )
            .arg(
                Arg::with_name("Precalculated mod combo star ratings (mania)")
                    .long("mod-combo-star-ratings-mania")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show precalculated mod combo star ratings (mania)"),
            )
            .arg(
                Arg::with_name("Drain time")
                    .long("drain-time")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show drain time"),
            )
            .arg(
                Arg::with_name("Total time")
                    .long("total-time")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show total time"),
            )
            .arg(
                Arg::with_name("Preview offset from start (ms)")
                    .long("preview-offset-from-start-ms")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show preview offset from start (ms)"),
            )
            .arg(
                Arg::with_name("Number of timing points")
                    .long("num-timing-points")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show the number of timing points"),
            )
            .arg(
                Arg::with_name("Timing points")
                    .long("timing-points")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show the timing points."),
            )
            .arg(
                Arg::with_name("Beatmap ID")
                    .long("beatmap-id")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show the beatmap ID"),
            )
            .arg(
                Arg::with_name("Beatmap set ID")
                    .long("beatmap-set-id")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show the beatmap set ID"),
            )
            .arg(
                Arg::with_name("Thread ID")
                    .long("thread-id")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show thread ID"),
            )
            .arg(
                Arg::with_name("Standard grade")
                    .long("standard-grade")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show standard grade"),
            )
            .arg(
                Arg::with_name("Taiko grade")
                    .long("taiko-grade")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show taiko grade"),
            )
            .arg(
                Arg::with_name("Catch the Beat grade")
                    .long("ctb-grade")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show Catch the Beat grade"),
            )
            .arg(
                Arg::with_name("Mania grade")
                    .long("mania-grade")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show mania grade"),
            )
            .arg(
                Arg::with_name("Local offset")
                    .long("local-offset")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show local offset"),
            )
            .arg(
                Arg::with_name("Stack leniency")
                    .long("stack-leniency")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show stack leniency"),
            )
            .arg(
                Arg::with_name("Gameplay mode")
                    .long("gameplay-mode")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show gameplay mode"),
            )
            .arg(
                Arg::with_name("Song source")
                    .long("song-source")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show song source"),
            )
            .arg(
                Arg::with_name("Song tags")
                    .long("song-tags")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show song tags"),
            )
            .arg(
                Arg::with_name("Online offset")
                    .long("online-offset")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show online offset"),
            )
            .arg(
                Arg::with_name("Font used for song title")
                    .long("font-used-for-song-title")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show font used for song title"),
            )
            .arg(
                Arg::with_name("Unplayed")
                    .long("unplayed")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show if the beatmap is unplayed"),
            )
            .arg(
                Arg::with_name("Last played")
                    .long("last-played")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show when the beatmap was last played"),
            )
            .arg(
                Arg::with_name("Is OSZ2")
                    .long("is-osz2")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show if the beatmap is OSZ2"),
            )
            .arg(
                Arg::with_name("Beatmap folder name")
                    .long("beatmap-folder-name")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show beatmap folder name"),
            )
            .arg(
                Arg::with_name("Last checked against repo")
                    .long("last-checked-against-repo")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show when the beatmap was last checked against the osu! repo"),
            )
            .arg(
                Arg::with_name("Ignore beatmap sound")
                    .long("ignore-beatmap-sound")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show whether ignoring beatmap hitsounds"),
            )
            .arg(
                Arg::with_name("Ignore beatmap skin")
                    .long("ignore-beatmap-skin")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show whether ignoring beatmap skin"),
            )
            .arg(
                Arg::with_name("Disable storyboard")
                    .long("disable-storyboard")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show whether storyboard is disabled"),
            )
            .arg(
                Arg::with_name("Visual override")
                    .long("visual-override")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show whether visual override is enabled"),
            )
            .arg(
                Arg::with_name("Unknown short")
                    .long("unknown-short")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help(
                        "Show unknown short (only present in beatmap versions 20140609 or older)",
                    ),
            )
            .arg(
                Arg::with_name("Offset from song start in editor (ms)")
                    .long("offset-from-song-start-in-editor-ms")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show offset from song start in editor (ms)"),
            )
            .arg(
                Arg::with_name("Mania scroll speed")
                    .long("mania-scroll-speed")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show mania scroll speed"),
            )
            .get_matches_from(input.split_ascii_whitespace());
        let [
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
            mania_scroll_speed,
        ] = if matches.is_present("All") {
            [true; 60]
        } else if matches.is_present("None") {
            [false; 60]
        } else {
            let entry_size = matches.is_present("Entry size");
            let artist_name = matches.is_present("Artist name (ASCII)");
            let artist_name_unicode = matches.is_present("Artist name (Unicode)");
            let song_title = matches.is_present("Song title (ASCII)");
            let song_title_unicode = matches.is_present("Song title (Unicode)");
            let creator_name = matches.is_present("Creator name");
            let difficulty = matches.is_present("Difficulty");
            let audio_file_name = matches.is_present("Audio file name");
            let md5_beatmap_hash = matches.is_present("MD5 beatmap hash");
            let dotosu_file_name = matches.is_present(".osu file name");
            let ranked_status = matches.is_present("Ranked status");
            let number_of_hitcircles = matches.is_present("Number of hitcircles");
            let number_of_sliders = matches.is_present("Number of sliders");
            let number_of_spinners = matches.is_present("Number of spinners");
            let last_modification_time = matches.is_present("Last modification time");
            let approach_rate = matches.is_present("Approach rate");
            let circle_size = matches.is_present("Circle size");
            let hp_drain = matches.is_present("HP drain");
            let overall_difficulty = matches.is_present("Overall difficulty");
            let slider_velocity = matches.is_present("Slider velocity");
            let num_mod_combo_star_ratings_standard =
                matches.is_present("Number of precalculated mod combo star ratings (standard)");
            let mod_combo_star_ratings_standard =
                matches.is_present("Precalculated mod combo star ratings (standard)");
            let num_mod_combo_star_ratings_taiko =
                matches.is_present("Number of precalculated mod combo star ratings (taiko)");
            let mod_combo_star_ratings_taiko =
                matches.is_present("Precalculated mod combo star ratings (taiko)");
            let num_mod_combo_star_ratings_ctb =
                matches.is_present("Number of precalculated mod combo star ratings (CTB)");
            let mod_combo_star_ratings_ctb =
                matches.is_present("Precalculated mod combo star ratings (CTB)");
            let num_mod_combo_star_ratings_mania =
                matches.is_present("Number of precalculated mod combo star ratings (mania)");
            let mod_combo_star_ratings_mania =
                matches.is_present("Precalculated mod combo star ratings (mania)");
            let drain_time = matches.is_present("Drain time");
            let total_time = matches.is_present("Total time");
            let preview_offset_from_start_ms =
                matches.is_present("Preview offset from start (ms)");
            let num_timing_points = matches.is_present("Number of timing points");
            let timing_points = matches.is_present("Timing points");
            let beatmap_id = matches.is_present("Beatmap ID");
            let beatmap_set_id = matches.is_present("Beatmap set ID");
            let thread_id = matches.is_present("Thread ID");
            let standard_grade = matches.is_present("Standard grade");
            let taiko_grade = matches.is_present("Taiko grade");
            let ctb_grade = matches.is_present("Catch the Beat grade");
            let mania_grade = matches.is_present("Mania grade");
            let local_offset = matches.is_present("Local offset");
            let stack_leniency = matches.is_present("Stack leniency");
            let gameplay_mode = matches.is_present("Gameplay mode");
            let song_source = matches.is_present("Song source");
            let song_tags = matches.is_present("Song tags");
            let online_offset = matches.is_present("Online offset");
            let font_used_for_song_title = matches.is_present("Font used for song title");
            let unplayed = matches.is_present("Unplayed");
            let last_played = matches.is_present("Last played");
            let is_osz2 = matches.is_present("Is OSZ2");
            let beatmap_folder_name = matches.is_present("Beatmap folder name");
            let last_checked_against_repo = matches.is_present("Last checked against repo");
            let ignore_beatmap_sound = matches.is_present("Ignore beatmap sound");
            let ignore_beatmap_skin = matches.is_present("Ignore beatmap skin");
            let disable_storyboard = matches.is_present("Disable storyboard");
            let disable_video = matches.is_present("Disable video");
            let visual_override = matches.is_present("Visual override");
            let unknown_short = matches.is_present("Unknown short");
            let offset_from_song_start_in_editor_ms =
                matches.is_present("Offset from song start in editor (ms)");
            let mania_scroll_speed = matches.is_present("Mania scroll speed");
            [
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
                mania_scroll_speed,
            ]
        };
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
            mania_scroll_speed,
        }
    }
}

impl Default for BeatmapMask {
    fn default() -> Self {
        BeatmapMask {
            entry_size: false,
            artist_name: false,
            artist_name_unicode: true,
            song_title: false,
            song_title_unicode: true,
            creator_name: true,
            difficulty: true,
            audio_file_name: false,
            md5_beatmap_hash: false,
            dotosu_file_name: false,
            ranked_status: true,
            number_of_hitcircles: true,
            number_of_sliders: true,
            number_of_spinners: true,
            last_modification_time: false,
            approach_rate: true,
            circle_size: true,
            hp_drain: true,
            overall_difficulty: true,
            slider_velocity: true,
            num_mod_combo_star_ratings_standard: false,
            mod_combo_star_ratings_standard: false,
            num_mod_combo_star_ratings_taiko: false,
            mod_combo_star_ratings_taiko: false,
            num_mod_combo_star_ratings_ctb: false,
            mod_combo_star_ratings_ctb: false,
            num_mod_combo_star_ratings_mania: false,
            mod_combo_star_ratings_mania: false,
            drain_time: true,
            total_time: true,
            preview_offset_from_start_ms: false,
            num_timing_points: false,
            timing_points: false,
            beatmap_id: false,
            beatmap_set_id: false,
            thread_id: false,
            standard_grade: true,
            taiko_grade: true,
            ctb_grade: true,
            mania_grade: true,
            local_offset: false,
            stack_leniency: false,
            gameplay_mode: true,
            song_source: false,
            song_tags: true,
            online_offset: false,
            font_used_for_song_title: false,
            unplayed: true,
            last_played: true,
            is_osz2: false,
            beatmap_folder_name: false,
            last_checked_against_repo: false,
            ignore_beatmap_sound: false,
            ignore_beatmap_skin: false,
            disable_storyboard: false,
            disable_video: false,
            visual_override: false,
            unknown_short: false,
            offset_from_song_start_in_editor_ms: false,
            mania_scroll_speed: false,
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
    pub beatmap_mask: BeatmapMask,
    pub unknown_short: bool,
}

impl OsuDbMask {
    pub fn new(
        version: bool,
        folder_count: bool,
        account_unlocked: bool,
        account_unlock_date: bool,
        player_name: bool,
        number_of_beatmaps: bool,
        beatmap_mask: BeatmapMask,
        unknown_short: bool,
    ) -> Self {
        OsuDbMask {
            version,
            folder_count,
            account_unlocked,
            account_unlock_date,
            player_name,
            number_of_beatmaps,
            beatmap_mask,
            unknown_short,
        }
    }
}

impl Mask for OsuDbMask {
    fn ignore_all(&self) -> bool {
        !self.version
            && !self.folder_count
            && !self.account_unlocked
            && !self.account_unlock_date
            && !self.player_name
            && !self.number_of_beatmaps
            && !self.unknown_short
            && self.beatmap_mask.ignore_all()
    }

    fn is_complete(&self) -> bool {
        self.version
            && self.folder_count
            && self.account_unlocked
            && self.account_unlock_date
            && self.player_name
            && self.number_of_beatmaps
            && self.unknown_short
            && self.beatmap_mask.is_complete()
    }

    fn from_input(input: &str) -> Self {
        let matches = App::new("osu!.db show options parser")
            .version("1.0.0")
            .author("Aurorans Solis")
            .about("Parser for show options for osu!.db")
            .arg(
                Arg::with_name("All")
                    .long("all")
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show all fields"),
            )
            .arg(
                Arg::with_name("None")
                    .long("none")
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show no fields"),
            )
            .arg(
                Arg::with_name("Version")
                    .long("version")
                    .conflicts_with_all(&["All", "None"])
                    .conflicts_with_all(&["All", "None"])
                    .required(false)
                    .takes_value(false)
                    .multiple(false)
                    .help("Show osu!.db version"),
            )
            .arg(
                Arg::with_name("Folder count")
                    .long("folder-count")
                    .conflicts_with_all(&["All", "None"])
                    .required(false)
                    .takes_value(false)
                    .multiple(false)
                    .help("Show folder count"),
            )
            .arg(
                Arg::with_name("Account unlocked")
                    .long("account-unlocked")
                    .conflicts_with_all(&["All", "None"])
                    .required(false)
                    .takes_value(false)
                    .multiple(false)
                    .help("Show whether account is unlocked"),
            )
            .arg(
                Arg::with_name("Account unlock date")
                    .long("account-unlock-date")
                    .conflicts_with_all(&["All", "None"])
                    .required(false)
                    .takes_value(false)
                    .multiple(false)
                    .help("Show account unlock date (Unix epoch if unlocked)"),
            )
            .arg(
                Arg::with_name("Player name")
                    .long("player-name")
                    .conflicts_with_all(&["All", "None"])
                    .required(false)
                    .takes_value(false)
                    .multiple(false)
                    .help("Show player name"),
            )
            .arg(
                Arg::with_name("Number of beatmaps")
                    .long("number-of-beatmaps")
                    .conflicts_with_all(&["All", "None"])
                    .required(false)
                    .takes_value(false)
                    .multiple(false)
                    .help("Show number of beatmaps"),
            )
            .arg(
                Arg::with_name("Beatmap show options")
                    .long("beatmap-show-options")
                    .required(false)
                    .takes_value(true)
                    .value_name("SHOW_OPTIONS")
                    .multiple(false)
                    .help("Beatmap show options"),
            )
            .arg(
                Arg::with_name("Unknown int")
                    .long("unknown-int")
                    .conflicts_with_all(&["All", "None"])
                    .required(false)
                    .takes_value(false)
                    .multiple(false)
                    .help("Show unknown int"),
            )
            .get_matches_from(input.split_ascii_whitespace());
        let [
            version,
            folder_count,
            account_unlocked,
            account_unlock_date,
            player_name,
            number_of_beatmaps,
            unknown_short
        ] = if matches.is_present("All") {
            [true; 7]
        } else if matches.is_present("None") {
            [false; 7]
        } else {
            let version = matches.is_present("Version");
            let folder_count = matches.is_present("Folder count");
            let account_unlocked = matches.is_present("Account unlocked");
            let account_unlock_date = matches.is_present("Account unlock date");
            let player_name = matches.is_present("Player name");
            let number_of_beatmaps = matches.is_present("Number of beatmaps");
            [
                version,
                folder_count,
                account_unlocked,
                account_unlock_date,
                player_name,
                number_of_beatmaps,
                unknown_short,
            ]
        };
        let meatmap_mask = if let Some(m) = matches.value_of("Beatmap show options") {
            BeatmapMask::from_input(m)
        } else {
            BeatmapMask::default()
        };
        let unknown_short = matches.is_present("Unknown int");
        OsuDbMask {
            version,
            folder_count,
            account_unlocked,
            account_unlock_date,
            player_name,
            number_of_beatmaps,
            beatmap_mask,
            unknown_short,
        }
    }
}
