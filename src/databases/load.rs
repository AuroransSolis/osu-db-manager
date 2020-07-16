use crate::load_settings::query::QueryStruct;
use crate::masks::mask::Mask;
use crate::read_error::ParseFileResult;

pub trait Load<'a>: Sized {
    fn read_from_bytes(jobs: usize, bytes: &'a [u8]) -> ParseFileResult<Self> {
        if jobs == 1 {
            Self::read_single_thread(bytes)
        } else {
            Self::read_multi_thread(jobs, bytes)
        }
    }
    fn read_single_thread(bytes: &'a [u8]) -> ParseFileResult<Self>;
    fn read_multi_thread(jobs: usize, bytes: &'a [u8]) -> ParseFileResult<Self>;
}

pub trait PartialLoad<'a, M: Mask, LS: QueryStruct<M>>: Sized {
    fn read_from_bytes(settings: LS, jobs: usize, bytes: &'a [u8]) -> ParseFileResult<Self> {
        if jobs == 1 {
            Self::read_single_thread(settings, bytes)
        } else {
            Self::read_multi_thread(settings, jobs, bytes)
        }
    }
    fn read_single_thread(settings: LS, bytes: &'a [u8]) -> ParseFileResult<Self>;
    fn read_multi_thread(settings: LS, jobs: usize, bytes: &'a [u8]) -> ParseFileResult<Self>;
}
