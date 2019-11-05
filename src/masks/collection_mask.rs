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
            md5_beatmap_hashes: true,
        }
    }
}

impl CollectionMask {
    fn is_complete(&self) -> bool {
        self.collection_name && self.number_of_beatmaps && self.md5_beatmap_hashes
    }

    fn from_show_matches(matches: &ArgMatches) -> Self {
        let collection_name = matches.is_present("Collection name");
        let number_of_beatmaps = matches.is_present("Number of beatmaps");
        let md5_beatmap_hashes = matches.is_present("MD5 beatmap hashes");
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
    pub collections_mask: Option<CollectionMask>,
}

impl CollectionDbMask {
    pub fn new(
        version: bool,
        number_of_collections: bool,
        collections_mask: Option<CollectionMask>,
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
        if let Some(collection_mask) = self.collections_mask {
            collection_mask.is_complete() && self.version && self.number_of_collections
        } else {
            false
        }
    }

    fn from_show_args(show_args: Vec<&str>) -> Self {
        let matches = App::new("collection.db show options parser")
            .arg(
                Arg::with_name("Version")
                    .long("VERSION")
                    .required(false)
                    .takes_value(false)
                    .multiple(false),
            )
            .arg(
                Arg::with_name("Number of collections")
                    .long("NUMBER-OF-COLLECTIONS")
                    .required(false)
                    .takes_value(false)
                    .multiple(false),
            )
            .subcommand(
                SubCommand::with_name("COLLECTION-OPTIONS")
                    .arg(
                        Arg::with_name("Collection name")
                            .long("COLLECTION-NAME")
                            .required(false)
                            .takes_value(false)
                            .multiple(false),
                    )
                    .arg(
                        Arg::with_name("Number of beatmaps")
                            .long("NUMBER-OF-BEATMAPS")
                            .required(false)
                            .takes_value(false)
                            .multiple(false),
                    )
                    .arg(
                        Arg::with_name("MD5 beatmap hashes")
                            .long("MD5-BEATMAP-HASHES")
                            .required(false)
                            .takes_value(false)
                            .multiple(false),
                    ),
            )
            .get_matches_from(show_args.into_iter());
        let version = matches.is_present("Version");
        let number_of_collections = matches.is_present("Number of collections");
        let collections_mask = if let Some(m) = matches.subcommand_matches("COLLECTION-OPTIONS") {
            Some(CollectionMask::from_show_matches(m))
        } else {
            None
        };
        CollectionDbMask {
            version,
            number_of_collections,
            collections_mask,
        }
    }
}
