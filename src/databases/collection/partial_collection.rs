use crate::deserialize_primitives::*;
use crate::load_settings::collection::collection_load_settings::CollectionLoadSettings;
use crate::maybe_deserialize_primitives::*;
use crate::read_error::ParseFileResult;
use crate::{masks::collection_mask::CollectionMask, maybe_print, maybe_print_vec};

#[derive(Debug, Clone)]
pub struct PartialCollection<'a> {
    pub collection_name: Option<&'a str>,
    pub number_of_beatmaps: Option<i32>,
    pub md5_beatmap_hashes: Option<Vec<&'a str>>,
}

impl<'a> PartialCollection<'a> {
    pub fn read_from_bytes(
        settings: CollectionLoadSettings,
        bytes: &'a [u8],
        i: &mut usize,
    ) -> ParseFileResult<Option<Self>> {
        let mut skip = false;
        let collection_name = maybe_read_str_utf8(
            &settings.collection_name,
            &mut skip,
            bytes,
            i,
            "collection name",
        )?;
        let number_of_beatmaps = read_int(bytes, i)?;
        let md5_beatmap_hashes = if settings.md5_beatmap_hash.is_ignore() {
            *i += number_of_beatmaps as usize * 34;
            None
        } else {
            if number_of_beatmaps == 0 {
                None
            } else {
                let mut tmp = Vec::with_capacity(number_of_beatmaps as usize);
                for _ in 0..number_of_beatmaps {
                    // Only load in the hashes that match the one we care about. Ignore the others.
                    if let Some(hash) = maybe_read_md5_hash(
                        &settings.md5_beatmap_hash.clone(),
                        &mut false,
                        bytes,
                        i,
                    )? {
                        tmp.push(hash);
                    }
                }
                Some(tmp)
            }
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
                md5_beatmap_hashes,
            }))
        }
    }

    pub fn display(&self, show: CollectionMask) {
        maybe_print!(show.collection_name, self.collection_name);
        maybe_print!(show.number_of_beatmaps, self.number_of_beatmaps);
        maybe_print_vec!(show.md5_beatmap_hashes, self.md5_beatmap_hashes, "hashes");
    }
}
