pub mod collection_mask;
pub mod osu_mask;
pub mod scores_mask;

use crate::masks::{
    collection_mask::CollectionDbMask, osu_mask::OsuDbMask, scores_mask::ScoresDbMask,
};
use structopt::StructOpt;

#[derive(StructOpt)]
pub enum DbMask {
    #[structopt(name = "collection-show")]
    CollectionMask(CollectionDbMask),
    #[structopt(name = "osu-show")]
    OsuMask(OsuDbMask),
    #[structopt(name = "scores-show")]
    ScoresMask(ScoresDbMask),
}

#[macro_export]
macro_rules! maybe_print {
    ($mask_field:expr, $db_field:expr) => {{
        if $mask_field && $db_field.is_some() {
            println!(
                "{}: {}",
                stringify!($db_field)
                    .replace("_", " ")
                    .rsplit("self.")
                    .next()
                    .unwrap(),
                $db_field.as_ref().unwrap(),
            );
        }
    }};
    ($mask_field:expr, $db_field:expr, $indent:literal) => {{
        if $mask_field && $db_field.is_some() {
            println!(
                "{}{}: {}",
                $indent,
                stringify!($db_field)
                    .replace("_", " ")
                    .rsplit("self.")
                    .next()
                    .unwrap(),
                $db_field.as_ref().unwrap(),
            );
        }
    }};
}

#[macro_export]
macro_rules! maybe_print_vec {
    ($mask_field:expr, $db_field:expr, $section:literal) => {{
        if $mask_field && $db_field.is_some() {
            println!("{} {{", $section);
            for item in $db_field.as_ref().unwrap() {
                println!("    {}", item);
            }
            println!("}}");
        }
    }};
    ($mask_field:expr, $db_field:expr, $section:literal, $indent:literal) => {{
        if $mask_field && $db_field.is_some() {
            println!("{}{} {{", $indent, $section);
            for item in $db_field.as_ref().unwrap() {
                println!("{}    {}", $indent, item);
            }
            println!("{}}}");
        }
    }};
}
