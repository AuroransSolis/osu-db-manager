use std::path::Path;

pub trait QueryStruct {}

pub trait PartialDb {}

pub trait Query<Q: QueryStruct, R: PartialDb> {
    fn query_loaded(&self, query: Q) -> R;
    fn load_and_query<P: Into<Path>>(path: P, jobs: usize, query: Q) -> R;
}