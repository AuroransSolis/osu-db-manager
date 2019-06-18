use crate::read_error::ParseFileResult;
use crate::masks::mask::Mask;

pub trait Load: Sized {
    fn read_from_bytes(jobs: usize, bytes: Vec<u8>) -> ParseFileResult<Self> {
        if jobs == 1 {
            Self::read_single_thread(bytes)
        } else {
            Self::read_multi_thread(jobs, bytes)
        }
    }
    fn read_single_thread(bytes: Vec<u8>) -> ParseFileResult<Self>;
    fn read_multi_thread(jobs: usize, bytes: Vec<u8>) -> ParseFileResult<Self>;
}

pub trait PartialLoad<M: Mask>: Sized {
    fn read_from_bytes(mask: M, jobs: usize, bytes: Vec<u8>) -> ParseFileResult<Self> {
        if jobs == 1 {
            Self::read_single_thread(mask, bytes)
        } else {
            Self::read_multi_thread(mask, jobs, bytes)
        }
    }
    fn read_single_thread(mask: M, bytes: Vec<u8>) -> ParseFileResult<Self>;
    fn read_multi_thread(mask: M, jobs: usize, bytes: Vec<u8>) -> ParseFileResult<Self>;
}