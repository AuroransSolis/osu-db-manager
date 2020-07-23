use crate::databases::scores::partial_score::PartialScore;
use crate::deserialize_primitives::*;
use crate::load_settings::scores::scoresdb_beatmap_load_settings::ScoresDbBeatmapLoadSettings;
use crate::maybe_deserialize_primitives::*;
use crate::read_error::ParseFileResult;
use crate::{masks::scores_mask::ScoresDbBeatmapMask, maybe_print};

#[derive(Debug, Clone)]
pub struct PartialScoresDbBeatmap<'a> {
    pub md5_beatmap_hash: Option<&'a str>,
    pub number_of_scores: i32,
    pub scores: Option<Vec<PartialScore<'a>>>,
}

impl<'a> PartialScoresDbBeatmap<'a> {
    pub fn read_from_bytes(
        settings: &ScoresDbBeatmapLoadSettings,
        bytes: &'a [u8],
        i: &mut usize,
    ) -> ParseFileResult<Self> {
        let mut skip = false;
        let s = &mut skip;
        let md5_beatmap_hash = maybe_read_md5_hash(&settings.md5_beatmap_hash, s, bytes, i)?;
        let number_of_scores = read_int(bytes, i)?;
        let scores = if settings.score_load_settings.ignore_all() || number_of_scores == 0 || *s {
            None
        } else {
            let mut scores = Vec::with_capacity(number_of_scores as usize);
            for _ in 0..number_of_scores {
                if let Some(score) =
                    PartialScore::read_from_bytes(&settings.score_load_settings, bytes, i)?
                {
                    scores.push(score);
                }
            }
            Some(scores)
        };
        Ok(PartialScoresDbBeatmap {
            md5_beatmap_hash,
            number_of_scores,
            scores,
        })
    }

    pub fn display(&self, show: ScoresDbBeatmapMask) {
        maybe_print!(show.md5_beatmap_hash, self.md5_beatmap_hash, "    ");
        if show.number_of_scores {
            println!("    number of scores: {}", self.number_of_scores);
        }
        if !show.scores_mask.ignore_all() && self.scores.is_some() {
            for score in self.scores.as_ref().unwrap() {
                score.display(show.scores_mask);
            }
        }
    }
}
