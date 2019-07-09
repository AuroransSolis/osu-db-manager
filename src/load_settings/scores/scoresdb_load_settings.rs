use std::io::Result as IoResult;

use crate::load_settings::{
    LoadSetting,
    Empty,
    query::QueryStruct,
    scores::scoresdb_beatmap_load_settings::ScoresDbBeatmapLoadSettings
};
use crate::masks::scores_mask::ScoresDbMask;

pub struct ScoresDbLoadSettings {
    pub version: LoadSetting<Empty>,
    pub number_of_beatmaps: LoadSetting<Empty>,
    pub beatmap_load_settings: ScoresDbBeatmapLoadSettings
}

impl QueryStruct<ScoresDbMask> for ScoresDbLoadSettings {
    fn load_all(&self) -> bool {
        self.beatmap_load_settings.load_all() && self.version.is_load()
            && self.number_of_beatmaps.is_load()
    }

    fn is_partial(&self) -> bool {
        self.beatmap_load_settings.is_partial() || self.version.is_ignore()
            || self.number_of_beatmaps.is_ignore()
    }

    fn set_from_query(&mut self, args: Vec<&str>) -> IoResult<()> {
        self.beatmap_load_settings.set_from_query(args)
    }

    fn set_from_mask(&mut self, mask: ScoresDbMask) {
        if self.version.is_ignore() && mask.version {
            self.version = LoadSetting::Load;
        }
        if self.number_of_beatmaps.is_ignore() && mask.number_of_beatmaps {
            self.number_of_beatmaps = LoadSetting::Load;
        }
        if let Some(m) = mask.beatmaps_mask {
            self.beatmap_load_settings.set_from_mask(m);
        }
    }
}