use crate::load_settings::query::QueryStruct;
use crate::masks::mask::Mask;
use crate::read_error::ParseFileResult;

pub trait Load: Sized {
    fn read_from_bytes(jobs: usize, bytes: &[u8]) -> ParseFileResult<Self> {
        if jobs == 1 {
            Self::read_single_thread(bytes)
        } else {
            Self::read_multi_thread(jobs, bytes)
        }
    }
    fn read_single_thread(bytes: &[u8]) -> ParseFileResult<Self>;
    fn read_multi_thread(jobs: usize, bytes: &[u8]) -> ParseFileResult<Self>;
}

pub trait PartialLoad<M: Mask, LS: QueryStruct<M>>: Sized {
    fn read_from_bytes(settings: LS, jobs: usize, bytes: &[u8]) -> ParseFileResult<Self> {
        if jobs == 1 {
            Self::read_single_thread(mask, bytes)
        } else {
            Self::read_multi_thread(mask, jobs, bytes)
        }
    }
    fn read_single_thread(settings: LS, bytes: &[u8]) -> ParseFileResult<Self>;
    fn read_multi_thread(settings: LS, jobs: usize, bytes: &[u8]) -> ParseFileResult<Self>;
}
