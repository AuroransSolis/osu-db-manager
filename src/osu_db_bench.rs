#[macro_use] extern crate criterion;

pub mod databases;
pub mod deserialize_primitives;

use databases::osu::OsuDb;

use criterion::{Criterion, black_box};
use std::time::Duration;
use std::fs::File;

criterion_group!{
    name = osu_db_bench;
    config = Criterion::default().sample_size(1000).measurement_time(Duration::from_secs(60));
    targets = bench_load_osudb
}

criterion_main!{osu_db_bench}

fn bench_load_osudb(c: &mut Criterion) {
    let mut file = File::open("osu!.db").unwrap();
    c.bench_function("Load entire osu!.db", move |b| {
        b.iter(|| {
            let mut f_tmp = file.try_clone().unwrap();
            black_box(OsuDb::read_from_file(&mut f_tmp));
        })
    });
}