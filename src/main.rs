mod databases;
mod deserialize_primitives;
mod interactive;
mod query;
mod serialize_primitives;
// mod partial_databases;
mod help;
mod argument;

use std::fs::read;
use std::time::Instant;

use databases::{load::Load, osu::OsuDb, scores::ScoresDb, collection::CollectionDb};

fn main() {
    let jobs = 1;
    let osudb_path = "tama-osu!.db";
    // let scoresdb_path = "old-auro-scores.db";
    // let collectiondb_path = "";
    read_osudb(jobs, osudb_path);
    // read_scoresdb(jobs, scoresdb_path);
    // read_collectiondb(jobs, collectiondb_path);
}

fn read_osudb(jobs: usize, path: &str) {
    let start = Instant::now();
    let file_bytes = read(path).expect("Failed to read file to byte vec.");
    let osudb = OsuDb::read_from_bytes(jobs, file_bytes);
    println!("Estimated time to run: {:?}", start.elapsed());
    if let Ok(osudb) = osudb {
        println!("Success! Loaded in osu!.db with {} beatmaps with {} thread(s).",
            osudb.number_of_beatmaps, jobs);
        println!("{} timing points", osudb.beatmaps.iter().map(|beatmap| beatmap.num_timing_points as usize).sum::<usize>());
    } else {
        println!("Fuck.\n{:?}", osudb.unwrap_err());
    }
}

fn read_scoresdb(jobs: usize, path: &str) {
    let start = Instant::now();
    let file_bytes = read(path).expect("Failed to read file to byte vec.");
    let scoresdb = ScoresDb::read_from_bytes(jobs, file_bytes);
    println!("Estimated time to run: {:?}", start.elapsed());
    if let Ok(scoresdb) = scoresdb {
        println!("Success! Loaded in scores.db with {} beatmaps and {} scores with {} thread(s).",
            scoresdb.number_of_beatmaps, scoresdb.beatmaps.iter()
                .map(|beatmap| beatmap.number_of_scores as usize).sum::<usize>(), jobs);
    } else {
        println!("Fuck.\n{:?}", scoresdb.unwrap_err());
    }
}

fn read_collectiondb(jobs: usize, path: &str) {
    let start = Instant::now();
    let file_bytes = read(path).expect("Failed to read file to byte vec.");
    let collectiondb = CollectionDb::read_from_bytes(jobs, file_bytes);
    println!("Estimated time to run: {:?}", start.elapsed());
    if let Ok(collectiondb) = collectiondb {
        println!("Success! Loaded in collection.db with {} collections with {} thread(s).",
            collectiondb.number_of_collections, jobs);
    } else {
        println!("Fuck.\n{:?}", collectiondb.unwrap_err());
    }
}