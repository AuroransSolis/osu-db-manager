use clap::{App, Arg};

use crate::masks::mask::Mask;

#[derive(Copy, Clone, Debug)]
pub struct ScoreMask {
    pub gameplay_mode: bool,
    pub score_version: bool,
    pub md5_beatmap_hash: bool,
    pub player_name: bool,
    pub md5_replay_hash: bool,
    pub number_of_300s: bool,
    pub number_of_100s: bool,  // 150s in Taiko, 100s in CTB, 200s in Mania
    pub number_of_50s: bool,   // small fruit in CTB, 50s in Mania
    pub number_of_gekis: bool, // max 300s in Mania
    pub number_of_katus: bool, // 100s in mania
    pub number_of_misses: bool,
    pub replay_score: bool,
    pub max_combo: bool,
    pub perfect_combo: bool,
    pub mods_used: bool,
    pub empty_string: bool,
    pub replay_timestamp: bool,
    pub negative_one: bool,
    pub online_score_id: bool,
}

impl ScoreMask {
    pub fn new(
        gameplay_mode: bool,
        score_version: bool,
        md5_beatmap_hash: bool,
        player_name: bool,
        md5_replay_hash: bool,
        number_of_300s: bool,
        number_of_100s: bool,
        number_of_50s: bool,
        number_of_gekis: bool,
        number_of_katus: bool,
        number_of_misses: bool,
        replay_score: bool,
        max_combo: bool,
        perfect_combo: bool,
        mods_used: bool,
        empty_string: bool,
        replay_timestamp: bool,
        negative_one: bool,
        online_score_id: bool,
    ) -> Self {
        ScoreMask {
            gameplay_mode,
            score_version,
            md5_beatmap_hash,
            player_name,
            md5_replay_hash,
            number_of_300s,
            number_of_100s,
            number_of_50s,
            number_of_gekis,
            number_of_katus,
            number_of_misses,
            replay_score,
            max_combo,
            perfect_combo,
            mods_used,
            empty_string,
            replay_timestamp,
            negative_one,
            online_score_id,
        }
    }

    pub fn ignore_all(&self) -> bool {
        !self.gameplay_mode
            && !self.score_version
            && !self.md5_beatmap_hash
            && !self.player_name
            && !self.md5_replay_hash
            && !self.number_of_300s
            && !self.number_of_100s
            && !self.number_of_50s
            && !self.number_of_gekis
            && !self.number_of_katus
            && !self.number_of_misses
            && !self.replay_score
            && !self.max_combo
            && !self.perfect_combo
            && !self.mods_used
            && !self.empty_string
            && !self.replay_timestamp
            && !self.negative_one
            && !self.online_score_id
    }

    fn is_complete(&self) -> bool {
        self.gameplay_mode
            && self.score_version
            && self.md5_beatmap_hash
            && self.player_name
            && self.md5_replay_hash
            && self.number_of_300s
            && self.number_of_100s
            && self.number_of_50s
            && self.number_of_gekis
            && self.number_of_katus
            && self.number_of_misses
            && self.replay_score
            && self.max_combo
            && self.perfect_combo
            && self.mods_used
            && self.empty_string
            && self.replay_timestamp
            && self.negative_one
            && self.online_score_id
    }

    fn from_input(input: &str) -> Self {
        let matches = App::new("scores.db beatmap score show options parsing")
            .version("1.0.0")
            .author("Aurorans Solis")
            .about("Parser for show options for scores.db beatmap entries (scores)")
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
                Arg::with_name("Gameplay mode")
                    .long("gameplay-mode")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show gameplay mode"),
            )
            .arg(
                Arg::with_name("Score version")
                    .long("score-version")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show score version"),
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
                Arg::with_name("Player name")
                    .long("player-name")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show player name"),
            )
            .arg(
                Arg::with_name("MD5 replay hash")
                    .long("md5-replay-hash")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show MD5 replay hash"),
            )
            .arg(
                Arg::with_name("Number of 300s")
                    .long("number-of-300s")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show number of 300s"),
            )
            .arg(
                Arg::with_name("Number of 100s")
                    .long("number-of-100s")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show number of 100s"),
            )
            .arg(
                Arg::with_name("Number of 50s")
                    .long("number-of-50s")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show number of 50s"),
            )
            .arg(
                Arg::with_name("Number of gekis")
                    .long("number-of-gekis")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show number of gekis"),
            )
            .arg(
                Arg::with_name("Number of katus")
                    .long("number-of-katus")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show number of katus"),
            )
            .arg(
                Arg::with_name("Number of misses")
                    .long("number-of-misses")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show number of misses"),
            )
            .arg(
                Arg::with_name("Replay score")
                    .long("replay-score")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show replay score"),
            )
            .arg(
                Arg::with_name("Max combo")
                    .long("max-combo")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show max combo"),
            )
            .arg(
                Arg::with_name("Perfect combo")
                    .long("perfect-combo")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show whether a perfect combo was achieved"),
            )
            .arg(
                Arg::with_name("Mods used")
                    .long("mods-used")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show which mods were used"),
            )
            .arg(
                Arg::with_name("Empty string")
                    .long("empty-string")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show empty string"),
            )
            .arg(
                Arg::with_name("Replay timestamp")
                    .long("replay-timestamp")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show replay timestamp"),
            )
            .arg(
                Arg::with_name("Negative one")
                    .long("negative-one")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show negative one"),
            )
            .arg(
                Arg::with_name("Online score ID")
                    .long("online-score-id")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show online score ID"),
            )
            .get_matches_from(input.split_ascii_whitespace());
        let [
            gameplay_mode,
            score_version,
            md5_beatmap_hash,
            player_name,
            md5_replay_hash,
            number_of_300s,
            number_of_100s,
            number_of_50s,
            number_of_gekis,
            number_of_katus,
            number_of_misses,
            replay_score,
            max_combo,
            perfect_combo,
            mods_used,
            empty_string,
            replay_timestamp,
            negative_one,
            online_score_id,
        ] =
            if matches.is_present("All") {
                [true; 19]
            } else if matches.is_present("None") {
                [false; 19]
            } else {
                let gameplay_mode = matches.is_present("Gameplay mode");
                let score_version = matches.is_present("Score version");
                let md5_beatmap_hash = matches.is_present("MD5 beatmap hash");
                let player_name = matches.is_present("Player name");
                let md5_replay_hash = matches.is_present("MD5 replay hash");
                let number_of_300s = matches.is_present("Number of 300s");
                let number_of_100s = matches.is_present("Number of 100s");
                let number_of_50s = matches.is_present("Number of 50s");
                let number_of_gekis = matches.is_present("Number of gekis");
                let number_of_katus = matches.is_present("Number of katus");
                let number_of_misses = matches.is_present("Number of misses");
                let replay_score = matches.is_present("Replay score");
                let max_combo = matches.is_present("Max combo");
                let perfect_combo = matches.is_present("Perfect combo");
                let mods_used = matches.is_present("Mods used");
                let empty_string = matches.is_present("Empty string");
                let replay_timestamp = matches.is_present("Replay timestamp");
                let negative_one = matches.is_present("Negative one");
                let online_score_id = matches.is_present("Online score ID");
                [
                    gameplay_mode,
                    score_version,
                    md5_beatmap_hash,
                    player_name,
                    md5_replay_hash,
                    number_of_300s,
                    number_of_100s,
                    number_of_50s,
                    number_of_gekis,
                    number_of_katus,
                    number_of_misses,
                    replay_score,
                    max_combo,
                    perfect_combo,
                    mods_used,
                    empty_string,
                    replay_timestamp,
                    negative_one,
                    online_score_id,
                ]
            };
        ScoreMask {
            gameplay_mode,
            score_version,
            md5_beatmap_hash,
            player_name,
            md5_replay_hash,
            number_of_300s,
            number_of_100s,
            number_of_50s,
            number_of_gekis,
            number_of_katus,
            number_of_misses,
            replay_score,
            max_combo,
            perfect_combo,
            mods_used,
            empty_string,
            replay_timestamp,
            negative_one,
            online_score_id,
        }
    }
}

impl Default for ScoreMask {
    fn default() -> Self {
        ScoreMask {
            gameplay_mode: true,
            score_version: true,
            md5_beatmap_hash: true,
            player_name: true,
            md5_replay_hash: true,
            number_of_300s: true,
            number_of_100s: true,
            number_of_50s: true,
            number_of_gekis: true,
            number_of_katus: true,
            number_of_misses: true,
            replay_score: true,
            max_combo: true,
            perfect_combo: true,
            mods_used: true,
            empty_string: false,
            replay_timestamp: true,
            negative_one: false,
            online_score_id: false,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ScoresDbBeatmapMask {
    pub md5_beatmap_hash: bool,
    pub number_of_scores: bool,
    pub scores_mask: ScoreMask,
}

impl ScoresDbBeatmapMask {
    pub fn new(md5_beatmap_hash: bool, number_of_scores: bool, scores_mask: ScoreMask) -> Self {
        ScoresDbBeatmapMask {
            md5_beatmap_hash,
            number_of_scores,
            scores_mask,
        }
    }
}

impl ScoresDbBeatmapMask {
    pub fn ignore_all(&self) -> bool {
        !self.md5_beatmap_hash && !self.number_of_scores && self.scores_mask.ignore_all()
    }

    fn is_complete(&self) -> bool {
        self.md5_beatmap_hash && self.number_of_scores && self.scores_mask.is_complete()
    }

    fn from_input(input: &str) -> Self {
        let matches = App::new("scores.db beatmap entry show options parsing")
            .version("1.0.0")
            .author("Aurorans Solis")
            .about("Parser for show options for scores.db entries (beatmaps)")
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
                Arg::with_name("MD5 beatmap hash")
                    .long("md5-beatmap-hash")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show MD5 beatmap hash"),
            )
            .arg(
                Arg::with_name("Number of scores")
                    .long("num-scores")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show number of scores"),
            )
            .arg(
                Arg::with_name("Score show options")
                    .long("score-show-options")
                    .takes_value(true)
                    .value_name("SHOW_OPTIONS")
                    .required(false)
                    .multiple(false)
                    .help("Show options for score entries"),
            )
            .get_matches_from(input.split_ascii_whitespace());
        let [md5_beatmap_hash, number_of_scores] = if matches.is_present("All") {
            [true; 2]
        } else if matches.is_present("None") {
            [false; 2]
        } else {
            let md5_beatmap_hash = matches.is_present("MD5 beatmap hash");
            let number_of_scores = matches.is_present("Number of scores");
            [md5_beatmap_hash, number_of_scores]
        };
        let scores_mask = if let Some(show_options) = matches.value_of("Score show options") {
            ScoreMask::from_input(show_options)
        } else {
            ScoreMask::default()
        };
        ScoresDbBeatmapMask {
            md5_beatmap_hash,
            number_of_scores,
            scores_mask,
        }
    }
}

impl Default for ScoresDbBeatmapMask {
    fn default() -> Self {
        ScoresDbBeatmapMask {
            md5_beatmap_hash: true,
            number_of_scores: false,
            scores_mask: ScoreMask::default(),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ScoresDbMask {
    pub version: bool,
    pub number_of_beatmaps: bool,
    pub beatmaps_mask: ScoresDbBeatmapMask,
}

impl ScoresDbMask {
    pub fn new(
        version: bool,
        number_of_beatmaps: bool,
        beatmaps_mask: ScoresDbBeatmapMask,
    ) -> Self {
        ScoresDbMask {
            version,
            number_of_beatmaps,
            beatmaps_mask,
        }
    }
}

impl Mask for ScoresDbMask {
    fn ignore_all(&self) -> bool {
        !self.version && !self.number_of_beatmaps && self.beatmaps_mask.ignore_all()
    }

    fn is_complete(&self) -> bool {
        self.version && self.number_of_beatmaps && self.beatmaps_mask.is_complete()
    }

    fn from_input(input: &str) -> Self {
        let matches = App::new("scores.db show options parsing")
            .version("1.0.0")
            .author("Aurorans Solis")
            .about("Parser for show options for scores.db")
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
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show scores.db version"),
            )
            .arg(
                Arg::with_name("Number of beatmaps")
                    .long("number-of-beatmaps")
                    .conflicts_with_all(&["All", "None"])
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show number of beatmaps"),
            )
            .arg(
                Arg::with_name("scores.db beatmap show options")
                    .long("scoredb-beatmap-show-options")
                    .takes_value(true)
                    .value_name("SHOW_OPTIONS")
                    .required(false)
                    .multiple(false)
                    .help("Show options for for scores.db entries (beatmaps)"),
            )
            .get_matches_from(input.split_ascii_whitespace());
        let [version, number_of_beatmaps] = if matches.is_present("All") {
            [true; 2]
        } else if matches.is_present("None") {
            [false; 2]
        } else {
            let version = matches.is_present("Version");
            let number_of_beatmaps = matches.is_present("Number of beatmaps");
            [version, number_of_beatmaps]
        };
        let beatmaps_mask =
            if let Some(show_options) = matches.value_of("scores.db beatmap show options") {
                ScoresDbBeatmapMask::from_input(show_options)
            } else {
                ScoresDbBeatmapMask::default()
            };
        ScoresDbMask {
            version,
            number_of_beatmaps,
            beatmaps_mask,
        }
    }
}

impl Default for ScoresDbMask {
    fn default() -> Self {
        ScoresDbMask {
            version: true,
            number_of_beatmaps: true,
            beatmaps_mask: ScoresDbBeatmapMask::default(),
        }
    }
}
