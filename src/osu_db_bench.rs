#[macro_use] extern crate criterion;
extern crate num_cpus;

mod databases;
mod deserialize_primitives;
mod argument;
mod maybe_deserialize_primitives;
mod masks;

use crate::databases::{osu::osudb::OsuDb, scores::scoresdb::ScoresDb, load::Load};
mod read_error;

use criterion::{Criterion, black_box};
use std::fs::read;

const OSUDB_FILE: &str = "tama-osu!.db";
const SCORESDB_FILE: &str = "old-auro-scores.db";

macro_rules! make_bench {
    ($(($number:literal, $osu_name:ident, $scores_name:ident)),*) => {
        $(
            fn $osu_name(c: &mut Criterion) {
                let id = format!("Load entire osu!.db (-j{})", $number);
                c.bench_function(id.as_str(), move |b| {
                    let file_bytes = read(OSUDB_FILE).unwrap();
                    b.iter_with_setup(|| file_bytes.clone(), |fb| {
                        if let Err(_) = OsuDb::read_from_bytes($number, fb) {
                            panic!("Uh oh!");
                        }
                    });
                });
            }
            
            fn $scores_name(c: &mut Criterion) {
                let id = format!("Load entire scores.db (-j{})", $number);
                c.bench_function(id.as_str(), move |b| {
                    let file_bytes = read(SCORESDB_FILE).unwrap();
                    b.iter_with_setup(|| file_bytes.clone(), |fb| {
                        if let Err(_) = ScoresDb::read_from_bytes($number, fb) {
                            panic!("Uh oh!");
                        }
                    });
                });
            }
        )*
        criterion_group!{
            name = osu_db_bench;
            config = Criterion::default();
            targets = bench_open_osudb_file, bench_open_scoresdb_file, $($osu_name, $scores_name),*
        }
    }
}

// Uncomment up to the number of threads available on your CPU
// Yes, I could make a proc macro for this, but I'm feeling lazy so no.
make_bench!{
    (1, load_osudb_jobs_1, load_scoresdb_jobs_1),
    (2, load_osudb_jobs_2, load_scoresdb_jobs_2),
    (3, load_osudb_jobs_3, load_scoresdb_jobs_3),
    (4, load_osudb_jobs_4, load_scoresdb_jobs_4),
    (5, load_osudb_jobs_5, load_scoresdb_jobs_5),
    (6, load_osudb_jobs_6, load_scoresdb_jobs_6),
    (7, load_osudb_jobs_7, load_scoresdb_jobs_7),
    (8, load_osudb_jobs_8, load_scoresdb_jobs_8)
    /*
    (9, load_osudb_jobs_9, load_scoresdb_jobs_9),
    (10, load_osudb_jobs_10, load_scoresdb_jobs_10),
    (11, load_osudb_jobs_11, load_scoresdb_jobs_11),
    (12, load_osudb_jobs_12, load_scoresdb_jobs_12),
    (13, load_osudb_jobs_13, load_scoresdb_jobs_13),
    (14, load_osudb_jobs_14, load_scoresdb_jobs_14),
    (15, load_osudb_jobs_15, load_scoresdb_jobs_15),
    (16, load_osudb_jobs_16, load_scoresdb_jobs_16)
    */
}

criterion_main!{osu_db_bench}

fn bench_open_osudb_file(c: &mut Criterion) {
    c.bench_function("Opening osu!.db file and reading to vector", move |b| {
        b.iter(|| {
            if let Err(_) = read(OSUDB_FILE) {
                panic!("Uh oh!");
            }
        });
    });
}

fn bench_open_scoresdb_file(c: &mut Criterion) {
    c.bench_function("Opening scores.db and reading to vector", move |b| {
        b.iter(|| {
            if let Err(_) = read(SCORESDB_FILE) {
                panic!("Uh oh!");
            }
        });
    });
}