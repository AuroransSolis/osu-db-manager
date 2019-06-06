use std::fs::read;
use std::io::{Result as IoResult, Error as IoError, ErrorKind::Other};
use crate::databases::merge::ConflictResolution;
use clap::{Arg, App, SubCommand};
use std::hint::unreachable_unchecked;

pub struct Arguments {
    pub db: Option<Database>,
    pub jobs: Option<usize>,
    pub interface: Option<InterfaceType>,
    pub database_query: Option<String>,
    pub show_options: Option<String>,
    pub merge: Option<(Database, ConflictResolution)>,
    pub help: Option<HelpWith>
}

impl Arguments {
    fn new() -> Self {
        Arguments {
            db: None,
            jobs: None,
            interface: None,
            database_query: None,
            show_options: None,
            merge: None,
            help: None
        }
    }
    
    fn new_help(help: HelpWith) -> Self {
        Arguments {
            db: None,
            jobs: None,
            interface: None,
            database_query: None,
            show_options: None,
            merge: None,
            help: Some(help)
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Database {
    OsuDb(Vec<u8>),
    CollectionDb(Vec<u8>),
    ScoresDb(Vec<u8>)
}

impl Database {
    fn new_of_same_type(first: &Self, bytes: Vec<u8>) -> Self {
        match first {
            &Database::OsuDb(_) => Database::OsuDb(bytes),
            &Database::CollectionDb(_) => Database::CollectionDb(bytes),
            &Database::ScoresDb(_) => Database::ScoresDb(bytes)
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum InterfaceType {
    Shell,
    Tui,
    None
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum HelpWith {
    Query(DbIndicator),
    Show(DbIndicator),
    ConflictResolution
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DbIndicator {
    OsuDb,
    CollectionDb,
    ScoresDb
}

impl From<&str> for DbIndicator {
    fn from(other: &str) -> Self {
        match other {
            "o" | "osu" => DbIndicator::OsuDb,
            "c" | "collection" => DbIndicator::CollectionDb,
            "s" | "scores" => DbIndicator::ScoresDb,
            _ => unreachable!()
        }
    }
}

pub fn get_arguments() -> IoResult<Arguments> {
    let matches = App::new("osu-db-manager")
        .version("1.0.0")
        .author("Aurorans Solis")
        .about("Tool to read, write, browse, and merge osu! databases.")
        .arg(Arg::with_name("osu!.db specifier")
            .short("o")
            .long("osu")
            .takes_value(true)
            .value_name("PATH")
            .required_unless_one(&["collection.db specifier", "scores.db specifier", "info"])
            .conflicts_with_all(&["collection.db specifier", "scores.db specifier", "info"])
            .help("Specifies that the given path is to an osu!.db"))
        .arg(Arg::with_name("collection.db specifier")
            .short("c")
            .long("collection")
            .takes_value(true)
            .value_name("PATH")
            .required_unless_one(&["osu!.db specifier", "scores.db specifier", "info"])
            .conflicts_with_all(&["osu!.db specifier", "scores.db specifier", "info"])
            .help("Specifies that the given path is to a collection.db"))
        .arg(Arg::with_name("scores.db specifier")
            .short("s")
            .long("scores")
            .takes_value(true)
            .value_name("PATH")
            .required_unless_one(&["osu!.db specifier", "collection.db specifier", "info"])
            .conflicts_with_all(&["osu!.db specifier", "collection.db specifier", "info"])
            .help("Specifies that the given path is to a scores.db"))
        .arg(Arg::with_name("Jobs")
            .short("j")
            .long("jobs")
            .takes_value(true)
            .number_of_values(1)
            .value_name("JOBS")
            .default_value("1")
            .help("Number of threads to load the database with. Default: 1"))
        .arg(Arg::with_name("Interface type")
            .short("i")
            .long("interface")
            .takes_value(true)
            .value_name("INTERFACE_TYPE")
            .possible_values(&["s", "shell", "t", "tui", "n", "none"])
            .multiple(false)
            .required(false)
            .help("Interface type specifier. Valid interfaces: 't'/'tui', 's'/'shell'. Default \
                is none (print requested information and quit)."))
        .arg(Arg::with_name("Database query")
            .short("q")
            .long("query")
            .value_name("QUERY")
            .takes_value(true)
            .multiple(false)
            .required(false)
            .help("Database query. Use 'info --query TYPE' for information on what you can \
                query in database type TYPE. Can be used in conjunction with show options."))
        .arg(Arg::with_name("Show options")
            .short("S")
            .long("show")
            .value_name("SHOW_OPTIONS")
            .takes_value(true)
            .multiple(false)
            .required(false)
            .conflicts_with("Interface type")
            .help("What information to show from each database entry. Use 'info --show TYPE' \
                for information on what you can show from each database type TYPE. Can be used in \
                conjunction with a query."))
        .arg(Arg::with_name("Merge")
            .short("m")
            .long("merge")
            .conflicts_with_all(&["osu!.db specifier", "Database query", "Show options"])
            .takes_value(true)
            .value_name("PATH")
            .multiple(false)
            .required(false)
            .help("Merge a second database at location PATH into the one specified with the \
                first argument. The databases will be treated as the same type, so make sure the \
                path points to one of the same type!"))
        .arg(Arg::with_name("Resolution")
            .short("r")
            .long("resolution")
            .multiple(false)
            .required(false)
            .takes_value(true)
            .value_name("RESOLUTION_TYPE")
            .help("Method used to resolve conflicts in a merge. Only required if no interface \
                is specified. For information on available conflict resolution methods, use 'info \
                --conflict-resolution'."))
        .subcommand(SubCommand::with_name("info")
            .about("Subcommand to provide additional information for options for osu-db-manager.")
            .version("1.0.0")
            .author("Aurorans Solis")
            .arg(Arg::with_name("query")
                .long("query")
                .takes_value(true)
                .value_name("TYPE")
                .possible_values(&["o", "osu", "c", "collection", "s", "scores", ""])
                .required_unless_one(&["show", "conflict resolution"])
                .conflicts_with_all(&["show", "conflict resolution"])
                .help("Shows all available fields to query in database TYPE as well as examples \
                    for querying each field of a database."))
            .arg(Arg::with_name("show")
                .long("show")
                .takes_value(true)
                .value_name("TYPE")
                .possible_values(&["o", "osu", "c", "collection", "s", "scores", ""])
                .required_unless_one(&["query", "conflict resolution"])
                .conflicts_with_all(&["query", "conflict resolution"])
                .help("Shows all available fields in database TYPE to display."))
            .arg(Arg::with_name("conflict resolution")
                .long("conflict-resolution")
                .takes_value(false)
                .multiple(false)
                .required_unless_one(&["query", "show"])
                .conflicts_with_all(&["query", "show"])
                .help("Shows available merging methods and information on them.")))
        .get_matches();
    if let ("help", Some(help_matches)) = matches.subcommand() {
        if let Some(value) = help_matches.value_of("query") {
            return Ok(Arguments::new_help(HelpWith::Query(DbIndicator::from(value))));
        } else if let Some(value) = help_matches.value_of("show") {
            return Ok(Arguments::new_help(HelpWith::Show(DbIndicator::from(value))));
        } else if help_matches.is_present("conflict resolution") {
            return Ok(Arguments::new_help(HelpWith::ConflictResolution));
        } else { // One of the three is required, this is just for optimization purposes
            unreachable!();
        }
        // If the 'info' command is used, then one of 'query', 'show', or 'merge' must be used, so
        // not having one of them is 'unreachable!()'.
    }
    let mut arguments = Arguments::new();
    if let Some(path) = matches.value_of("osu!.db specifier") {
        arguments.db = Some(Database::OsuDb(read(path)?))
    } else if let Some(path) = matches.value_of("collection.db specifier") {
        arguments.db = Some(Database::CollectionDb(read(path)?));
    } else if let Some(path) = matches.value_of("scores.db specifier") {
        arguments.db = Some(Database::ScoresDb(read(path)?));
    } else { // One of the three is required; this is just for optimization purposes
        unreachable!()
    };
    if let Some(jobs) = matches.value_of("Jobs") {
        let jobs = jobs.parse::<usize>()
            .map_err(|_| IoError::new(Other, "Invalid number of jobs."))?;
        arguments.jobs = Some(jobs);
    } else {
        arguments.jobs = Some(1);
    }
    if let Some(path) = matches.value_of("Merge") {
        let second = Database::new_of_same_type(arguments.db.as_ref().unwrap(),read(path)?);
        let resolution_method = if let Some(resolution_method) = matches.value_of("Resolution") {
            ConflictResolution::from_argument(resolution_method).ok_or_else(|| {
                IoError::new(Other, "Invalid conflict resolution type.")
            })?
        } else {
            ConflictResolution::MergeSubentries
        };
        arguments.merge = Some((second, resolution_method));
    }
    if let Some(interface) = matches.value_of("Interface type") {
        let interface = match interface {
            "n" | "none" => InterfaceType::None,
            "s" | "shell" => InterfaceType::Shell,
            "t" | "tui" => InterfaceType::Tui,
            _ => unreachable!() // No other values are accepted
        };
        arguments.interface = Some(interface);
    } else {
        arguments.interface = Some(InterfaceType::None);
    }
    if let Some(query) = matches.value_of("Database query") {
        if arguments.merge.is_some() {
            return Err(IoError::new(Other, "Queries can only be passed in as an argument when no \
                interface is specified."));
        } else {
            arguments.database_query = Some(query.to_string());
        }
    }
    if let Some(show_opts) = matches.value_of("Show options") {
        if arguments.merge.is_some() {
            return Err(IoError::new(Other, "Show options can only be passed in as an argument when \
                no interface is specified."));
        } else {
            arguments.show_options = Some(show_opts.to_string());
        }
    } else if arguments.interface == Some(InterfaceType::None) {
        arguments.show_options = Some("ALL".to_string());
    }
    Ok(arguments)
}