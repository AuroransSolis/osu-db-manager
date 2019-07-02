use clap::{Arg, App, SubCommand, AppSettings, ArgGroup};

use crate::masks::mask::Mask;

const BEATMAP_FIELDS: [&str; 55] = ["Entry size", "Artist name", "Artist name unicode",
    "Song title", "Song title unicode", "Creator name", "Difficulty", "Audio file name",
    "MD5 beatmap hash", ".osu file name", "Ranked status", "Number of hitcircles",
    "Number of sliders", "Number of spinners", "Last modification time", "Approach rate",
    "Circle size", "HP drain", "Overall difficulty", "Slider velocity",
    "Number of precalculated mod combo star ratings (standard)",
    "Number of precalculated mod combo star ratings (taiko)",
    "Number of precalculated mod combo star ratings (CTB)",
    "Number of precalculated mod combo star ratings (mania)", "Drain time", "Total time",
    "Preview offset from start (ms)", "Number of timing points", "Beatmap ID", "Beatmap set ID",
    "Thread ID", "Standard grade", "Taiko grade", "CTB grade", "Mania grade", "Local offset",
    "Stack leniency", "Gameplay mode", "Song source", "Song tags", "Online offset",
    "Font used for song title", "Unplayed", "Last played", "Is OSZ2", "Beatmap folder name",
    "Last checked against repo", "Ignore beatmap sound", "Ignore beatmap skin",
    "Disable storyboard", "Disable video", "Visual override", "Unknown short",
    "Offset from song start in editor (ms)", "Mania scroll speed"];

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

    fn from_show_args(show_args: Vec<&str>) -> Self {
        let matches = App::new("osu!.db beatmap show options parser")
            .arg(Arg::with_name("Entry size")
                .long("ENTRY-SIZE")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name("Artist name")
                .long("ARTIST-NAME")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name("Artist name unicode")
                .long("ARTIST-NAME-UNICODE")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name("Song title")
                .long("SONG-TITLE")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name("Song title unicode")
                .long("SONG-TITLE-UNICODE")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name("Creator name")
                .long("CREATOR-NAME")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name("Difficulty")
                .long("DIFFICULTY")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name("Audio file name")
                .long("AUDIO-FILE-NAME")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name("MD5 beatmap hash")
                .long("MD5-BEATMAP-HASH")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name(".osu file name")
                .long("DOTOSU-FILE-NAME")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name("Ranked status")
                .long("RANKED-STATUS")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name("Number of hitcircles")
                .long("NUMBER-OF-HITCIRCLES")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name("Number of sliders")
                .long("NUMBER-OF-SLIDERS")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name("Number of spinners")
                .long("NUMBER-OF-SPINNERS")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name("Last modification time")
                .long("LAST-MODIFICATION-TIME")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name("Approach rate")
                .long("APPROACH-RATE")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name("Circle size")
                .long("CIRCLE-SIZE")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name("HP drain")
                .long("HP-DRAIN")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name("Overall difficulty")
                .long("OVERALL-DIFFICULTY")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name("Slider velocity")
                .long("SLIDER-VELOCITY")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name("Number of precalculated mod combo star ratings (standard)")
                .long("NUM-MOD-COMBO-STAR-RATINGS-STANDARD")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name("Number of precalculated mod combo star ratings (taiko)")
                .long("NUM-MOD-COMBO-STAR-RATINGS-TAIKO")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name("Number of precalculated mod combo star ratings (CTB)")
                .long("NUM-MOD-COMBO-STAR-RATINGS-CTB")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name("Number of precalculated mod combo star ratings (mania)")
                .long("NUM-MOD-COMBO-STAR-RATINGS-MANIA")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name("Drain time")
                .long("DRAIN-TIME")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name("Total time")
                .long("TOTAL-TIME")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name("Preview offset from start (ms)")
                .long("PREVIEW-OFFSET-FROM-START-MS")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name("Number of timing points")
                .long("NUM-TIMING-POINTS")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name("Beatmap ID")
                .long("BEATMAP-ID")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name("Beatmap set ID")
                .long("BEATMAP-SET-ID")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name("Thread ID")
                .long("THREAD-ID")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name("Standard grade")
                .long("STANDARD-GRADE")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name("Taiko grade")
                .long("TAIKO-GRADE")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name("CTB grade")
                .long("CTB-GRADE")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name("Mania grade")
                .long("MANIA-GRADE")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name("Local offset")
                .long("LOCAL-OFFSET")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name("Stack leniency")
                .long("STACK-LENIENCY")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name("Gameplay mode")
                .long("GAMEPLAY-MODE")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name("Song source")
                .long("SONG-SOURCE")
                .multiple(false)
                .required(false)
                .takes_value(true)
                .number_of_values(1)
                .value_name("SOURCE")
                .takes_value(false))
            .arg(Arg::with_name("Song tags")
                .long("SONG-TAGS")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name("Online offset")
                .long("ONLINE-OFFSET")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name("Font used for song title")
                .long("FONT-USED-FOR-SONG-TITLE")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name("Unplayed")
                .long("UNPLAYED")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name("Last played")
                .long("LAST-PLAYED")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name("Is OSZ2")
                .long("IS-OSZ2")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name("Beatmap folder name")
                .long("BEATMAP-FOLDER-NAME")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name("Last checked against repo")
                .long("LAST-CHECKED-AGAINST-REPO")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name("Ignore beatmap sound")
                .long("IGNORE-BEATMAP-SOUND")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name("Ignore beatmap skin")
                .long("IGNORE-BEATMAP-SKIN")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name("Disable storyboard")
                .long("DISABLE-STORYBOARD")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name("Disable video")
                .long("DISABLE-VIDEO")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name("Visual override")
                .long("VISUAL-OVERRIDE")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name("Unknown short")
                .long("UNKNOWN-SHORT")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name("Offset from song start in editor (ms)")
                .long("OFFSET-FROM-SONG-START-IN-EDITOR-MS")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name("Mania scroll speed")
                .long("MANIA-SCROLL-SPEED")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .get_matches_from(show_args.into_iter());
        let entry_size = matches.is_present("Entry size");
        let artist_name = matches.is_present("Artist name");
        let artist_name_unicode = matches.is_present("Artist name unicode");
        let song_title = matches.is_present("Song title");
        let song_title_unicode = matches.is_present("Song title unicode");
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
        let num_mod_combo_star_ratings_standard = matches.is_present("Number of precalculated mod combo star ratings (standard)");
        let num_mod_combo_star_ratings_taiko = matches.is_present("Number of precalculated mod combo star ratings (taiko)");
        let num_mod_combo_star_ratings_ctb = matches.is_present("Number of precalculated mod combo star ratings (CTB)");
        let num_mod_combo_star_ratings_mania = matches.is_present("Number of precalculated mod combo star ratings (mania)");
        let drain_time = matches.is_present("Drain time");
        let total_time = matches.is_present("Total time");
        let preview_offset_from_start_ms = matches.is_present("Preview offset from start (ms)");
        let num_timing_points = matches.is_present("Number of timing points");
        let beatmap_id = matches.is_present("Beatmap ID");
        let beatmap_set_id = matches.is_present("Beatmap set ID");
        let thread_id = matches.is_present("Thread ID");
        let standard_grade = matches.is_present("Standard grade");
        let taiko_grade = matches.is_present("Taiko grade");
        let ctb_grade = matches.is_present("CTB grade");
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
        let offset_from_song_start_in_editor_ms = matches.is_present("Offset from song start in editor (ms)");
        let mania_scroll_speed = matches.is_present("Mania scroll speed");
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

    fn from_show_args(show_args: Vec<&str>) -> Self {
        let matches = App::new("osu!.db show options parser")
            .arg(Arg::with_name("Version")
                .long("VERSION")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name("Folder count")
                .long("FOLDER-COUNT")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name("Account unlocked")
                .long("ACCOUNT-UNLOCKED")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name("Account unlock date")
                .long("ACCOUNT-UNLOCK-DATE")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name("Player name")
                .long("PLAYER-NAME")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name("Number of beatmaps")
                .long("NUMBER-OF-BEATMAPS")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .arg(Arg::with_name("Beatmap show options")
                .long("BEATMAP-OPTIONS")
                .required(false)
                .multiple(false)
                .takes_value(true)
                .number_of_values(1)
                .value_name("OPTIONS"))
            .arg(Arg::with_name("Unknown int")
                .long("UNKNOWN-INT")
                .required(false)
                .takes_value(false)
                .multiple(false))
            .get_matches_from(show_args.into_iter());
        let version = matches.is_present("Version");
        let folder_count = matches.is_present("Folder count");
        let account_unlocked = matches.is_present("Account unlocked");
        let account_unlock_date = matches.is_present("Account unlock date");
        let player_name = matches.is_present("Player name");
        let number_of_beatmaps = matches.is_present("Number of beatmaps");
        let beatmap_mask = if let Some(beatmap_options) = matches.value_of("Beatmap show options") {
            let beatmap_show_args = beatmap_options.split_whitespace().collect::<Vec<_>>();
            Some(BeatmapMask::from_show_args(beatmap_show_args))
        } else {
            None
        };
        let unknown_int = matches.is_present("Unknown int");
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