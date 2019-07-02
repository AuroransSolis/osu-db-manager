use crate::masks::{
    osu_mask::OsuDbMask,
    collection_mask::CollectionDbMask,
    scores_mask::ScoresDbMask
};

pub trait Mask {
    fn is_complete(&self) -> bool;
    fn from_show_args(show_args: Vec<&str>) -> Self;
}

pub enum DbMask {
    OsuMask(OsuDbMask),
    CollectionMask(CollectionDbMask),
    ScoresMask(ScoresDbMask)
}