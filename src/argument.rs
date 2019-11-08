use std::fs::read;
use std::io::{Error as IoError, ErrorKind::Other, Result as IoResult};

use clap::{App, AppSettings, Arg, ArgGroup, SubCommand};

use crate::databases::merge::ConflictResolution;

pub struct Arguments {
    pub db: Option<Database>,
    pub verbosity: Verbosity,
    pub jobs: Option<usize>,
    pub interface: Option<InterfaceType>,
    pub database_query: Option<String>,
    pub show_options: Option<String>,
    pub merge: Option<(Database, ConflictResolution)>,
    pub info: Option<Info>,
}

impl Arguments {
    fn new() -> Self {
        Arguments {
            db: None,
            verbosity: Verbosity::Quiet,
            jobs: None,
            interface: None,
            database_query: None,
            show_options: None,
            merge: None,
            info: None,
        }
    }

    fn new_info(info: Info) -> Self {
        Arguments {
            db: None,
            jobs: None,
            verbosity: Verbosity::Quiet,
            interface: None,
            database_query: None,
            show_options: None,
            merge: None,
            info: Some(info),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Database {
    OsuDb(Vec<u8>),
    CollectionDb(Vec<u8>),
    ScoresDb(Vec<u8>),
}

impl Database {
    fn new_of_same_type(first: &Self, bytes: Vec<u8>) -> Self {
        match first {
            &Database::OsuDb(_) => Database::OsuDb(bytes),
            &Database::CollectionDb(_) => Database::CollectionDb(bytes),
            &Database::ScoresDb(_) => Database::ScoresDb(bytes),
        }
    }
}

pub enum Verbosity {
    Quiet,
    Low,
    High,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum InterfaceType {
    Shell,
    Tui,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Info {
    Query(DbIndicator),
    Show(DbIndicator),
    ConflictResolution,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DbIndicator {
    OsuDb,
    CollectionDb,
    ScoresDb,
}

impl From<&str> for DbIndicator {
    fn from(other: &str) -> Self {
        match other {
            "o" | "osu" => DbIndicator::OsuDb,
            "c" | "collection" => DbIndicator::CollectionDb,
            "s" | "scores" => DbIndicator::ScoresDb,
            _ => unreachable!(),
        }
    }
}

pub fn get_arguments() -> IoResult<Arguments> {
    let matches = App::new("osu-db-manager")
        .version("1.0.0")
        .author("Aurorans Solis")
        .about("Tool to read, write, browse, and merge osu! databases.")
        .setting(AppSettings::SubcommandsNegateReqs)
        .arg(Arg::with_name("osu!.db specifier")
            .short("o")
            .long("osu")
            .takes_value(true)
            .value_name("PATH")
            .help("Specifies that the given path is to an osu!.db"))
        .arg(Arg::with_name("collection.db specifier")
            .short("c")
            .long("collection")
            .takes_value(true)
            .value_name("PATH")
            .help("Specifies that the given path is to a collection.db"))
        .arg(Arg::with_name("scores.db specifier")
            .short("s")
            .long("scores")
            .takes_value(true)
            .value_name("PATH")
            .help("Specifies that the given path is to a scores.db"))
        .group(ArgGroup::with_name("Database type indicator and path")
            .args(&["osu!.db specifier", "collection.db specifier", "scores.db specifier"])
            .multiple(false)
            .required(true))
        .arg(Arg::with_name("Verbose")
            .short("v")
            .long("verbose")
            .takes_value(false)
            .help("Show verbose output."))
        .arg(Arg::with_name("Extra verbose")
            .short("vv")
            .long("extra-verbose")
            .takes_value(false)
            .help("Show highly verbose output."))
        .group(ArgGroup::with_name("Verbosity")
            .args(&["Verbose", "Extra verbose"])
            .multiple(false)
            .conflicts_with("Interface type")
            .required(false))
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
            .conflicts_with_all(&["Database query", "Show options"])
            .takes_value(true)
            .value_name("INTERFACE_TYPE")
            .possible_values(&["s", "shell", "t", "tui"])
            .multiple(false)
            .required(false)
            .help("Alternative interface type specifier. Valid interfaces: 't'/'tui', 's'/'shell'. Default \
                is none (print requested information and quit)."))
        .arg(Arg::with_name("Database query")
            .short("q")
            .long("query")
            .value_name("QUERY")
            .takes_value(true)
            .required(false)
            .help("Database query. Use 'info --query TYPE' for information on what you can \
                query in database type TYPE. Can be used in conjunction with show options."))
        .arg(Arg::with_name("Show options")
            .short("S")
            .long("show")
            .value_name("SHOW_OPTIONS")
            .takes_value(true)
            .required(false)
            .help("What information to show from each database entry. Use 'info --show TYPE' \
                for information on what you can show from each database type TYPE. Can be used in \
                conjunction with a query."))
        .arg(Arg::with_name("Merge")
            .short("m")
            .long("merge")
            .takes_value(true)
            .min_values(1)
            .max_values(2)
            .value_names(&["PATH", "OSU_DB_PATH"])
            .multiple(true)
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
    if let ("info", Some(info_matches)) = matches.subcommand() {
        if let Some(value) = info_matches.value_of("query") {
            return Ok(Arguments::new_info(Info::Query(DbIndicator::from(value))));
        } else if let Some(value) = info_matches.value_of("show") {
            return Ok(Arguments::new_info(Info::Show(DbIndicator::from(value))));
        } else if info_matches.is_present("conflict resolution") {
            return Ok(Arguments::new_info(Info::ConflictResolution));
        } else {
            // One of the three is required, this is just for optimization purposes
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
    } else {
        // One of the three is required; this is just for optimization purposes
        unreachable!()
    };
    if let Some(jobs) = matches.value_of("Jobs") {
        let jobs = jobs
            .parse::<usize>()
            .map_err(|_| IoError::new(Other, "Invalid number of jobs."))?;
        arguments.jobs = Some(jobs);
    } else {
        arguments.jobs = Some(1);
    }
    let merge = if let Some(path) = matches.value_of("Merge") {
        let second = Database::new_of_same_type(arguments.db.as_ref().unwrap(), read(path)?);
        Some(second)
    } else {
        None
    };
    if let Some(interface) = matches.value_of("Interface type") {
        let interface = match interface {
            "s" | "shell" => InterfaceType::Shell,
            "t" | "tui" => InterfaceType::Tui,
            _ => unreachable!(), // No other values are accepted
        };
        arguments.interface = Some(interface);
        return Ok(arguments);
    }
    let resolution = if let Some(res_method) = matches.value_of("Resolution") {
        let res_method = ConflictResolution::from_argument(res_method).ok_or_else(|| {
            IoError::new(
                Other,
                "Unrecognized merge conflict resolution method. Valid methods:\n\
                 ignore-duplicates, replace-destination, merge-subentries,\
                 rename-source-with-prefix=PREFIX, rename-source-with-suffix=SUFFIX,\
                 rename-destination-with-prefix=PREFIX, rename-destination-with-suffix=SUFFIX\n\n\
                 Note: prefix/suffix methods must be put in quotes, for example:\n    \
                 -r \"rename-source-with-suffix=foo\"",
            )
        })?;
        Some(res_method)
    } else {
        None
    };
    if let Some(merge) = merge {
        if let Some(resolution) = resolution {
            arguments.merge = Some((merge, resolution));
        } else {
            arguments.merge = Some((merge, ConflictResolution::MergeSubentries))
        }
    } else if resolution.is_some() {
        return Err(IoError::new(
            Other,
            "Conflict resolution method specified without merge argument.",
        ));
    }
    if let Some(query) = matches.value_of("Database query") {
        if arguments.merge.is_some() {
            return Err(IoError::new(
                Other,
                "Queries can only be passed in as an argument when no \
                 interface is specified.",
            ));
        } else {
            arguments.database_query = Some(query.to_string());
        }
    }
    if let Some(show_opts) = matches.value_of("Show options") {
        if arguments.merge.is_some() {
            return Err(IoError::new(
                Other,
                "Show options can only be passed in as an argument when \
                 no interface is specified.",
            ));
        } else {
            arguments.show_options = Some(show_opts.to_string());
        }
    } else if arguments.interface == Some(InterfaceType::None) {
        arguments.show_options = Some("ALL".to_string());
    }
    Ok(arguments)
}
