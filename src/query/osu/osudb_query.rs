use crate::read_error::{ParseFileResult, DbFileParseError, ParseErrorKind::QueryError};

use crate::query::{AskCompareIgnore, Comparison, osu::beatmap_query::BeatmapQuery};
use crate::masks::osu_mask::OsuDbMask;

pub struct OsuDbQuery {
    pub version: AskCompareIgnore<bool>,
    pub folder_count: AskCompareIgnore<bool>,
    pub account_unlocked: AskCompareIgnore<bool>,
    pub account_unlock_date: AskCompareIgnore<bool>,
    pub player_name: AskCompareIgnore<bool>,
    pub number_of_beatmaps: AskCompareIgnore<bool>,
    pub beatmap: Option<BeatmapQuery>,
    pub unknown_int: AskCompareIgnore<bool>
}

impl OsuDbQuery {
    fn new() -> Self {
        OsuDbQuery {
            version: AskCompareIgnore::Ignore,
            folder_count: AskCompareIgnore::Ignore,
            account_unlocked: AskCompareIgnore::Ignore,
            account_unlock_date: AskCompareIgnore::Ignore,
            player_name: AskCompareIgnore::Ignore,
            number_of_beatmaps: AskCompareIgnore::Ignore,
            beatmap: None,
            unknown_int: AskCompareIgnore::Ignore
        }
    }
    
    pub fn from_string(s: String) -> ParseFileResult<Self> {
        let mut query = OsuDbQuery::new();
        let mut modified = [false; 8];
        for parameter in s.split(',') {
            let field = parameter.split('=').next().unwrap();
            match field {
                "VERSION" => {

                }
            }
        }
        Ok(query)
    }
}

impl Into<OsuDbMask> for OsuDbQuery {
    fn into(self) -> OsuDbMask {

    }
}

macro_rules! query {
    ($item:tt, $query:ident) => {
        if $query.$item {
            Some(self.$item)
        } else {
            None
        }
    }
}

impl Query for OsuDb {
    fn query_loaded(&self, query: OsuDbQuery) -> PartialOsuDb {
        let version = query!(version, query);
        let folder_count = query!(folder_count, query);
        let account_unlocked = query!(account_unlocked, query);
        let account_unlock_date = query!(account_unlock_date, query);
        let player_name = query!(player_name, query);
        let number_of_beatmaps = query!(number_of_beatmaps, query);
        let beatmaps = query!(beatmaps, query);
        let unknown_int = query!(unknown_int, query);
    }

    fn load_and_query<P: Into<Path>>(path: P, query: OsuDbQuery) -> PartialOsuDb {

    }
}