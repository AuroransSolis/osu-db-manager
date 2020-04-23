use std::io::Result as IoResult;

use chrono::NaiveDate;

use crate::load_settings::{
    collection::collection_load_settings::CollectionLoadSettings, query::QueryStruct, LoadSetting,
};
use crate::masks::collection_mask::CollectionDbMask;
use crate::read_error::{DbFileParseError, ParseErrorKind::QueryError, ParseFileResult};

pub struct CollectionDbLoadSettings {
    pub version: bool,
    pub number_of_collections: bool,
    pub collections_query: CollectionLoadSettings,
}

impl QueryStruct<CollectionDbMask> for CollectionDbLoadSettings {
    fn load_all(&self) -> bool {
        self.version && self.number_of_collections && self.collections_query.load_all()
    }

    fn ignore_all(&self) -> bool {
        !self.version && !self.number_of_collections && self.collections_query.ignore_all()
    }

    fn is_partial(&self) -> bool {
        !self.version || !self.number_of_collections || self.collections_query.is_partial()
    }

    fn set_from_query(&mut self, args: Vec<&str>) -> IoResult<()> {
        self.collections_query.set_from_query(args)
    }

    fn set_from_mask(&mut self, mask: CollectionDbMask) {
        self.version |= mask.version;
        self.number_of_collections |= mask.number_of_collections;
        self.collections_query.set_from_mask(&mask.collections_mask);
    }
}
