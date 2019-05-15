extern crate byteorder;

mod databases;
mod deserialize_primitives;
mod interactive;
mod query;
mod serialize_primitives;
mod partial_databases;

use std::fs::File;
use std::time::Instant;

use databases::osu::OsuDb;

fn main() {
    let mut f = File::open("osu!.db").unwrap();
    let start = Instant::now();
    let osudb = OsuDb::read_from_file(&mut f);
    println!("Estimated time to run: {:?}", start.elapsed());
    if let Ok(osudb) = osudb {
        println!("Success!");
    } else {
        println!("Fuck.\n{:?}", osudb.unwrap_err());
    }
}