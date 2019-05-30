extern crate byteorder;
extern crate rayon;

mod databases;
mod deserialize_primitives;
mod interactive;
mod query;
mod serialize_primitives;
// mod partial_databases;
mod help;

use std::fs::{File, read};
use std::time::Instant;
use std::io::Cursor;
use std::sync::{Arc, Mutex, atomic::AtomicUsize};

use rayon::{ThreadPoolBuilder, ThreadPoolBuildError};

use databases::osu::OsuDb;
use databases::load::Load;
// use crate::databases::osu::OsuDbLoadSettings;

fn main() {
    let jobs = 1;
    let start = Instant::now();
    let file_bytes = read("tama-osu!.db").unwrap();
    let osudb = OsuDb::read_from_bytes(jobs, file_bytes);
    println!("Estimated time to run: {:?}", start.elapsed());
    if let Ok(osudb) = osudb {
        println!("Success! Loaded in osu!.db with {} beatmaps with {} thread(s).",
            osudb.number_of_beatmaps, jobs);
    } else {
        println!("Fuck.\n{:?}", osudb.unwrap_err());
    }
}

pub fn build_global_threadpool_with_jobs(jobs: usize) -> Result<(), ThreadPoolBuildError> {
    ThreadPoolBuilder::new().num_threads(jobs).build_global()
}