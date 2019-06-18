pub trait Mask: Default {
    fn is_complete(&self) -> bool;
    fn from_show_and_query(show: Self, query: Self) -> Self;
}