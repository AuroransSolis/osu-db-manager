#[macro_use] extern crate criterion;
extern crate num_cpus;

pub mod databases;
pub mod deserialize_primitives;

use crate::databases::{osu::OsuDb, load::Load};

use criterion::{Criterion, black_box};
use std::fs::read;

const OSUDB_FILE: &str = "tama-osu!.db";

macro_rules! make_bench {
    ($(($number:literal, $name:ident)),*) => {
        $(
            fn $name(c: &mut Criterion) {
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
        )*
        criterion_group!{
            name = osu_db_bench;
            config = Criterion::default();
            targets = bench_open_database_file, $($name),*
        }
    }
}

// Uncomment up to the number of threads available on your CPU
// Yes, I could make a proc macro for this, but I'm feeling lazy so no.
make_bench!{
    (1, load_osudb_jobs_1), (2, load_osudb_jobs_2), (3, load_osudb_jobs_3), (4, load_osudb_jobs_4)
    /*
    (5, load_osudb_jobs_1), (6, load_osudb_jobs_2), (7, load_osudb_jobs_3), (8, load_osudb_jobs_4),
    (9, load_osudb_jobs_1), (10, load_osudb_jobs_2), (11, load_osudb_jobs_3),
    (12, load_osudb_jobs_4), (13, load_osudb_jobs_1), (14, load_osudb_jobs_2),
    (15, load_osudb_jobs_3), (16, load_osudb_jobs_4)
    */
}

criterion_main!{osu_db_bench}

fn bench_clone_database_vector(c: &mut Criterion) {
    c.bench_function("Cloning osu!.db vector.", move |b| {
        let file_bytes = read(OSUDB_FILE).unwrap();
        b.iter(|| {
            black_box(file_bytes.clone());
        });
    });
}

fn bench_open_database_file(c: &mut Criterion) {
    c.bench_function("Opening osu!.db file and reading to vector", move |b| {
        b.iter(|| {
            if let Err(_) = read(OSUDB_FILE) {
                panic!("Uh oh!");
            }
        });
    });
}