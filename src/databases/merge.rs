use crate::databases::osu::OsuDb;

pub enum ConflictResolution {
    IgnoreDuplicates,
    ReplaceDestination,
    RenameSourceWithPrefix(String),
    RenameSourceWithSuffix(String),
    RenameDestinationWithPrefix(String),
    RenameDestinationWithSuffix(String),
    MergeSubentries
}

pub trait Merge {
    fn merge_checked(self, conflict_resolution: ConflictResolution, other: Self, osudb: OsuDb)
        -> Self;
    fn merge_unchecked(self, conflict_resolution: ConflictResolution, other: Self) -> Self;
}