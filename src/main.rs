mod databases;
mod deserialize_primitives;
mod interactive;
mod query;
mod serialize_primitives;
// mod partial_databases;
mod help;

use std::fs::read;
use std::time::Instant;

use databases::osu::OsuDb;
use databases::load::Load;
// use crate::databases::osu::OsuDbLoadSettings;

fn main() {
    let jobs = 1;
    let start = Instant::now();
    let file_bytes = read("../jminn-osu!.db").expect("Failed to read file to byte vec.");
    let osudb = OsuDb::read_from_bytes(jobs, file_bytes);
    println!("Estimated time to run: {:?}", start.elapsed());
    if let Ok(osudb) = osudb {
        println!("Success! Loaded in osu!.db with {} beatmaps with {} thread(s).",
            osudb.number_of_beatmaps, jobs);
    } else {
        println!("Fuck.\n{:?}", osudb.unwrap_err());
    }
}
