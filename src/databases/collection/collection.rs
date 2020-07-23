use crate::deserialize_primitives::*;
use crate::read_error::ParseFileResult;

/// The collection is the entry type in collection.db. Each entry has a name and the hashes of the
/// beatmaps in a given collection.
#[derive(Debug, Clone)]
pub struct Collection<'a> {
    pub collection_name: Option<&'a str>,
    pub number_of_beatmaps: i32,
    pub md5_beatmap_hashes: Vec<&'a str>,
}

impl<'a> Collection<'a> {
    pub fn read_from_bytes(bytes: &'a [u8], i: &mut usize) -> ParseFileResult<Self> {
        let collection_name = read_str_utf8(bytes, i, "collection name")?;
        let number_of_beatmaps = read_int(bytes, i)?;
        let mut md5_beatmap_hashes = Vec::with_capacity(number_of_beatmaps as usize);
        for _ in 0..number_of_beatmaps {
            md5_beatmap_hashes.push(read_md5_hash(bytes, i)?);
        }
        Ok(Collection {
            collection_name,
            number_of_beatmaps,
            md5_beatmap_hashes,
        })
    }

    pub fn display(&self) {
        if self.collection_name.is_some() {
            println!(
                "    collection name: {}",
                self.collection_name.as_ref().unwrap()
            );
        } else {
            println!("    collection name: \"\"");
        }
        println!("    number of beatmaps: {}", self.number_of_beatmaps);
        println!("    Hashes {{");
        for &hash in &self.md5_beatmap_hashes {
            println!("        {}", hash);
        }
        println!("    }}");
    }
}
