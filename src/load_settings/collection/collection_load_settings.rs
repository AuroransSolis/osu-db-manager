use crate::query::Comparison;

pub struct CollectionQuery {
    pub collection_name: Comparison<String>,
    pub number_of_beatmaps: Comparison<i32>,
    pub md5_beatmap_hash: Comparison<String>
}