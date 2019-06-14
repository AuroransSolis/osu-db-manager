use std::path::Path;

pub trait QueryStruct {}

pub trait PartialDb {}

pub trait Query<Q: QueryStruct, R: PartialDb> {
    fn query(&self, query: Q) -> Result<R, String>;
}