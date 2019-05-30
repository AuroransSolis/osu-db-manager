#[macro_use] extern crate criterion;

pub mod databases;
pub mod deserialize_primitives;

use databases::osu::OsuDb;

use criterion::{Criterion, black_box};
use std::fs::read;
use crate::databases::load::Load;

const OSUDB_FILE: &str = "jminn-osu!.db";

criterion_group!{
    name = osu_db_bench;
    config = Criterion::default();
    targets = bench_clone_database_vector, bench_load_osudb_1_job, bench_load_osudb_2_jobs,
        bench_load_osudb_3_jobs, bench_load_osudb_4_jobs
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

fn bench_load_osudb_1_job(c: &mut Criterion) {
    c.bench_function("Load entire osu!.db (-j1)", move |b| {
        let file_bytes = read(OSUDB_FILE).unwrap();
        b.iter(|| {
            if let Err(_) = OsuDb::read_from_bytes(1, file_bytes.clone()) {
                panic!("Uh oh!");
            }
        });
    });
}

fn bench_load_osudb_2_jobs(c: &mut Criterion) {
    c.bench_function("Load entire osu!.db (-j2)", move |b| {
        let file_bytes = read(OSUDB_FILE).unwrap();
        b.iter(|| {
            if let Err(_) = OsuDb::read_from_bytes(2, file_bytes.clone()) {
                panic!("Uh oh!");
            }
        });
    });
}

fn bench_load_osudb_3_jobs(c: &mut Criterion) {
    c.bench_function("Load entire osu!.db (-j3)", move |b| {
        let file_bytes = read(OSUDB_FILE).unwrap();
        b.iter(|| {
            if let Err(_) = OsuDb::read_from_bytes(1, file_bytes.clone()) {
                panic!("Uh oh!");
            }
        });
    });
}

fn bench_load_osudb_4_jobs(c: &mut Criterion) {
    c.bench_function("Load entire osu!.db (-j4)", move |b| {
        let file_bytes = read(OSUDB_FILE).unwrap();
        b.iter(|| {
            if let Err(_) = OsuDb::read_from_bytes(4, file_bytes.clone()) {
                panic!("Uh oh!");
            }
        });
    });
}