use std::io::Result as IoResult;

use crate::masks::mask::Mask;

pub trait QueryStruct<M: Mask> {
    fn load_all(&self) -> bool;
    fn ignore_all(&self) -> bool;
    fn is_partial(&self) -> bool;
    fn set_from_query(&mut self, query_args: Vec<&str>) -> IoResult<()>;
    fn set_from_mask(&mut self, mask: &M);
}

pub trait Query<M: Mask, Q: QueryStruct<M>> {
    type Output;

    fn query(bytes: Vec<u8>, query: Q) -> Self::Output;
}