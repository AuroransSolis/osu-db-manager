use crate::read_error::{ParseFileResult, ParseErrorKind::PrimitiveError, DbFileParseError};
use crate::deserialize_primitives::*;
use crate::maybe_deserialize_primitives::*;
use crate::load_settings::{
    FilterResult,
    collection::collection_load_settings::CollectionLoadSettings
};

#[derive(Debug, Clone)]
pub struct PartialCollection {
    pub collection_name: Option<String>,
    pub number_of_beatmaps: Option<i32>,
    pub md5_beatmap_hashes: Option<Vec<String>>
}

impl PartialCollection {
    pub fn read_from_bytes(settings: CollectionLoadSettings, bytes: &[u8], i: &mut usize)
        -> ParseFileResult<Option<Self>> {
        let mut skip = false;
        let collection_name = maybe_read_string_utf8(settings.collection_name, &mut skip, bytes,
            i, "collection name")?;
        let number_of_beatmaps = read_int(bytes, i)?;
        let md5_beatmap_hashes = if !settings.md5_beatmap_hash.is_ignore() && !skip {
            if number_of_beatmaps == 0 {
                None
            } else {
                let mut tmp = Vec::with_capacity(number_of_beatmaps as usize);
                for _ in 0..number_of_beatmaps {
                    if let Some(hash) = maybe_read_md5_hash(settings.md5_beatmap_hash.clone(),
                        &mut false, bytes, i)? {
                        tmp.push(hash)
                    }
                }
                Some(tmp)
            }
        } else {
            *i += number_of_beatmaps as usize * 34;
            None
        };
        let number_of_beatmaps = if settings.number_of_beatmaps.is_load() {
            Some(number_of_beatmaps)
        } else {
            None
        };
        if skip {
            Ok(None)
        } else {
            Ok(Some(PartialCollection {
                collection_name,
                number_of_beatmaps,
                md5_beatmap_hashes
            }))
        }
    }
}