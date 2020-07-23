use crate::databases::scores::score::Score;
use crate::deserialize_primitives::*;
use crate::read_error::ParseFileResult;

#[derive(Debug, Clone)]
pub struct ScoresDbBeatmap<'a> {
    pub md5_beatmap_hash: &'a str,
    pub number_of_scores: i32,
    pub scores: Option<Vec<Score<'a>>>,
}

impl<'a> ScoresDbBeatmap<'a> {
    pub fn read_from_bytes(bytes: &'a [u8], i: &mut usize) -> ParseFileResult<Self> {
        let md5_beatmap_hash = read_md5_hash(bytes, i)?;
        let number_of_scores = read_int(bytes, i)?;
        let scores = if number_of_scores == 0 {
            None
        } else {
            let mut scores = Vec::with_capacity(number_of_scores as usize);
            for _ in 0..number_of_scores {
                scores.push(Score::read_from_bytes(bytes, i)?);
            }
            Some(scores)
        };
        Ok(ScoresDbBeatmap {
            md5_beatmap_hash,
            number_of_scores,
            scores,
        })
    }

    pub fn display(&self) {
        println!("    md5 beatmap hash: {}", self.md5_beatmap_hash);
        println!("    number of scores: {}", self.number_of_scores);
        println!("    scores {{");
        if self.scores.is_some() {
            for score in self.scores.as_ref().unwrap() {
                score.display();
            }
        } else {
            println!("        no scores");
        }
        println!("    }}");
    }
}
