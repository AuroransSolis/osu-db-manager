extern crate chrono;
extern crate clap;

mod argument;
mod databases;
mod deserialize_primitives;
mod interactive;
mod load_settings;
mod masks;
mod maybe_deserialize_primitives;
mod read_error;
mod serialize_primitives;

use std::time::Instant;

use argument::*;
use databases::database::OsuDatabase;

fn main() {
    let arguments = get_arguments().unwrap();
    if let Some(_) = arguments.info {
        println!("Got help command!");
        return;
    }
    let Arguments { db, jobs, .. } = arguments;
    let database = db.unwrap();
    let jobs = jobs.unwrap();
    let timer = Instant::now();
    let database = OsuDatabase::read_from_bytes(jobs, &database);
    let elapsed = timer.elapsed();
    if let Ok(_) = database {
        println!("Successfully loaded database! Time taken: {:?}", elapsed);
    } else {
        println!("Fuck.\n{:?}", database.unwrap_err());
    }
}
