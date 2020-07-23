use structopt::StructOpt;

#[derive(Copy, Clone, Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct ScoreMask {
    #[structopt(long = "show-gameplay-mode")]
    pub gameplay_mode: bool,
    #[structopt(long = "show-score-version")]
    pub score_version: bool,
    #[structopt(long = "show-score-md5-beatmap-hash")]
    pub score_md5_beatmap_hash: bool,
    #[structopt(long = "show-player-name")]
    pub player_name: bool,
    #[structopt(long = "show-md5-replay-hash")]
    pub md5_replay_hash: bool,
    #[structopt(long = "show-number-of-300s")]
    pub number_of_300s: bool,
    #[structopt(long = "show-number-of-100s")]
    pub number_of_100s: bool, // 150s in Taiko, 100s in CTB, 200s in Mania
    #[structopt(long = "show-number-of-50s")]
    pub number_of_50s: bool, // small fruit in CTB, 50s in Mania
    #[structopt(long = "show-number-of-gekis")]
    pub number_of_gekis: bool, // max 300s in Mania
    #[structopt(long = "show-number-of-katus")]
    pub number_of_katus: bool, // 100s in mania
    #[structopt(long = "show-number-of-misses")]
    pub number_of_misses: bool,
    #[structopt(long = "show-replay-score")]
    pub replay_score: bool,
    #[structopt(long = "show-max-combo")]
    pub max_combo: bool,
    #[structopt(long = "show-perfect-combo")]
    pub perfect_combo: bool,
    #[structopt(long = "show-mods-used")]
    pub mods_used: bool,
    #[structopt(long = "show-empty-string")]
    pub empty_string: bool,
    #[structopt(long = "show-replay-timestamp")]
    pub replay_timestamp: bool,
    #[structopt(long = "show-negative-one")]
    pub negative_one: bool,
    #[structopt(long = "show-online-score-id")]
    pub online_score_id: bool,
}

impl ScoreMask {
    pub fn ignore_all(&self) -> bool {
        !self.gameplay_mode
            && !self.score_version
            && !self.score_md5_beatmap_hash
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
            && self.score_md5_beatmap_hash
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
}

#[derive(Copy, Clone, Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct ScoresDbBeatmapMask {
    #[structopt(long = "show-md5-beatmap-hash")]
    pub md5_beatmap_hash: bool,
    #[structopt(long = "show-number-of-scores")]
    pub number_of_scores: bool,
    #[structopt(flatten)]
    pub scores_mask: ScoreMask,
}

impl ScoresDbBeatmapMask {
    pub fn ignore_all(&self) -> bool {
        !self.md5_beatmap_hash && !self.number_of_scores && self.scores_mask.ignore_all()
    }

    fn is_complete(&self) -> bool {
        self.md5_beatmap_hash && self.number_of_scores && self.scores_mask.is_complete()
    }
}

#[derive(Copy, Clone, Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct ScoresDbMask {
    #[structopt(long = "show-version")]
    pub version: bool,
    #[structopt(long = "show-number-of-beatmaps")]
    pub number_of_beatmaps: bool,
    #[structopt(flatten)]
    pub beatmaps_mask: ScoresDbBeatmapMask,
}

impl ScoresDbMask {
    fn ignore_all(&self) -> bool {
        !self.version && !self.number_of_beatmaps && self.beatmaps_mask.ignore_all()
    }

    fn is_complete(&self) -> bool {
        self.version && self.number_of_beatmaps && self.beatmaps_mask.is_complete()
    }
}
