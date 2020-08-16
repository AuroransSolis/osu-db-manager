use crate::databases::merge::Merge;
use crate::load_settings::{
    collection::collectiondb_load_settings::CollectionDbLoadSettings,
    osu::osudb_load_settings::OsuDbLoadSettings,
    scores::scoresdb_load_settings::ScoresDbLoadSettings,
};
use crate::masks::{
    collection_mask::CollectionDbMask, osu_mask::OsuDbMask, scores_mask::ScoresDbMask,
};
use std::str::FromStr;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    name = "osu-db-manager",
    author = "Aurorans Solis",
    about = "osu! database managing, merging, parsing, and querying tool",
    version = "0.0.1",
    after_help = r#"Information about value name notation:
    Each option will have one of the following value names:
        - EQ
        - EQ-BOOL
        - RELATIONAL
        - RELATIONAL-DATE
    EQ and EQ-BOOL both have the same expected syntax, and RELATIONAL and RELATIONAL-DATE do
    as well. Here are their expected syntaxes:
        - EQ: just a value, for instance --artist-name 'Thank You Scientist'
        - EQ-BOOL: a boolean indicated by t, true, y, yes, 1, f, false, n, no, or 0
        - RELATIONAL: there are many accepted formats for relationals:
            - equal: --ar 9
            - greater than: --ar '(9..)'
            - less than: --ar '(..9)'
            - greater than or equal to: --ar '[9..)'
            - less than or equal to: --ar '(..9]'
            - in range (exclusive, exclusive): --ar '(8..10)'
            - in range (exclusive, inclusive): --ar '(8..10]'
            - in range (inclusive, exlusive): --ar '[8..10)'
            - in range (inclusive, inclusive): --ar '[8..10]'
        - RELATIONAL-DATE: just like RELATIONAL, except with a date. The date is expected to be in a
            YYYY-MM-DD format.

Information about interface types:
    - None (no option given): controlled by command line arguments, much like you're doing now
    - Shell: presents a shell-like interface to browse a database.
    - TUI: presents a text-based "graphical" browser of the database."#
)]
pub struct Arguments {
    #[structopt(
        name = "database type",
        short = "t",
        long = "type",
        value_name = "TYPE",
        possible_values(&["collection", "osu", "scores"]),
        parse(try_from_str)
    )]
    pub db_type: DbIndicator,
    #[structopt(
        name = "database path",
        short = "p",
        long = "path",
        value_name = "PATH"
    )]
    pub db_path: String,
    #[structopt(
        name = "jobs",
        short = "j",
        long = "jobs",
        value_name = "NUM",
        default_value = "1"
    )]
    pub jobs: usize,
    #[structopt(subcommand)]
    pub merge_search_or_interface: Option<MergeSearchOrInterface>,
}

#[derive(StructOpt)]
pub enum Search {
    #[structopt(name = "osu-search")]
    OsuSearch {
        #[structopt(flatten)]
        load_settings: OsuDbLoadSettings,
        #[structopt(flatten)]
        mask: OsuDbMask,
    },
    #[structopt(name = "collection-search")]
    CollectionSearch {
        #[structopt(flatten)]
        load_settings: CollectionDbLoadSettings,
        #[structopt(flatten)]
        mask: CollectionDbMask,
    },
    #[structopt(name = "scores-search")]
    ScoresSearch {
        #[structopt(flatten)]
        load_settings: ScoresDbLoadSettings,
        #[structopt(flatten)]
        mask: ScoresDbMask,
    },
}

#[derive(StructOpt)]
pub enum MergeSearchOrInterface {
    Search {
        #[structopt(subcommand)]
        search: Search,
    },
    #[structopt(name = "merge")]
    Merge(Merge),
    #[structopt(name = "interface")]
    Interface {
        #[structopt(
            name = "interface type",
            short = "i",
            long = "interface",
            value_name = "INTERFACE",
            possible_values(&["s", "shell", "t", "tui"]),
            parse(try_from_str)
        )]
        interface: InterfaceType,
    },
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum InterfaceType {
    None,
    Shell,
    Tui,
}

impl FromStr for InterfaceType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "s" | "shell" => Ok(InterfaceType::Shell),
            "t" | "tui" => Ok(InterfaceType::Tui),
            _ => Err(format!("Unknown interface type: {}", s)),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DbIndicator {
    OsuDb,
    CollectionDb,
    ScoresDb,
}

impl FromStr for DbIndicator {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "collection" => Ok(DbIndicator::CollectionDb),
            "osu" => Ok(DbIndicator::OsuDb),
            "scores" => Ok(DbIndicator::ScoresDb),
            other @ _ => Err(format!("Invalid database type: {}", other)),
        }
    }
}
