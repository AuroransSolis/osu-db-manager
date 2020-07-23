use crate::argument::DbIndicator;
use crate::databases::{
    collection::{collectiondb::CollectionDb, partial_collectiondb::PartialCollectionDb},
    osu::{osudb::OsuDb, partial_osudb::PartialOsuDb},
    scores::{partial_scoresdb::PartialScoresDb, scoresdb::ScoresDb},
};
use crate::load_settings::LoadSettings::{self, *};
use crate::masks::DbMask;
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
    pub fn read_from_bytes(
        jobs: usize,
        db_type: DbIndicator,
        bytes: &'a [u8],
    ) -> ParseFileResult<Self> {
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
            CollectionSettings(s) => {
                PartialCollection(PartialCollectionDb::read_from_bytes(s, jobs, bytes)?)
            }
            ScoresSettings(s) => PartialScores(PartialScoresDb::read_from_bytes(s, jobs, bytes)?),
        })
    }

    pub fn display(&self, show: Option<DbMask>) {
        match (self, show) {
            (Osu(osudb), None) => osudb.display(),
            (Collection(collectiondb), None) => collectiondb.display(),
            (Scores(scoresdb), None) => scoresdb.display(),
            (PartialOsu(partialosudb), Some(DbMask::OsuMask(mask))) => partialosudb.display(mask),
            (PartialCollection(partialcollectiondb), Some(DbMask::CollectionMask(mask))) => {
                partialcollectiondb.display(mask)
            }
            (PartialScores(partialscoresdb), Some(DbMask::ScoresMask(mask))) => {
                partialscoresdb.display(mask)
            }
            _ => unreachable!(),
        }
    }
}
