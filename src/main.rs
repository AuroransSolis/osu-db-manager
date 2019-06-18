extern crate clap;
extern crate num_cpus;

mod databases;
mod deserialize_primitives;
mod interactive;
mod query;
mod serialize_primitives;
mod argument;
mod read_error;
mod masks;

use std::time::Instant;

use databases::database::OsuDatabase;
use argument::*;

fn main() {
    let arguments = get_arguments().unwrap();
    if let Some(info) = arguments.info {
        println!("Got help command!");
        return;
    }
    let Arguments { db, jobs, .. } = arguments;
    let database = db.unwrap();
    let jobs = jobs.unwrap();
    let timer = Instant::now();
    let database = OsuDatabase::read_from_bytes(jobs, database);
    let elapsed = timer.elapsed();
    if let Ok(_) = database {
        println!("Successfully loaded database! Time taken: {:?}", elapsed);
    } else {
        println!("Fuck.\n{:?}", database.unwrap_err());
    }
}