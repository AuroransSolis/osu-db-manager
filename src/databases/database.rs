use crate::databases::{
    complete::{osu::OsuDb, collection::CollectionDb, scores::ScoresDb},
    partial::{
        partial_osu::PartialOsuDb,
        partial_collection::PartialCollectionDb,
        partial_scores::PartialScoresDb
    },
    load::Load};
use crate::argument::Database;
use crate::read_error::ParseFileResult;

#[derive(Debug)]
pub enum OsuDatabase {
    Complete(CompleteDatabase),
    Partial
}

pub enum CompleteDatabase {
    Osu(OsuDb),
    Collection(CollectionDb),
    Scores(ScoresDb)
}

pub enum PartialDatabase {

}

use self::OsuDatabase::*;

impl OsuDatabase {
    pub fn read_from_bytes(jobs: usize, db: DatabaseAndMask) -> ParseFileResult<Self> {
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
            Collection(_) => panic!("Tried to unwrap a CollectionDb with 'unwrap_osu()'."),
            Scores(_) => panic!("Tried to unwrap a ScoresDb with 'unwrap_osu()'.")
        }
    }

    pub fn unwrap_collection(self) -> CollectionDb {
        match self {
            Osu(_) => panic!("Tried to unwrap an OsuDb with 'unwrap_collection()'."),
            Collection(collection) => collection,
            Scores(_) => panic!("Tried to unwrap a ScoresDb with 'unwrap_collection()'.")
        }
    }

    pub fn unwrap_scores(self) -> ScoresDb {
        match self {
            Osu(_) => panic!("Tried to unwrap an OsuDb with 'unwrap_scores()'."),
            Collection(_) => panic!("Tried to unwrap a CollectionDb with 'unwrap_scores()'."),
            Scores(scores) => scores
        }
    }
}