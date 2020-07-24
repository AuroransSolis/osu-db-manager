#![allow(dead_code)]

mod deserialize_primitives;
mod read_error;

use chrono::{naive::NaiveDate, Duration as ChronoDuration};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::{thread_rng, Rng};
use read_error::{DbFileParseError, ParseErrorKind::PrimitiveError};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use deserialize_primitives::*;

fn bench_read_byte(c: &mut Criterion) {
    c.bench_function("Read byte", move |b| {
        let mut rng = thread_rng();
        let num = rng.gen();
        let byte = [num];
        b.iter(|| {
            assert!(Ok(num) == read_byte(&byte, &mut 0));
        });
    });
}

fn bench_read_short(c: &mut Criterion) {
    c.bench_function("Read short", move |b| {
        let mut rng = thread_rng();
        let num = rng.gen::<i16>();
        let bytes = i16::to_le_bytes(num);
        b.iter(|| {
            assert!(Ok(num) == read_short(&bytes, &mut 0));
        });
    });
}

fn bench_read_int(c: &mut Criterion) {
    c.bench_function("Read int", move |b| {
        let mut rng = thread_rng();
        let num = rng.gen::<i32>();
        let bytes = i32::to_le_bytes(num);
        b.iter(|| {
            assert!(Ok(num) == read_int(&bytes, &mut 0));
        });
    });
}

fn bench_read_long(c: &mut Criterion) {
    c.bench_function("Read long", move |b| {
        let mut rng = thread_rng();
        let num = rng.gen::<i64>();
        let bytes = i64::to_le_bytes(num);
        b.iter(|| {
            assert!(Ok(num) == read_long(&bytes, &mut 0));
        });
    });
}

fn bench_read_uleb128_1_byte(c: &mut Criterion) {
    c.bench_function("Read ULEB128 integer (1 byte)", move |b| {
        let byte = [0];
        b.iter(|| {
            if let Err(_) = read_uleb128(&byte, &mut 0) {
                panic!("Uh oh!");
            }
        });
    });
}

fn bench_read_uleb128_2_bytes(c: &mut Criterion) {
    c.bench_function("Read ULEB128 integer (2 bytes)", move |b| {
        let bytes = [0b11101010, 0b01000110];
        b.iter(|| {
            if let Err(_) = read_uleb128(&bytes, &mut 0) {
                panic!("Uh oh!");
            }
        });
    });
}

fn bench_read_single(c: &mut Criterion) {
    c.bench_function("Read single", move |b| {
        let mut rng = thread_rng();
        let num = rng.gen::<u32>();
        let bytes = u32::to_le_bytes(num);
        let num = f32::from_bits(num);
        b.iter(|| {
            assert!(Ok(num) == read_single(&bytes, &mut 0));
        });
    });
}

fn bench_read_double(c: &mut Criterion) {
    c.bench_function("Read double", move |b| {
        let mut rng = thread_rng();
        let num = rng.gen::<u64>();
        let bytes = u64::to_le_bytes(num);
        let num = f64::from_bits(num);
        b.iter(|| {
            assert!(Ok(num) == read_double(&bytes, &mut 0));
        });
    });
}

fn bench_read_str_empty(c: &mut Criterion) {
    c.bench_function("Read str (empty)", move |b| {
        let byte = [0];
        b.iter(|| {
            if let Err(_) = read_str_utf8(&byte, &mut 0, "") {
                panic!("Uh oh!");
            }
        });
    });
}

fn bench_read_str(c: &mut Criterion) {
    c.bench_function("Read str (13 bytes)", move |b| {
        let bytes = [
            0x0b, 13, 72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33,
        ];
        b.iter(|| {
            if let Err(_) = read_str_utf8(&bytes, &mut 0, "") {
                panic!("Uh oh!");
            }
        });
    });
}

fn bench_read_boolean(c: &mut Criterion) {
    c.bench_function("Read boolean", move |b| {
        let mut rng = thread_rng();
        let boolean = rng.gen::<bool>();
        let byte = [boolean as u8];
        b.iter(|| {
            assert!(Ok(boolean) == read_boolean(&byte, &mut 0));
        });
    });
}

fn bench_read_datetime(c: &mut Criterion) {
    c.bench_function("Read datetime", move |b| {
        let mut rng = thread_rng();
        let offset_100_nanosecs = rng.gen::<u64>();
        let duration = Duration::from_secs(offset_100_nanosecs / 10_000_000);
        let chrono_duration = ChronoDuration::from_std(duration).map_err(|e| {
            let msg = format!(
                "Failed to convert std::time::Duration to chrono::Duration\n\
             {}",
                e
            );
            DbFileParseError::new(PrimitiveError, msg)
        });
        let chrono_duration = match chrono_duration {
            Ok(chrono_duration) => chrono_duration,
            Err(ref e) => {
                println!(
                    "Tried to benchmark invalid datetime with offset of {:?}.\n{:?}",
                    chrono_duration, e
                );
                return;
            }
        };
        let datetime = NaiveDate::from_ymd(1970, 1, 1) + chrono_duration;
        let bytes = u64::to_le_bytes(offset_100_nanosecs);
        b.iter(|| {
            assert!(Ok(datetime) == read_datetime(&bytes, &mut 0));
        });
    });
}

fn bench_read_md5_hash(c: &mut Criterion) {
    c.bench_function("Read MD5 hash", move |b| {
        let bytes = [
            0x0b, 32, 97, 97, 54, 99, 52, 49, 49, 98, 101, 49, 101, 99, 53, 55, 55, 51, 50, 100,
            97, 48, 57, 98, 101, 97, 50, 56, 52, 98, 100, 50, 48, 48,
        ];
        b.iter(|| {
            if let Err(_) = read_md5_hash(&bytes, &mut 0) {
                panic!("Uh oh!");
            }
        });
    });
}

fn bench_read_player_name(c: &mut Criterion) {
    c.bench_function("Read player name", move |b| {
        let bytes = [0x0b, 10, 80, 114, 105, 109, 97, 32, 76, 117, 99, 101];
        b.iter(|| {
            if let Err(_) = read_player_name(&bytes, &mut 0) {
                panic!("Uh oh!");
            }
        });
    });
}

criterion_group! {
    name = primitives_bench;
    config = Criterion::default();
    targets = bench_read_byte, bench_read_short, bench_read_int, bench_read_long,
        bench_read_uleb128_1_byte, bench_read_uleb128_2_bytes, bench_read_single, bench_read_double,
        bench_read_str_empty, bench_read_str, bench_read_boolean, bench_read_datetime,
        bench_read_md5_hash, bench_read_player_name
}

criterion_main! { primitives_bench }
