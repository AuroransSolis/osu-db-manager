mod argument;
mod databases;
mod deserialize_primitives;
mod masks;
mod maybe_deserialize_primitives;

use crate::databases::{load::Load, osu::osudb::OsuDb, scores::scoresdb::ScoresDb};
mod read_error;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs::read;

const OSUDB_FILES: [&str; 4] = [
    "auro-osu!.db",
    "bigger-tama-osu!.db",
    "jminn-osu!.db",
    "tama-osu!.db",
];
const SCORESDB_FILES: [&str; 3] = ["new-auro-scores.db", "old-auro-scores.db", "rtrx-scores.db"];

fn osudb_benchmarks(c: &mut Criterion) {
    OSUDB_FILES
        .iter()
        .copied()
        .map(|name| {
            (
                name,
                read(name).expect("Failed to open osudb file for benchmarking"),
            )
        })
        .flat_map(|(name, bytes)| (1..=num_cpus::get()).map(|jobs| (name, bytes, jobs)))
        .for_each(|(name, bytes, jobs)| {
            c.bench_function(&format!("{}: {} job(s)", name, jobs), move |b| {
                b.iter(|| {
                    if let Err(_) = OsuDb::read_from_bytes(jobs, &bytes) {
                        panic!("Uh oh!");
                    }
                });
            });
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
        .flat_map(|(name, bytes)| (1..=num_cpus::get()).map(|jobs| (name, bytes, jobs)))
        .for_each(|(name, bytes, jobs)| {
            c.bench_function(&format!("{}: {} job(s)", name, jobs), move |b| {
                b.iter(|| {
                    if let Err(_) = ScoresDb::read_from_bytes(jobs, &bytes) {
                        panic!("Uh oh!");
                    }
                });
            });
        });
}

criterion_group! {
    name = osu_db_bench;
    config = Criterion::default();
    targets = osudb_benchmarks, scoresdb_benchmarks
}

criterion_main! {osu_db_bench}
