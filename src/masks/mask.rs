use crate::masks::{
    collection_mask::CollectionDbMask, osu_mask::OsuDbMask, scores_mask::ScoresDbMask,
};

pub trait Mask {
    fn is_complete(&self) -> bool;
    fn from_input(input: &str) -> Self;
    fn all() -> Self;
    fn none() -> Self;
}

pub enum DbMask {
    OsuMask(OsuDbMask),
    CollectionMask(CollectionDbMask),
    ScoresMask(ScoresDbMask),
}
