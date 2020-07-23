mod argument;
mod databases;
mod deserialize_primitives;
mod interactive;
mod load_settings;
mod masks;
mod maybe_deserialize_primitives;
mod read_error;
mod serialize_primitives;

use argument::*;
use databases::database::OsuDatabase;
use load_settings::LoadSettings;
use masks::DbMask;
use std::fs::File;
use std::io::Read;
use std::time::Instant;
use structopt::StructOpt;

fn main() {
    let Arguments {
        db_type,
        db_path,
        jobs,
        merge_search_or_interface,
    } = Arguments::from_args();
    let timer = Instant::now();
    let mut buffer = Vec::new();
    let mut file = File::open(&db_path).expect("Failed to open database file.");
    file.read_to_end(&mut buffer)
        .expect("Failed to read database file.");
    let (database, mask, interface) = if let Some(msi) = merge_search_or_interface {
        match msi {
            MergeSearchOrInterface::Merge(_merge) => {
                println!("Merging not currently supported.");
                return;
            }
            MergeSearchOrInterface::Search { search } => match search {
                Search::OsuSearch {
                    mut load_settings,
                    mask,
                } => {
                    load_settings.set_from_mask(&mask);
                    (
                        OsuDatabase::read_partial_from_bytes(
                            jobs,
                            LoadSettings::OsuSettings(load_settings),
                            &buffer,
                        )
                        .expect("Failed to parse database."),
                        Some(DbMask::OsuMask(mask)),
                        InterfaceType::None,
                    )
                }
                Search::CollectionSearch {
                    mut load_settings,
                    mask,
                } => {
                    load_settings.set_from_mask(&mask);
                    (
                        OsuDatabase::read_partial_from_bytes(
                            jobs,
                            LoadSettings::CollectionSettings(load_settings),
                            &buffer,
                        )
                        .expect("Failed to parse database."),
                        Some(DbMask::CollectionMask(mask)),
                        InterfaceType::None,
                    )
                }
                Search::ScoresSearch {
                    mut load_settings,
                    mask,
                } => {
                    load_settings.set_from_mask(&mask);
                    (
                        OsuDatabase::read_partial_from_bytes(
                            jobs,
                            LoadSettings::ScoresSettings(load_settings),
                            &buffer,
                        )
                        .expect("Failed to parse database."),
                        Some(DbMask::ScoresMask(mask)),
                        InterfaceType::None,
                    )
                }
            },
            MergeSearchOrInterface::Interface { interface } => {
                let database = OsuDatabase::read_from_bytes(jobs, db_type, &buffer)
                    .expect("Failed to parse database.");
                (database, None, interface)
            }
        }
    } else {
        let database = OsuDatabase::read_from_bytes(jobs, db_type, &buffer)
            .expect("Failed to parse database.");
        (database, None, InterfaceType::None)
    };
    match interface {
        InterfaceType::None => database.display(mask),
        _ => println!("Interface {:?} not yet supported.", interface),
    }
    let elapsed = timer.elapsed();
    println!("Successfully loaded database! Time taken: {:?}", elapsed);
}
