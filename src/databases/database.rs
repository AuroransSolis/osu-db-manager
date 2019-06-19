use crate::databases::{
    osu::{osudb::OsuDb, partial_osudb::PartialOsuDb},
    collection::{collectiondb::CollectionDb, partial_collectiondb::PartialCollectionDb},
    scores::{scoresdb::ScoresDb, partial_scoresdb::PartialScoresDb},
    load::Load
};
use crate::argument::Database;
use crate::read_error::ParseFileResult;

#[derive(Debug)]
pub enum OsuDatabase {
    Complete(CompleteDatabase),
    Partial
}

#[derive(Debug)]
pub enum CompleteDatabase {
    Osu(OsuDb),
    Collection(CollectionDb),
    Scores(ScoresDb)
}

#[derive(Debug)]
pub enum PartialDatabase {
    Osu(PartialOsuDb),
    Collection(PartialCollectionDb),
    Scores(Partial)
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