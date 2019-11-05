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
}

impl Mask for ScoreMask {
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

    fn from_show_and_query(show: Self, query: Self) -> Self {
        ScoreMask {
            gameplay_mode: show.gameplay_mode || query.gameplay_mode,
            score_version: show.score_version || query.score_version,
            md5_beatmap_hash: show.md5_beatmap_hash || query.md5_beatmap_hash,
            player_name: show.player_name || query.player_name,
            md5_replay_hash: show.md5_replay_hash || query.md5_replay_hash,
            number_of_300s: show.number_of_300s || query.number_of_300s,
            number_of_100s: show.number_of_100s || query.number_of_100s,
            number_of_50s: show.number_of_50s || query.number_of_50s,
            number_of_gekis: show.number_of_gekis || query.number_of_gekis,
            number_of_katus: show.number_of_katus || query.number_of_katus,
            number_of_misses: show.number_of_misses || query.number_of_misses,
            replay_score: show.replay_score || query.replay_score,
            max_combo: show.max_combo || query.max_combo,
            perfect_combo: show.perfect_combo || query.perfect_combo,
            mods_used: show.mods_used || query.mods_used,
            empty_string: show.empty_string || query.empty_string,
            replay_timestamp: show.replay_timestamp || query.replay_timestamp,
            negative_one: show.negative_one || query.negative_one,
            online_score_id: show.online_score_id || query.online_score_id,
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
            empty_string: true,
            replay_timestamp: true,
            negative_one: true,
            online_score_id: true,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ScoresDbBeatmapMask {
    pub md5_beatmap_hash: bool,
    pub number_of_scores: bool,
    pub scores_mask: Option<ScoreMask>,
}

impl ScoresDbBeatmapMask {
    pub fn new(
        md5_beatmap_hash: bool,
        number_of_scores: bool,
        scores_mask: Option<ScoreMask>,
    ) -> Self {
        ScoresDbBeatmapMask {
            md5_beatmap_hash,
            number_of_scores,
            scores_mask,
        }
    }
}

impl Mask for ScoresDbBeatmapMask {
    fn is_complete(&self) -> bool {
        if let Some(scores_mask) = self.scores_mask {
            scores_mask.is_complete() && self.md5_beatmap_hash && self.number_of_scores
        } else {
            false
        }
    }

    fn from_show_and_query(show: Self, query: Self) -> Self {
        ScoresDbBeatmapMask {
            md5_beatmap_hash: show.md5_beatmap_hash || query.md5_beatmap_hash,
            number_of_scores: show.number_of_scores || query.number_of_scores,
            scores_mask: match (show.scores_mask, query.scores_mask) {
                (Some(show_mask), Some(query_mask)) => {
                    Some(ScoreMask::from_show_and_query(show_mask, query_mask))
                }
                (Some(show_mask), None) => Some(show_mask),
                (None, Some(query_mask)) => Some(query_mask),
                (None, None) => None,
            },
        }
    }
}

impl Default for ScoresDbBeatmapMask {
    fn default() -> Self {
        ScoresDbBeatmapMask {
            md5_beatmap_hash: true,
            number_of_scores: true,
            scores_mask: Some(ScoreMask::default()),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ScoresDbMask {
    pub version: bool,
    pub number_of_beatmaps: bool,
    pub beatmaps_mask: Option<ScoresDbBeatmapMask>,
}

impl ScoresDbMask {
    pub fn new(
        version: bool,
        number_of_beatmaps: bool,
        beatmaps_mask: Option<ScoresDbBeatmapMask>,
    ) -> Self {
        ScoresDbMask {
            version,
            number_of_beatmaps,
            beatmaps_mask,
        }
    }
}

impl Mask for ScoresDbMask {
    fn is_complete(&self) -> bool {
        if let Some(beatmaps_mask) = self.beatmaps_mask {
            beatmaps_mask.is_complete() && self.version && self.number_of_beatmaps
        } else {
            false
        }
    }

    fn from_show_and_query(show: Self, query: Self) -> Self {
        ScoresDbMask {
            version: show.version || query.version,
            number_of_beatmaps: show.number_of_beatmaps || query.number_of_beatmaps,
            beatmaps_mask: match (show.beatmaps_mask, query.beatmaps_mask) {
                (Some(show_mask), Some(query_mask)) => Some(
                    ScoresDbBeatmapMask::from_show_and_query(show_mask, query_mask),
                ),
                (Some(show_mask), None) => Some(show_mask),
                (None, Some(query_mask)) => Some(query_mask),
                (None, None) => None,
            },
        }
    }
}

impl Default for ScoresDbMask {
    fn default() -> Self {
        ScoresDbMask {
            version: true,
            number_of_beatmaps: true,
            beatmaps_mask: Some(ScoresDbBeatmapMask::default()),
        }
    }
}
