use crate::read_error::{ParseFileResult, ParseErrorKind::PrimitiveError, DbFileParseError};
use crate::deserialize_primitives::*;
use crate::masks::collection_mask::CollectionMask;

#[derive(Debug, Clone)]
pub struct PartialCollection {
    pub collection_name: Option<String>,
    pub number_of_beatmaps: i32,
    pub md5_beatmap_hashes: Option<Vec<String>>
}

impl PartialCollection {
    pub fn read_from_bytes(mask: CollectionMask, bytes: &[u8], i: &mut usize)
        -> ParseFileResult<Self> {
        let collection_name = if mask.collection_name {
            read_string_utf8(bytes, i, "collection name")?
        } else {
            let ind = read_byte(bytes, i)?;
            if ind == 0x0b {
                let len = read_uleb128(bytes, i)?;
                *i += len;
            } else if ind != 0 {
                return Err(DbFileParseError::new(PrimitiveError, "Read invalid string indicator!"));
            }
            None
        };
        let number_of_beatmaps = read_int(bytes, i)?;
        let mut md5_beatmap_hashes = if mask.md5_beatmap_hashes {
            let mut tmp = Vec::with_capacity(number_of_beatmaps as usize);
            for _ in 0..number_of_beatmaps {
                tmp.push(read_md5_hash(bytes, i)?);
            }
            Some(tmp)
        } else {
            for _ in 0..number_of_beatmaps {
                match read_byte(bytes, i)? {
                    0 => continue,
                    0x0b => *i += read_byte(bytes, i)? as usize,
                    _ => {
                        return Err(DbFileParseError::new(PrimitiveError,
                            "Read invalid string indicator!"));
                    }
                }
            }
            None
        };
        Ok(PartialCollection {
            collection_name,
            number_of_beatmaps,
            md5_beatmap_hashes
        })
    }
}