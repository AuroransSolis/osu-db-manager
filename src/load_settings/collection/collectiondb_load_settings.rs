use std::io::{Result as IoResult, Error as IoError, ErrorKind::InvalidInput};

use clap::{Arg, App, SubCommand, AppSettings, ArgGroup};

use crate::query::collection::collection_query::CollectionQuery;

pub struct CollectionDbQuery {
    pub version: bool,
    pub number_of_collections: bool,
    pub collections_query: Option<CollectionQuery>
}

impl CollectionDbQuery {
    pub fn from_args(args: Vec<&str>) -> IoResult<Self> {
        let matches = App::new("collection.db query parser")
            .arg(Arg::with_name("Version")
                .long("VERSION")
                .required(false)
                .multiple(false)
                .takes_value(false))
            .arg(Arg::with_name("Number of collections")
                .long("NUMBER-OF-COLLECTIONS")
                .required(false))
            .subcommand(SubCommand::with_name("query")
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
                    .value_name("HASH"))).get_matches_from(args.into_iter());
        let version = matches.is_present("Version");
        let number_of_collections = matches.is_present("Number of collections");
        let collections_query = if let Some(subcommand_matches)
            = matches.subcommand_matches("query") {
            get_and_assign_string!(subcommand_matches {
                "Collection name" => collection_name;
                "MD5 beatmap hash" => md5_beatmap_hash;
            });
            get_parse_and_assign!(subcommand_matches {
                "Number of beatmaps", number_of_beatmaps => i32;
            });
            Some(CollectionQuery {
                collection_name,
                number_of_beatmaps,
                md5_beatmap_hash
            })
        } else {
            None
        };
        Ok(CollectionDbQuery {
            version,
            number_of_collections,
            collections_query
        })
    }
}