use crate::databases::merge::ConflictResolution;

enum ArgumentParseErrorKind {
    InvalidArgument,
    UnknownArgument,
    MissingArgument
}

use self::ArgumentParseErrorKind::*;

struct ArgumentParseError {
    kind: ArgumentParseErrorKind,
    message: &str
}

impl ArgumentParseError {
    fn new(kind: ArgumentParseErrorKind, message: &str) -> Self {
        ArgumentParseError {
            kind,
            message
        }
    }
}

enum Argument {
    Jobs(usize),
    Database((Database, String)),
    Interface(Interface),
    Query(DbQuery),
    Show(ShowOptions),
    Merge((String, String, ConflictResolution))
}

use self::Argument::*;
use core::num::dec2flt::parse::ParseResult::Invalid;

impl Argument {
    fn from_strings(strings: Vec<String>) -> Result<Vec<Self>, ArgumentParseError> {
        let mut strings = strings.into_iter();
        let mut args = vec![{
            Argument::Database(
                (Database::from_string(strings.next()
                    .ok_or_else(|| ArgumentParseError::new(MissingArgument,
                        "Missing database type specifier.")?))?,
                strings.next().ok_or_else(|| ArgumentParseError::new(MissingArgument,
                    "Missing databasee path."))?
                )
            )
        }];
        while let Some(string) = strings.next() {
            let short_form = match string {
                s if &s[0..2] == "-j" => {
                    let number = (&s[2..]).parse::<usize>().map_err(|_| {
                        let msg = format!("Invalid argument for jobs number: {}", string);
                        ArgumentParseError::new(InvalidArgument, msg.as_str())
                    })?;
                    Ok(Jobs(number))
                },
                "-o" => Ok(Database(Database::Osu)),
                "-c" => Ok(Database(Database::Collection)),
                "-s" => Ok(Database(Database::Scores)),
                "-i" => {
                    let
                }
            };
            match string {

            }
        }
    }
}

enum Database {
    Osu,
    Collection,
    Scores
}

impl Database {
    fn from_string(string: String) -> Result<Self, ArgumentParseError> {
        match string.as_str() {
            "-o" | "--osu" => Ok(Database::Osu),
            "-c" | "--collection" => Ok(Database::Collection),
            "-s" | "--scores" => Ok(Database::Scores),
            _ => {
                let msg = format!("Invalid database type: {}", string);
                Err(ArgumentParseError::new(InvalidArgument, msg.as_str()))
            }
        }
    }
}

enum Interface {
    None,
    Shell,
    Tui
}

enum DbQuery {
    OsuQuery,
    CollectionQuery,
    ScoresQuery
}

enum ShowOptions {
    OsuShow,
    CollectionShow,
    ScoresShow
}