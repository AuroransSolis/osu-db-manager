use std::fs::read;
use std::io::{Result as IoResult, Error as IoError, ErrorKind::Other};
use crate::databases::merge::ConflictResolution;
use clap::{Arg, App, SubCommand};
use std::hint::unreachable_unchecked;

fn create_app<'a, 'b>() -> App<'a, 'b> {
    App::new("osu-db-manager")
        .version("1.0.0")
        .author("Aurorans Solis")
        .about("Tool to read, write, browse, and merge osu! databases.")
        .arg(Arg::with_name("osu!.db specifier")
            .short("o")
            .long("osu")
            .index(1)
            .takes_value(true)
            .value_name("PATH")
            .required_unless_one(&["collection.db specifier", "scores.db specifier"])
            .conflicts_with_all(&["collection.db specifier", "scores.db specifier"])
            .help("Specifies that the given path is to an osu!.db"))
        .arg(Arg::with_name("collection.db specifier")
            .short("c")
            .long("collection")
            .index(1)
            .takes_value(true)
            .value_name("PATH")
            .required_unless_one(&["osu!.db specifier", "scores.db specifier"])
            .conflicts_with_all(&["osu!.db specifier", "scores.db specifier"])
            .help("Specifies that the given path is to a collection.db"))
        .arg(Arg::with_name("scores.db specifier")
            .short("s")
            .long("scores")
            .index(1)
            .takes_value(true)
            .value_name("PATH")
            .required_unless_one(&["osu!.db specifier", "collection.db specifier"])
            .conflicts_with_all(&["osu!.db specifier", "collection.db specifier"])
            .help("Specifies that the given path is to a scores.db"))
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
            .help("Database query. Use 'help ==query TYPE' for information on what you can \
                query in database type TYPE. Can be used in conjunction with show options."))
        .arg(Arg::with_name("Show options")
            .short("s")
            .long("show")
            .value_name("SHOW_OPTIONS")
            .takes_value(true)
            .default_value("ALL")
            .multiple(false)
            .required(false)
            .conflicts_with("Interface type")
            .help("What information to show from each database entry. Use 'help --show TYPE' \
                for information on what you can show from each database type TYPE. Can be used in \
                conjunction with a query."))
        .arg(Arg::with_name("Merge")
            .short("m")
            .long("merge")
            .conflicts_with_all(&["osu!.db specifier", "Database query", "Show options"])
            .takes_value(true)
            .number_of_values(2)
            .value_names(&["CONFLICT_RESOLUTION", "PATH"])
            .default_value("merge-subentries")
            .multiple(false)
            .required(false)
            .help("Merge a second database at location PATH into the second using resolution \
                method CONFLICT_RESOLUTION. The conflict resolution method is only required if no \
                interactive merging method is used. Use 'help --merge' to for information on the \
                available merge conflict resolution methods."))
        .subcommand(SubCommand::with_name("help")
            .about("Subcommand to provide additional information for options for osu-db-manager.")
            .version("1.0.0")
            .author("Aurorans Solis")
            .arg(Arg::with_name("query")
                .long("query")
                .takes_value(true)
                .value_name("TYPE")
                .possible_values(&["o", "osu", "c", "collection", "s", "scores", ""])
                .required_unless_one(&["show", "merge"])
                .conflicts_with_all(&["show", "merge"])
                .help("Shows all available fields to query in database TYPE as well as examples \
                    for querying each field of a database."))
            .arg(Arg::with_name("show")
                .long("show")
                .takes_value(true)
                .value_name("TYPE")
                .possible_values(&["o", "osu", "c", "collection", "s", "scores", ""])
                .required_unless_one(&["query", "merge"])
                .conflicts_with_all(&["query", "merge"])
                .help("Shows all available fields in database TYPE to display."))
            .arg(Arg::with_name("merge")
                .long("merge")
                .takes_value(false)
                .multiple(false)
                .required_unless_one(&["query", "show"])
                .conflicts_with_all(&["query", "show"])
                .help("Shows available merging methods and information on them.")))
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Argument {
    Db(Database),
    Interface(InterfaceType),
    DatabaseQuery(String),
    Show(String),
    Merge((Database, ConflictResolution)),
    Help(HelpWith)
}

impl Argument {
    fn ref_database(&self) -> &Database {
        match self {
            Argument::Db(inner) => inner,
            _ => unreachable!()
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
    Merge
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DbIndicator {
    OsuDb,
    CollectionDb,
    ScoresDb,
    General
}

fn get_arguments<'a, 'b>(app: App<'a, 'b>) -> IoResult<Vec<Argument>> {
    let matches = app.get_matches();
    if let ("help", Some(help_matches)) = matches.subcommand() {
        if let Some(value) = help_matches.value_of("query") {
            return Ok(vec![Argument::Help(HelpWith::Query(match value {
                "o" | "osu" => DbIndicator::OsuDb,
                "c" | "collection" => DbIndicator::CollectionDb,
                "s" | "scores" => DbIndicator::ScoresDb,
                "" => DbIndicator::General,
                _ => unreachable!()
            }))]);
        } else if let Some(value) = help_matches.value_of("show") {
            return Ok(vec![Argument::Help(HelpWith::Show(match value {
                "o" | "osu" => DbIndicator::OsuDb,
                "c" | "collection" => DbIndicator::CollectionDb,
                "s" | "scores" => DbIndicator::ScoresDb,
                "" => DbIndicator::General,
                _ => unreachable!()
            }))]);
        } else if help_matches.is_present("merge") {
            return Ok(vec![Argument::Help(HelpWith::Merge)]);
        } else {
            unreachable!();
        }
        // If the 'help' command is used, then one of 'query', 'show', or 'merge' must be used, so
        // not having one of them is 'unreachable!()'.
    }
    let mut arguments = Vec::new();
    let db = if let Some(path) = matches.value_of("osu!.db specifier") {
        Database::OsuDb(read(path)?)
    } else if let Some(path) = matches.value_of("collection.db specifier") {
        Database::CollectionDb(read(path)?)
    } else if let Some(path) = matches.value_of("scores.db specifier") {
        Database::ScoresDb(read(path)?)
    } else { // One of the three is required; this is just to keep rustc happy
        unreachable!()
    };
    arguments.push(Argument::Db(db));
    if let Some(mut values) = matches.values_of("Merge") {
        let path = values.next().unwrap(); // command is guaranteed to have two values
        let resolution = values.next().unwrap();
        let second = Database::new_of_same_type(&arguments[0].ref_database(),read(path)?);
        let resolution = ConflictResolution::from_argument(resolution)
            .ok_or_else(|| IoError::new(Other, "Invalid conflict resolution option."))?;
        arguments.push(Argument::Merge((second, resolution)));
        arguments.push(if matches.is_present("Interface type") {
            Argument::Interface(match matches.value_of("Interface type").unwrap() {
                "n" | "none" => InterfaceType::None,
                "s" | "shell" => InterfaceType::Shell,
                "t" | "tui" => InterfaceType::Tui,
                _ => unreachable!() // No other values are accepted
            })
        } else {
            Argument::Interface(InterfaceType::None)
        });
        return Ok(arguments);
    }
    if matches.is_present("Interface type") {
        if let Some(interface) = matches.value_of("Interface type") {
            arguments.push(Argument::Interface(match interface {
                "n" | "none" => InterfaceType::None,
                "s" | "shell" => InterfaceType::Shell,
                "t" | "tui" => InterfaceType::Tui,
                _ => unreachable!() // No other values are accepted
            }));
        } else { // If '-i'/'--interface' is present, a value must be present
            unreachable!();
        }
    } else {
        arguments.push(Argument::Interface(InterfaceType::None));
    }
    if let Some(query) = matches.value_of("Database query") {
        if arguments.contains(&Argument::Interface(InterfaceType::Shell))
            || arguments.contains(&Argument::Interface(InterfaceType::Tui)) {
            return Err(IoError::new(Other, "Queries can only be passed in as an argument when no \
                interface is specified."));
        } else {
            arguments.push(Argument::DatabaseQuery(query.to_string()))
        }
    }
    if let Some(show_opts) = matches.value_of("Show options") {
        if arguments.contains(&Argument::Interface(InterfaceType::Shell))
            || arguments.contains(&Argument::Interface(InterfaceType::Tui)) {
            return Err(IoError::new(Other, "Queries can only be passed in as an argument when no \
                interface is specified."));
        } else {
            arguments.push(Argument::Show(show_opts.to_string()))
        }
    }
    Ok(arguments)
}