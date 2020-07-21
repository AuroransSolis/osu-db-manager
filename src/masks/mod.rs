pub mod collection_mask;
pub mod osu_mask;
pub mod scores_mask;

use crate::masks::{
    collection_mask::CollectionDbMask, osu_mask::OsuDbMask, scores_mask::ScoresDbMask,
};
use structopt::StructOpt;

#[derive(StructOpt)]
pub enum DbMask {
    #[structopt(name = "collection-show")]
    CollectionMask(CollectionDbMask),
    #[structopt(name = "osu-show")]
    OsuMask(OsuDbMask),
    #[structopt(name = "scores-show")]
    ScoresMask(ScoresDbMask),
}
