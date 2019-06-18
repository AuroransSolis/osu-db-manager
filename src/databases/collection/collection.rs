use crate::read_error::ParseFileResult;
use crate::deserialize_primitives::*;

#[derive(Debug, Clone)]
pub struct Collection {
    pub collection_name: Option<String>,
    pub number_of_beatmaps: i32,
    pub md5_beatmap_hashes: Vec<Option<String>>
}

impl Collection {
    pub fn read_from_bytes(bytes: &[u8], i: &mut usize) -> ParseFileResult<Self> {
        let collection_name = read_string_utf8(bytes, i, "collection name")?;
        let number_of_beatmaps = read_int(bytes, i)?;
        let mut md5_beatmap_hashes = Vec::with_capacity(number_of_beatmaps as usize);
        for _ in 0..number_of_beatmaps {
            md5_beatmap_hashes.push(read_md5_hash(bytes, i)?);
        }
        Ok(Collection {
            collection_name,
            number_of_beatmaps,
            md5_beatmap_hashes
        })
    }
}