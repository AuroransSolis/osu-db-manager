extern crate byteorder;

mod databases;
mod deserialize_primitives;
mod interactive;
mod query;
mod serialize_primitives;
//mod partial_databases;

use std::fs::File;
use std::time::Instant;
use std::io::Cursor;
use std::sync::{Arc, Mutex, atomic::AtomicUsize};

use byteorder::{ReadBytesExt, WriteBytesExt};

use databases::osu::OsuDb;
use databases::load::Load;
use crate::databases::osu::OsuDbLoadSettings;

fn main() {
    let mut f = File::open("auro-osu!.db").unwrap();
    let start = Instant::now();
    let load_settings = OsuDbLoadSettings::new(1);
    let osudb = OsuDb::read_from_file(load_settings, f);
    println!("Estimated time to run: {:?}", start.elapsed());
    if let Ok(osudb) = osudb {
        println!("Success! Loaded in osu!.db with {} beatmaps.", osudb.number_of_beatmaps);
    } else {
        println!("Fuck.\n{:?}", osudb.unwrap_err());
    }
}

pub fn clone_file(file: &File) -> File {
    if cfg!(target_os = "linux") {
        let rawfd = file.as_raw_fd();
        unsafe { File::from_raw_fd(rawfd) }
    } else {
        let raw_handle = file.as_raw_handle();
        unsafe { File::from_raw_handle(raw_handle) }
    }
}