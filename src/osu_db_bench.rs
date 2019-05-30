#[macro_use] extern crate criterion;

pub mod databases;
pub mod deserialize_primitives;

use databases::osu::OsuDb;

use criterion::{Criterion, black_box};
use std::fs::read;
use crate::databases::load::Load;

const OSUDB_FILE: &str = "jminn-osu!.db";

macro_rules! make_bench {
    ($(($number:literal, $name:ident)),*) => {
        $(
            fn $name(c: &mut Criterion) {
                let id = format!("Load entire osu!.db (-j{})", $number);
                c.bench_function(id.as_str(), move |b| {
                    let file_bytes = read(OSUDB_FILE).unwrap();
                    b.iter(|| {
                        if let Err(_) = OsuDb::read_from_bytes($number, file_bytes.clone()) {
                            panic!("Uh oh!");
                        }
                    });
                });
            }
        )*
        criterion_group!{
            name = osu_db_bench;
            config = Criterion::default();
            targets = bench_clone_database_vector, $($name),*
        }
    }
}

make_bench!{
    (1, load_osudb_jobs_1), (2, load_osudb_jobs_2), (3, load_osudb_jobs_3), (4, load_osudb_jobs_4),
    (5, load_osudb_jobs_5), (6, load_osudb_jobs_6), (7, load_osudb_jobs_7), (8, load_osudb_jobs_8)
}

/*criterion_group!{
    name = osu_db_bench;
    config = Criterion::default();
    targets = bench_clone_database_vector, bench_load_osudb_1_job, bench_load_osudb_2_jobs,
        bench_load_osudb_3_jobs, bench_load_osudb_4_jobs
}*/

criterion_main!{osu_db_bench}

fn bench_clone_database_vector(c: &mut Criterion) {
    c.bench_function("Cloning osu!.db vector.", move |b| {
        let file_bytes = read(OSUDB_FILE).unwrap();
        b.iter(|| {
            black_box(file_bytes.clone());
        });
    });
}