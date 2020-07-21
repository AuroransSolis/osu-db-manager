use crate::argument::DbIndicator;
use crate::databases::{
    collection::{collectiondb::CollectionDb, partial_collectiondb::PartialCollectionDb},
    osu::{osudb::OsuDb, partial_osudb::PartialOsuDb},
    scores::{partial_scoresdb::PartialScoresDb, scoresdb::ScoresDb},
};
use crate::load_settings::LoadSettings::{self, *};
use crate::read_error::ParseFileResult;

#[derive(Debug)]
pub enum OsuDatabase<'a> {
    Osu(OsuDb<'a>),
    Collection(CollectionDb<'a>),
    Scores(ScoresDb<'a>),
    PartialOsu(PartialOsuDb<'a>),
    PartialCollection(PartialCollectionDb<'a>),
    PartialScores(PartialScoresDb<'a>),
}

use self::OsuDatabase::*;

impl<'a> OsuDatabase<'a> {
    pub fn read_from_bytes(jobs: usize, db_type: DbIndicator, bytes: &'a [u8]) -> ParseFileResult<Self> {
        Ok(match db_type {
            DbIndicator::OsuDb => Osu(OsuDb::read_from_bytes(jobs, bytes)?),
            DbIndicator::CollectionDb => Collection(CollectionDb::read_from_bytes(jobs, bytes)?),
            DbIndicator::ScoresDb => Scores(ScoresDb::read_from_bytes(jobs, bytes)?),
        })
    }

    pub fn read_partial_from_bytes(
        jobs: usize,
        settings: LoadSettings,
        bytes: &'a [u8],
    ) -> ParseFileResult<Self> {
        Ok(match settings {
            OsuSettings(s) => PartialOsu(PartialOsuDb::read_from_bytes(s, jobs, bytes)?),
            CollectionSettings(s) => PartialCollection(PartialCollectionDb::read_from_bytes(s, jobs, bytes)?),
            ScoresSettings(s) => PartialScores(PartialScoresDb::read_from_bytes(s, jobs, bytes)?),
        })
    }

    pub fn unwrap_osu(self) -> OsuDb<'a> {
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

    pub fn unwrap_collection(self) -> CollectionDb<'a> {
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

    pub fn unwrap_scores(self) -> ScoresDb<'a> {
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

    pub fn unwrap_partial_osu(self) -> PartialOsuDb<'a> {
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

    pub fn unwrap_partial_collection(self) -> PartialCollectionDb<'a> {
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

    pub fn unwrap_partial_scores(self) -> PartialScoresDb<'a> {
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
