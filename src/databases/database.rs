use crate::argument::Database::{self, *};
use crate::databases::{
    collection::{collectiondb::CollectionDb, partial_collectiondb::PartialCollectionDb},
    load::{Load, PartialLoad},
    osu::{osudb::OsuDb, partial_osudb::PartialOsuDb},
    scores::{partial_scoresdb::PartialScoresDb, scoresdb::ScoresDb},
};
use crate::masks::mask::DbMask::{self, *};
use crate::read_error::ParseFileResult;

#[derive(Debug)]
pub enum OsuDatabase {
    Osu(OsuDb),
    Collection(CollectionDb),
    Scores(ScoresDb),
    PartialOsu(PartialOsuDb),
    PartialCollection(PartialCollectionDb),
    PartialScores(PartialScoresDb),
}

use self::OsuDatabase::*;

impl OsuDatabase {
    pub fn read_from_bytes(jobs: usize, db: Database) -> ParseFileResult<Self> {
        Ok(match db {
            OsuDb(b) => Osu(OsuDb::read_from_bytes(jobs, b)?),
            CollectionDb(b) => Collection(CollectionDb::read_from_bytes(jobs, b)?),
            ScoresDb(b) => Scores(ScoresDb::read_from_bytes(jobs, b)?),
        })
    }

    pub fn read_partial_from_bytes(
        jobs: usize,
        db: Database,
        mask: DbMask,
    ) -> ParseFileResult<Self> {
        Ok(match (db, mask) {
            (OsuDb(b), OsuMask(m)) => PartialOsu(PartialOsuDb::read_from_bytes(m, jobs, b)?),
            (CollectionDb(b), CollectionMask(m)) => {
                PartialCollection(PartialCollectionDb::read_from_bytes(m, jobs, b)?)
            }
            (ScoresDb(b), ScoresMask(m)) => {
                PartialScores(PartialScoresDb::read_from_bytes(m, jobs, b)?)
            }
            _ => unreachable!(),
        })
    }

    pub fn unwrap_osu(self) -> OsuDb {
        match self {
            Osu(osu) => osu,
            Collection(_) => panic!("Tried to unwrap a CollectionDb with 'unwrap_osu()'."),
            Scores(_) => panic!("Tried to unwrap a ScoresDb with 'unwrap_osu()'."),
            PartialOsu(_) => panic!("Tried to unwrap a PartialOsuDb with 'unwrap_osu()'."),
            PartialCollection(_) => {
                panic!("Tried to unwrap a PartialCollectionDb with 'unwrap_osu()'.")
            }
            PartialScores(_) => panic!("Tried to unwrap a PartialScoresDb with 'unwrap_osu()'."),
        }
    }

    pub fn unwrap_collection(self) -> CollectionDb {
        match self {
            Osu(_) => panic!("Tried to unwrap an OsuDb with 'unwrap_collection()'."),
            Collection(collection) => collection,
            Scores(_) => panic!("Tried to unwrap a ScoresDb with 'unwrap_collection()'."),
            PartialOsu(_) => panic!("Tried to unwrap a PartialOsuDb with 'unwrap_collection()'."),
            PartialCollection(_) => {
                panic!("Tried to unwrap a PartialCollectionDb with 'unwrap_collection()'.")
            }
            PartialScores(_) => {
                panic!("Tried to unwrap a PartialScoresDb with 'unwrap_collection()'.")
            }
        }
    }

    pub fn unwrap_scores(self) -> ScoresDb {
        match self {
            Osu(_) => panic!("Tried to unwrap an OsuDb with 'unwrap_scores()'."),
            Collection(_) => panic!("Tried to unwrap a CollectionDb with 'unwrap_scores()'."),
            Scores(scores) => scores,
            PartialOsu(_) => panic!("Tried to unwrap a PartialOsuDb with 'unwrap_scores()'."),
            PartialCollection(_) => {
                panic!("Tried to unwrap a PartialCollectionDb with 'unwrap_scores()'.")
            }
            PartialScores(_) => panic!("Tried to unwrap a PartialScoresDb with 'unwrap_scores()'."),
        }
    }

    pub fn unwrap_partial_osu(self) -> PartialOsuDb {
        match self {
            Osu(_) => panic!("Tried to unwrap an OsuDb with 'unwrap_partial_osu()'."),
            Collection(_) => panic!("Tried to unwrap a CollectionDb with 'unwrap_partial_osu()'."),
            Scores(_) => panic!("Tried to unwrap a ScoresDb with 'unwrap_partial_osu()'."),
            PartialOsu(partial_osu) => partial_osu,
            PartialCollection(_) => {
                panic!("Tried to unwrap a PartialCollectionDb with 'unwrap_partial_osu()'.")
            }
            PartialScores(_) => {
                panic!("Tried to unwrap a PartialScoresDb with 'unwrap_partial_osu()'.")
            }
        }
    }

    pub fn unwrap_partial_collection(self) -> PartialCollectionDb {
        match self {
            Osu(_) => panic!("Tried to unwrap an OsuDb with 'unwrap_partial_collection()'."),
            Collection(_) => {
                panic!("Tried to unwrap a CollectionDb with 'unwrap_partial_collection()'.")
            }
            Scores(_) => panic!("Tried to unwrap a ScoresDb with 'unwrap_partial_collection()'."),
            PartialOsu(_) => {
                panic!("Tried to unwrap a PartialOsuDb with 'unwrap_partial_collection()'.")
            }
            PartialCollection(partial_collection) => partial_collection,
            PartialScores(_) => {
                panic!("Tried to unwrap a PartialScoresDb with 'unwrap_partial_collection()'.")
            }
        }
    }

    pub fn unwrap_partial_scores(self) -> PartialScoresDb {
        match self {
            Osu(_) => panic!("Tried to unwrap an OsuDb with 'unwrap_partial_scores()'."),
            Collection(_) => {
                panic!("Tried to unwrap a CollectionDb with 'unwrap_partial_scores()'.")
            }
            Scores(_) => panic!("Tried to unwrap a ScoresDb with 'unwrap_partial_scores()'."),
            PartialOsu(_) => {
                panic!("Tried to unwrap a PartialOsuDb with 'unwrap_partial_scores()'.")
            }
            PartialCollection(_) => {
                panic!("Tried to unwrap a PartialCollectionDb with 'unwrap_partial_scores()'.")
            }
            PartialScores(partial_scores) => partial_scores,
        }
    }
}
