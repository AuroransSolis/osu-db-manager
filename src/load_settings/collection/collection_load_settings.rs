use crate::load_settings::{EqualClone, Relational};
use crate::masks::collection_mask::CollectionMask;
use clap::{App, Arg};
use std::io::Result as IoResult;

pub struct CollectionLoadSettings {
    pub collection_name: EqualClone<String>,
    pub number_of_beatmaps: Relational<i32>,
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

    pub fn set_from_query(&mut self, args: Vec<&str>) -> IoResult<()> {
        if args.len() == 0 {
            return Ok(());
        }
        let matches = App::new("collection.db collection query parser")
            .arg(
                Arg::with_name("Collection name")
                    .long("COLLECTION-NAME")
                    .required(false)
                    .multiple(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NAME"),
            )
            .arg(
                Arg::with_name("Number of beatmaps")
                    .long("NUMBER-OF-BEATMAPS")
                    .required(false)
                    .multiple(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NUMBER/RANGE"),
            )
            .arg(
                Arg::with_name("MD5 beatmap hash")
                    .long("MD5-BEATMAP-HASH")
                    .required(false)
                    .multiple(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("HASH"),
            )
            .get_matches_from(args.into_iter());
        self.collection_name = EqualClone::from_matches(&matches, "Collection name")?.into();
        self.number_of_beatmaps = Relational::from_matches(&matches, "Number of beatmaps")?.into();
        self.md5_beatmap_hash = EqualClone::from_matches(&matches, "MD5 beatmap hash")?.into();
        Ok(())
    }

    pub fn set_from_mask(&mut self, mask: &CollectionMask) {
        self.collection_name.apply_mask(mask.collection_name);
        self.number_of_beatmaps.apply_mask(mask.number_of_beatmaps);
        self.md5_beatmap_hash.apply_mask(mask.md5_beatmap_hashes);
    }
}
