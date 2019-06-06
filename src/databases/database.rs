use std::io::Result as IoResult;
use crate::databases::{osu::OsuDb, collection::CollectionDb, scores::ScoresDb, load::Load};
use crate::argument::Database;

#[derive(Debug)]
pub enum OsuDatabase {
    Osu(OsuDb),
    Collection(CollectionDb),
    Scores(ScoresDb)
}

use self::OsuDatabase::*;

impl OsuDatabase {
    pub fn read_from_bytes(jobs: usize, db: Database) -> IoResult<Self> {
        Ok(match db {
            Database::OsuDb(b) => {
                println!("Loading osu!.db with {} thread(s).", jobs);
                Osu(OsuDb::read_from_bytes(jobs, b)?)
            },
            Database::CollectionDb(b) => {
                println!("Loading collection.db with {} thread(s).", jobs);
                Collection(CollectionDb::read_from_bytes(jobs, b)?)
            },
            Database::ScoresDb(b) => {
                println!("Loading scores.db with {} thread(s).", jobs);
                Scores(ScoresDb::read_from_bytes(jobs, b)?)
            }
        })
    }

    pub fn unwrap_osu(self) -> OsuDb {
        match self {
            Osu(osu) => osu,
            _ => unreachable!()
        }
    }

    pub fn unwrap_collection(self) -> CollectionDb {
        match self {
            Collection(collection) => collection,
            _ => unreachable!()
        }
    }

    pub fn unwrap_scores(self) -> ScoresDb {
        match self {
            Scores(scores) => scores,
            _ => unreachable!()
        }
    }
}