use std::io::Result as IoResult;

pub trait Load: Sized {
    fn read_from_bytes(jobs: usize, bytes: Vec<u8>) -> IoResult<Self> {
        if jobs == 1 {
            Self::read_single_thread(bytes)
        } else {
            Self::read_multi_thread(jobs, bytes)
        }
    }
    fn read_single_thread(bytes: Vec<u8>) -> IoResult<Self>;
    fn read_multi_thread(jobs: usize, bytes: Vec<u8>) -> IoResult<Self>;
}
