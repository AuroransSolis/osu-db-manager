use std::fs::File;
use std::io::{Result as IoResult, ErrorKind};
use std::env;

pub trait Load: Sized {
    fn read_from_bytes(jobs: usize, bytes: Vec<u8>) -> IoResult<Self> {
        if jobs == 1 {
            Self::read_single_thread(bytes)
        } else {
            let rayon_jobs_default = env::var("RAYON_NUM_THREADS").or_else(env::var("RAYON_RS_NUM_CPUS"))?;
            let rayon_jobs_default = rayon_jobs_default.parse::<usize>()
                .map_err(|err| IoError::new(ErrorKind::Other, format!("{:?}", err)))?;
            if jobs != rayon_jobs_default {
                build_global_threadpool_with_jobs(jobs);
            }
            Self::read_multi_thread(jobs, bytes)
        }
    }
    fn read_single_thread(bytes: Vec<u8>) -> IoResult<Self>;
    fn read_multi_thread(jobs: usize, bytes: Vec<u8>) -> IoResult<Self>;
}