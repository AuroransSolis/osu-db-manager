#![rustfmt::skip]
use clap::{App, AppSettings, Arg, ArgGroup, ArgMatches, SubCommand};

use crate::masks::mask::Mask;

#[derive(Copy, Clone, Debug)]
pub struct CollectionMask {
    pub collection_name: bool,
    pub number_of_beatmaps: bool,
    pub md5_beatmap_hashes: bool,
}

impl CollectionMask {
    pub fn new(collection_name: bool, number_of_beatmaps: bool, md5_beatmap_hashes: bool) -> Self {
        CollectionMask {
            collection_name,
            number_of_beatmaps,
            md5_beatmap_hashes,
        }
    }
}

impl Default for CollectionMask {
    fn default() -> Self {
        CollectionMask {
            collection_name: true,
            number_of_beatmaps: true,
            md5_beatmap_hashes: false,
        }
    }
}

impl CollectionMask {
    fn is_complete(&self) -> bool {
        self.collection_name && self.number_of_beatmaps && self.md5_beatmap_hashes
    }

    fn from_input(input: &str) -> Self {
        let matches = App::new("collection.db collection entry show options parsing")
            .version("1.0.0")
            .author("Aurorans Solis")
            .about("Parser for show options for entries in collection.db (collections)")
            .arg(
                Arg::with_name("All")
                    .long("all")
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show all fields"),
            )
            .arg(
                Arg::with_name("None")
                    .long("none")
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show no fields"),
            )
            .arg(
                Arg::with_name("Collection name")
                    .long("collection-name")
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show collection name"),
            )
            .arg(
                Arg::with_name("Number of beatmaps")
                    .long("num-beatmaps")
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show number of beatmaps"),
            )
            .arg(
                Arg::with_name("MD5 beatmap hashes")
                    .long("md5-beatmap-hashes")
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show MD5 beatmap hashes"),
            )
            .get_matches_from(input.split_ascii_whitespace());
        let [collection_name, number_of_beatmaps, md5_beatmap_hashes] = if matches.is_present("All")
        {
            [true; 3]
        } else if matches.is_present("None") {
            [false; 3]
        } else {
            let collection_name = matches.is_present("Collection name");
            let number_of_beatmaps = matches.is_present("Number of beatmaps");
            let md5_beatmap_hashes = matches.is_present("MD5 beatmap hashes");
            [collection_name, number_of_beatmaps, md5_beatmap_hashes]
        };
        CollectionMask {
            collection_name,
            number_of_beatmaps,
            md5_beatmap_hashes,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct CollectionDbMask {
    pub version: bool,
    pub number_of_collections: bool,
    pub collections_mask: CollectionMask,
}

impl CollectionDbMask {
    pub fn new(
        version: bool,
        number_of_collections: bool,
        collections_mask: CollectionMask,
    ) -> Self {
        CollectionDbMask {
            version,
            number_of_collections,
            collections_mask,
        }
    }
}

impl Mask for CollectionDbMask {
    fn is_complete(&self) -> bool {
        self.version && self.number_of_collections && self.collections_mask.is_complete()
    }

    fn from_input(input: &str) -> Self {
        let matches = App::new("collection.db show options parsing")
            .version("1.0.0")
            .author("Aurorans Solis")
            .about("Parser for show options for collection.db")
            .arg(
                Arg::with_name("All")
                    .long("all")
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show all fields"),
            )
            .arg(
                Arg::with_name("None")
                    .long("none")
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show no fields"),
            )
            .arg(
                Arg::with_name("Version")
                    .long("VERSION")
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show collection.db version"),
            )
            .arg(
                Arg::with_name("Number of collections")
                    .long("NUMBER-OF-COLLECTIONS")
                    .takes_value(false)
                    .required(false)
                    .multiple(false)
                    .help("Show number of collections"),
            )
            .arg(
                Arg::with_name("Collection show options")
                    .long("collection-show-options")
                    .takes_value(true)
                    .value_name("SHOW_OPTIONS")
                    .required(false)
                    .multiple(false)
                    .help("Show options for collection.db entries (collections)"),
            )
            .get_matches_from(input.split_ascii_whitespace());
        let [version, number_of_collections] = if matches.is_present("All") {
            [true; 2]
        } else if matches.is_present("None") {
            [false; 2]
        } else {
            let version = matches.is_present("Version");
            let number_of_collections = matches.is_present("Number of collections");
            [version, number_of_collections]
        };
        let collections_mask =
            if let Some(collections_mask_options) = matches.value_of("Collection show options") {
                CollectionMask::from_input(collections_mask_options)
            } else {
                CollectionMask::default()
            };
        CollectionDbMask {
            version,
            number_of_collections,
            collections_mask,
        }
    }
}
