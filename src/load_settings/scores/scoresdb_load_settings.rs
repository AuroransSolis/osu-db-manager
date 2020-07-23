use crate::load_settings::scores::scoresdb_beatmap_load_settings::ScoresDbBeatmapLoadSettings;
use crate::masks::scores_mask::ScoresDbMask;
use std::default::Default;
use structopt::StructOpt;

#[derive(Clone, StructOpt)]
pub struct ScoresDbLoadSettings {
    #[structopt(skip)]
    pub version: bool,
    #[structopt(skip)]
    pub number_of_beatmaps: bool,
    #[structopt(flatten)]
    pub beatmap_load_settings: ScoresDbBeatmapLoadSettings,
}

impl ScoresDbLoadSettings {
    pub fn set_from_mask(&mut self, mask: &ScoresDbMask) {
        self.version |= mask.version;
        self.number_of_beatmaps |= mask.number_of_beatmaps;
        self.beatmap_load_settings
            .set_from_mask(&mask.beatmaps_mask);
    }
}

impl Default for ScoresDbLoadSettings {
    fn default() -> Self {
        ScoresDbLoadSettings {
            version: bool::default(),
            number_of_beatmaps: bool::default(),
            beatmap_load_settings: ScoresDbBeatmapLoadSettings::default(),
        }
    }
}
