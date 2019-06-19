use crate::deserialize_primitives::*;
use crate::databases::scores::score::Score;
use crate::read_error::ParseFileResult;
use crate::masks::scores_mask::ScoresDbBeatmapMask;

#[derive(Debug, Clone)]
pub struct PartialScoresDbBeatmap {
    pub md5_beatmap_hash: Option<String>,
    pub number_of_scores: i32,
    pub scores: Option<Vec<Score>>
}

impl PartialScoresDbBeatmap {
    pub fn read_from_bytes(mask: ScoresDbBeatmapMask, bytes: &[u8], i: &mut usize)
        -> ParseFileResult<Self> {
        let md5_beatmap_hash = if mask.md5_beatmap_hash {
            Some(read_md5_hash(bytes, i)?)
        } else {
            *i += 34;
            None
        };
        let number_of_scores = read_int(bytes, i)?;
        let mut scores = if number_of_scores == 0 {
            None
        } else {
            let mut scores = Vec::with_capacity(number_of_scores as usize);
            for _ in 0..number_of_scores {
                scores.push(Score::read_from_bytes(bytes, i)?);
            }
            Some(scores)
        };
        Ok(PartialScoresDbBeatmap {
            md5_beatmap_hash,
            number_of_scores,
            scores
        })
    }
}