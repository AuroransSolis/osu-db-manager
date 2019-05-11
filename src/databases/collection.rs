use std::fs::File;
use std::io::{Result as IoResult, Error as IoError, ErrorKind::InvalidData};
use std::fmt::{Display, Formatter, Result as FmtResult};
use byteorder::ReadBytesExt;
use crate::deserialize_primitives::*;

pub struct Collection {
    collection_name: String,
    number_of_beatmaps: i32,
    md5_beatmap_hashes: Vec<String>
}

impl Collection {
    pub fn read_from_file(file: &mut File) -> IoResult<Self> {
        let collection_name = fromutf8_to_ioresult(read_string_utf8(file)?, "collection name")?;
        let number_of_beatmaps = read_int(file)?;
        let mut md5_beatmap_hashes = Vec::with_capacity(number_of_beatmaps as usize);
        for _ in 0..number_of_beatmaps {
            md5_beatmap_hashes.push(fromutf8_to_ioresult(read_string_utf8(file)?,
                "MD5 beatmap hash")?);
        }
        Ok(Collection {
            collection_name,
            number_of_beatmaps,
            md5_beatmap_hashes
        })
    }
}

pub struct CollectionDb {
    version: i32,
    number_of_collections: i32,
    collections: Vec<Collection>
}

impl CollectionDb {
    pub fn read_from_file(file: &mut File) -> IoResult<Self> {
        let version = read_int(file)?;
        let number_of_collections = read_int(file)?;
        let mut collections = Vec::with_capacity(number_of_collections as usize);
        for _ in 0..number_of_collections {
            collections.push(Collection::read_from_file(file)?);
        }
        Ok(CollectionDb {
            version,
            number_of_collections,
            collections
        })
    }
}