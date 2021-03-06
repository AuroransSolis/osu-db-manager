macro_rules! continue_if {
    ($skip:expr) => {
        if $skip {
            continue;
        }
    };
}

pub mod collection;
pub mod database;
pub mod merge;
pub mod osu;
pub mod scores;
