use std::io::Result as IoResult;

use crate::load_settings::{
    query::QueryStruct,
    scores::scoresdb_beatmap_load_settings::ScoresDbBeatmapLoadSettings,
};
use crate::masks::scores_mask::ScoresDbMask;

#[derive(Clone)]
pub struct ScoresDbLoadSettings {
    pub version: bool,
    pub number_of_beatmaps: bool,
    pub beatmap_load_settings: ScoresDbBeatmapLoadSettings,
}

impl QueryStruct<ScoresDbMask> for ScoresDbLoadSettings {
    fn load_all(&self) -> bool {
        self.version && self.number_of_beatmaps && self.beatmap_load_settings.load_all()
    }

    fn ignore_all(&self) -> bool {
        !self.version && !self.number_of_beatmaps && self.beatmap_load_settings.ignore_all()
    }

    fn is_partial(&self) -> bool {
        !self.version || !self.number_of_beatmaps || self.beatmap_load_settings.is_partial()
    }

    fn set_from_query(&mut self, args: Vec<&str>) -> IoResult<()> {
        self.beatmap_load_settings.set_from_query(args)
    }

    fn set_from_mask(&mut self, mask: &ScoresDbMask) {
        self.version |= mask.version;
        self.number_of_beatmaps |= mask.number_of_beatmaps;
        self.beatmap_load_settings
            .set_from_mask(&mask.beatmaps_mask);
    }
}
