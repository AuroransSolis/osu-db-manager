#[macro_use] extern crate criterion;

pub mod databases;
pub mod deserialize_primitives;

use databases::osu::OsuDb;

use criterion::Criterion;
use std::fs::File;

criterion_group!{
    name = osu_db_bench;
    config = Criterion::default();
    targets = bench_load_osudb
}

criterion_main!{osu_db_bench}

fn bench_load_osudb(c: &mut Criterion) {
    c.bench_function("Load entire osu!.db", move |b| {
        b.iter(|| {
            let mut file = File::open("osu!.db").unwrap();
            if let Err(_) = OsuDb::read_from_file(&mut file) {
                panic!("Uh oh!");
            }
        })
    });
}