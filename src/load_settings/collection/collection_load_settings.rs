use std::io::Result as IoResult;

use clap::{Arg, App, SubCommand, AppSettings};

use crate::load_settings::{
    LoadSetting,
    SpecialArgType,
    parse_from_arg,
    parse_from_arg_special,
    query::QueryStruct
};
use crate::masks::collection_mask::CollectionMask;

pub struct CollectionLoadSettings {
    pub collection_name: LoadSetting<Option<String>>,
    pub number_of_beatmaps: LoadSetting<i32>,
    pub md5_beatmap_hash: LoadSetting<String>
}

impl CollectionLoadSettings {
    pub fn load_all(&self) -> bool {
        !(self.collection_name.is_ignore() || self.number_of_beatmaps.is_ignore()
            || self.md5_beatmap_hash.is_ignore())
    }

    pub fn set_from_query(&mut self, args: Vec<&str>) -> IoResult<()> {
        let matches = App::new("collection.db collection query parser")
            .arg(Arg::with_name("Collection name")
                .long("COLLECTION-NAME")
                .required(false)
                .multiple(false)
                .takes_value(true)
                .number_of_values(1)
                .value_name("NAME"))
            .arg(Arg::with_name("Number of beatmaps")
                .long("NUMBER-OF-BEATMAPS")
                .required(false)
                .multiple(false)
                .takes_value(true)
                .number_of_values(1)
                .value_name("NUMBER/RANGE"))
            .arg(Arg::with_name("MD5 beatmap hash")
                .long("MD5-BEATMAP-HASH")
                .required(false)
                .multiple(false)
                .takes_value(true)
                .number_of_values(1)
                .value_name("HASH"))
            .get_matches_from(args.into_iter());
        self.collection_name = parse_from_arg_special::<Option<String>>(&matches, "Collection name",
            SpecialArgType::OptionString)?;
        self.number_of_beatmaps = parse_from_arg::<i32>(&matches, "Number of beatmaps")?;
        self.md5_beatmap_hash = parse_from_arg_special::<String>(&matches, "MD5 beatmap hash",
            SpecialArgType::String)?;
        Ok(())
    }

    pub fn set_from_mask(&mut self, mask: CollectionMask) {
        if self.collection_name.is_ignore() && mask.collection_name {
            self.collection_name = LoadSetting::Load;
        }
        if self.number_of_beatmaps.is_ignore() && mask.number_of_beatmaps {
            self.number_of_beatmaps = LoadSetting::Load;
        }
        if self.md5_beatmap_hash.is_ignore() && mask.md5_beatmap_hashes {
            self.md5_beatmap_hash = LoadSetting::Load;
        }
    }
}