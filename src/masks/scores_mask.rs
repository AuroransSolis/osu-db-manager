use structopt::StructOpt;

#[derive(Copy, Clone, Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct ScoreMask {
    #[structopt(long)]
    pub gameplay_mode: bool,
    #[structopt(long)]
    pub score_version: bool,
    #[structopt(long)]
    pub score_md5_beatmap_hash: bool,
    #[structopt(long)]
    pub player_name: bool,
    #[structopt(long)]
    pub md5_replay_hash: bool,
    #[structopt(long)]
    pub number_of_300s: bool,
    #[structopt(long)]
    pub number_of_100s: bool,  // 150s in Taiko, 100s in CTB, 200s in Mania
    #[structopt(long)]
    pub number_of_50s: bool,   // small fruit in CTB, 50s in Mania
    #[structopt(long)]
    pub number_of_gekis: bool, // max 300s in Mania
    #[structopt(long)]
    pub number_of_katus: bool, // 100s in mania
    #[structopt(long)]
    pub number_of_misses: bool,
    #[structopt(long)]
    pub replay_score: bool,
    #[structopt(long)]
    pub max_combo: bool,
    #[structopt(long)]
    pub perfect_combo: bool,
    #[structopt(long)]
    pub mods_used: bool,
    #[structopt(long)]
    pub empty_string: bool,
    #[structopt(long)]
    pub replay_timestamp: bool,
    #[structopt(long)]
    pub negative_one: bool,
    #[structopt(long)]
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
    #[structopt(long)]
    pub md5_beatmap_hash: bool,
    #[structopt(long)]
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
    #[structopt(long)]
    pub version: bool,
    #[structopt(long)]
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
