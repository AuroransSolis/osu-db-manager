#![allow(dead_code)]

mod argument;
mod databases;
mod deserialize_primitives;
mod load_settings;
mod masks;
mod maybe_deserialize_primitives;
mod read_error;

use crate::databases::{osu::osudb::OsuDb, scores::scoresdb::ScoresDb};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs::read;

const OSUDB_FILES: [(&str, Option<usize>); 5] = [
    ("auro-osu!.db", None),
    ("bigger-tama-osu!.db", None),
    ("even-bigger-tama-osu!.db", Some(1)),
    ("jminn-osu!.db", None),
    ("tama-osu!.db", None),
];
const SCORESDB_FILES: [&str; 3] = ["new-auro-scores.db", "old-auro-scores.db", "rtrx-scores.db"];

fn osudb_benchmarks(c: &mut Criterion) {
    OSUDB_FILES
        .iter()
        .copied()
        .map(|(name, job_limit)| {
            (
                name,
                job_limit,
                read(name).expect("Failed to open osudb file for benchmarking"),
            )
        })
        .for_each(|(name, jobs_limit, bytes)| {
            for jobs in 1..=jobs_limit.unwrap_or(num_cpus::get()) {
                c.bench_function(&format!("{}: {} job(s)", name, jobs), |b| {
                    b.iter(|| {
                        if let Err(_) = OsuDb::read_from_bytes(jobs, &bytes) {
                            panic!("Uh oh!");
                        }
                    });
                });
            }
        });
}

fn scoresdb_benchmarks(c: &mut Criterion) {
    SCORESDB_FILES
        .iter()
        .copied()
        .map(|name| {
            (
                name,
                read(name).expect("Failed to open scoresdb file for benchmarking"),
            )
        })
        .for_each(|(name, bytes)| {
            for jobs in 1..=num_cpus::get() {
                c.bench_function(&format!("{}: {} job(s)", name, jobs), |b| {
                    b.iter(|| {
                        if let Err(_) = ScoresDb::read_from_bytes(jobs, &bytes) {
                            panic!("Uh oh!");
                        }
                    });
                });
            }
        });
}

criterion_group! {
    name = osu_db_bench;
    config = Criterion::default();
    targets = osudb_benchmarks, scoresdb_benchmarks
}

criterion_main! {osu_db_bench}
