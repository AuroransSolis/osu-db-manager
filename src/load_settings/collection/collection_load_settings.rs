use crate::load_settings::{EqualClone, Relational};
use crate::masks::collection_mask::CollectionMask;
use structopt::StructOpt;

#[derive(Clone, StructOpt)]
pub struct CollectionLoadSettings {
    #[structopt(
        name = "collection name",
        long = "collection-name",
        value_name = "EQ",
        default_value,
        parse(try_from_str)
    )]
    pub collection_name: EqualClone<String>,
    #[structopt(
        name = "number of beatmaps",
        long = "number-of-beatmaps",
        value_name = "RELATIONAL",
        default_value,
        parse(try_from_str)
    )]
    pub number_of_beatmaps: Relational<i32>,
    #[structopt(
        name = "md5 beatmap hash",
        long = "md5-beatmap-hash",
        value_name = "EQ",
        default_value,
        parse(try_from_str)
    )]
    pub md5_beatmap_hash: EqualClone<String>,
}

impl CollectionLoadSettings {
    pub fn load_all(&self) -> bool {
        self.collection_name.is_load()
            && self.number_of_beatmaps.is_load()
            && self.md5_beatmap_hash.is_load()
    }

    pub fn ignore_all(&self) -> bool {
        self.collection_name.is_ignore()
            && self.number_of_beatmaps.is_ignore()
            && self.md5_beatmap_hash.is_ignore()
    }

    pub fn is_partial(&self) -> bool {
        self.collection_name.is_ignore()
            || self.number_of_beatmaps.is_ignore()
            || self.md5_beatmap_hash.is_ignore()
    }

    pub fn set_from_mask(&mut self, mask: &CollectionMask) {
        self.collection_name.apply_mask(mask.collection_name);
        self.number_of_beatmaps.apply_mask(mask.number_of_beatmaps);
        self.md5_beatmap_hash.apply_mask(mask.md5_beatmap_hashes);
    }
}
