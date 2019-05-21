use std::fs::File;
use std::io::Result as IoResult;
use byteorder::ReadBytesExt;

pub trait LoadSettings {}

pub trait Load: Sized {
    fn read_from_file<S: LoadSettings>(settings: S, mut file: File) -> IoResult<Self>;
    fn read_single_thread<S: LoadSettings>(settings: S, mut file: File) -> IoResult<Self>;
    fn read_multi_thread<S: LoadSettings>(settings: S, mut file: File) -> IoResult<Self>;
}