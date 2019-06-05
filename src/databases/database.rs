use std::io::Result as IoResult;
use crate::databases::{osu::OsuDb, collection::CollectionDb, scores::ScoresDb, load::Load};
use crate::argument::Database;

pub enum OsuDatabase {
    Osu(OsuDb),
    Collection(CollectionDb),
    Scores(ScoresDb)
}

impl OsuDatabase {
    pub fn read_from_bytes<Db: LoadDb>(jobs: usize, db: Database) -> IoResult<Self> {
        match db {
            Database::OsuDb(bytes) => OsuDb::read_from_bytes(jobs, bytes),
            Database::CollectionDb(bytes) => CollectionDb::read_from_bytes(jobs, bytes),
            Database::ScoresDb(bytes) => ScoresDb::read_from_bytes(jobs, bytes)
        }
    }
}