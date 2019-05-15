use std::path::Path;

pub trait QueryStruct {}

pub trait PartialDb {}

pub trait Query {
    fn query_loaded<T: QueryStruct, R: PartialDb>(&self, query: T) -> R;
    fn load_and_query<T: QueryStruct, R: PartialDb, P: Into<Path>>(path: P, jobs: usize,
        query: T) -> R;
}