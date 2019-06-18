use crate::databases::osu::{
    partial_beatmap::PartialBeatmap,
    primitives::{ByteSingle::*, GameplayMode, TimingPoint},
    versions::*
};
use crate::deserialize_primitives::*;
use std::time::SystemTime;

pub struct PartialOsuDb {
    pub version: Option<i32>,
    pub folder_count: Option<i32>,
    pub account_unlocked: Option<bool>,
    pub account_unlock_date: Option<SystemTime>,
    pub player_name: Option<String>,
    pub number_of_beatmaps: Option<i32>,
    pub beatmaps: Option<Vec<PartialBeatmap>>,
    pub unknown_int: Option<i32>
}