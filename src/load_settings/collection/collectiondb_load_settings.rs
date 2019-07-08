use std::io::Result as IoResult;

use chrono::NaiveDate;

use crate::read_error::{ParseFileResult, DbFileParseError, ParseErrorKind::QueryError};
use crate::load_settings::{
    collection::collection_load_settings::CollectionLoadSettings, query::QueryStruct, LoadSetting
};
use crate::masks::collection_mask::CollectionDbMask;

pub struct CollectionDbLoadSettings {
    pub version: LoadSetting<i32>,
    pub number_of_collections: LoadSetting<i32>,
    pub collections_query: CollectionLoadSettings
}

impl QueryStruct<CollectionDbMask> for CollectionDbLoadSettings {
    fn load_all(&self) -> bool {
        self.collections_query.load_all()
            && !(self.version.is_ignore() || self.number_of_collections.is_ignore())
    }

    fn set_from_query(&mut self, args: Vec<&str>) -> IoResult<()> {
        self.collections_query.set_from_query(args)
    }

    fn set_from_mask(&mut self, mask: CollectionDbMask) {
        if self.version.is_ignore() && mask.version {
            self.version = LoadSetting::Load;
        }
        if self.number_of_collections.is_ignore() && mask.number_of_collections {
            self.number_of_collections = LoadSetting::Load;
        }
        if let Some(m) = mask.collections_mask {
            self.collections_query.set_from_mask(m);
        }
    }
}