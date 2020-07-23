use crate::load_settings::{
    scores::score_load_settings::ScoreLoadSettings, EqualClone, Relational,
};
use crate::masks::scores_mask::ScoresDbBeatmapMask;
use std::default::Default;
use structopt::StructOpt;

#[derive(Clone, StructOpt)]
pub struct ScoresDbBeatmapLoadSettings {
    #[structopt(
        name = "md5 beatmap hash",
        long = "md5-beatmap-hash",
        value_name = "EQ",
        default_value,
        parse(try_from_str)
    )]
    pub md5_beatmap_hash: EqualClone<String>,
    #[structopt(
        name = "number of scores",
        long = "number-of-scores",
        value_name = "RELATIONAL",
        default_value,
        parse(try_from_str)
    )]
    pub number_of_scores: Relational<i32>,
    #[structopt(flatten)]
    pub score_load_settings: ScoreLoadSettings,
}

impl ScoresDbBeatmapLoadSettings {
    pub fn ignore_all(&self) -> bool {
        self.md5_beatmap_hash.is_ignore()
            && self.number_of_scores.is_ignore()
            && self.score_load_settings.ignore_all()
    }

    pub fn is_partial(&self) -> bool {
        self.md5_beatmap_hash.is_ignore()
            || self.number_of_scores.is_ignore()
            || self.score_load_settings.is_partial()
    }

    pub fn set_from_mask(&mut self, mask: &ScoresDbBeatmapMask) {
        if self.md5_beatmap_hash.is_ignore() && mask.md5_beatmap_hash {
            self.md5_beatmap_hash = EqualClone::Load;
        }
        if self.number_of_scores.is_ignore() && mask.number_of_scores {
            self.number_of_scores = Relational::Load;
        }
        self.score_load_settings.set_from_mask(&mask.scores_mask);
    }
}

impl Default for ScoresDbBeatmapLoadSettings {
    fn default() -> Self {
        ScoresDbBeatmapLoadSettings {
            md5_beatmap_hash: EqualClone::default(),
            number_of_scores: Relational::default(),
            score_load_settings: ScoreLoadSettings::default(),
        }
    }
}
