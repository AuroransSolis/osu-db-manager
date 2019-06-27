pub trait QueryStruct {}

pub trait Query<Q: QueryStruct> {
    type Output;

    fn query(bytes: Vec<u8>, query: Q) -> <Self as Query>::Output;
}