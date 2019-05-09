extern crate byteorder;

pub mod databases;
mod deserialize_primitives;
mod interactive;
mod query;
mod serialize_primitives;

use std::fs::File;

use databases::osu::OsuDb;

fn main() {
    let mut f = File::open("osu!.db").unwrap();
    let osudb = OsuDb::read_from_file(&mut f);
    if let Ok(_) = osudb {
        println!("Success!");
    } else {
        println!("Fuck.\n{:?}", osudb.unwrap_err());
    }
}
