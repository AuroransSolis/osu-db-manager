use crate::masks::mask::Mask;

#[derive(Copy, Clone, Debug)]
pub struct ScoreMask {
    pub gameplay_mode: bool,
    pub score_version: bool,
    pub md5_beatmap_hash: bool,
    pub player_name: bool,
    pub md5_replay_hash: bool,
    pub number_of_300s: bool,
    pub number_of_100s: bool, // 150s in Taiko, 100s in CTB, 200s in Mania
    pub number_of_50s: bool, // small fruit in CTB, 50s in Mania
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
    pub online_score_id: bool
}

/*
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
    online_score_id: bool
    
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
    online_score_id
*/

impl ScoreMask {
    pub fn new(gameplay_mode: bool, score_version: bool, md5_beatmap_hash: bool, player_name: bool,
        md5_replay_hash: bool, number_of_300s: bool, number_of_100s: bool, number_of_50s: bool,
        number_of_gekis: bool, number_of_katus: bool, number_of_misses: bool, replay_score: bool,
        max_combo: bool, perfect_combo: bool, mods_used: bool, empty_string: bool,
        replay_timestamp: bool, negative_one: bool, online_score_id: bool) -> Self {
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
            online_score_id
        }
    }
}

impl Mask for ScoreMask {
    fn is_complete(&self) -> bool {
        self.gameplay_mode && self.score_version && self.md5_beatmap_hash && self.player_name
            && self.md5_replay_hash && self.number_of_300s && self.number_of_100s
            && self.number_of_50s && self.number_of_gekis && self.number_of_katus
            && self.number_of_misses && self.replay_score && self.max_combo&& self.perfect_combo
            && self.mods_used && self.empty_string && self.replay_timestamp && self.negative_one
            && online_score_id
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
            online_score_id: show.online_score_id || query.online_score_id
        }
    }
}