use crate::masks::{
    osu_mask::OsuDbMask,
    collection_mask::CollectionDbMask,
    scores_mask::ScoresDbMask
};

pub trait Mask: Default {
    fn is_complete(&self) -> bool;
    fn from_show_and_query(show: Self, query: Self) -> Self;
}

pub enum DbMask {
    OsuMask(OsuDbMask),
    CollectionMask(CollectionDbMask),
    ScoresMask(ScoresDbMask)
}