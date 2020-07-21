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
use std::fs::File;
use std::io::Read;
use std::time::Instant;
use structopt::StructOpt;

fn main() {
    let arguments = Arguments::from_args();
    let Arguments {
        db_type,
        db_path,
        jobs,
        ..
    } = arguments;
    let timer = Instant::now();
    let mut buffer = Vec::new();
    let mut file = File::open(&db_path).expect("Failed to open database file.");
    file.read_to_end(&mut buffer)
        .expect("Failed to read database file.");
    let database = OsuDatabase::read_from_bytes(jobs, db_type, &buffer);
    let elapsed = timer.elapsed();
    if let Ok(_) = database {
        println!("Successfully loaded database! Time taken: {:?}", elapsed);
    } else {
        println!("Fuck.\n{:?}", database.unwrap_err());
    }
}
