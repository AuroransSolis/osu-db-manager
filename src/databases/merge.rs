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

use self::ConflictResolution::*;

impl ConflictResolution {
    pub fn from_argument(arg: &str) -> Option<Self> {
        match arg {
            "ignore-duplicates" => Some(IgnoreDuplicates),
            "replace-destination" => Some(ReplaceDestination),
            "merge-subentries" => Some(MergeSubentries),
            _ => {
                let mut split = arg.split('=');
                let resolution_type = match split.next() {
                    Some(s) => s,
                    None => return None
                };
                let ps = match split.next() {
                    Some(s) => s,
                    None => return None,
                };
                match resolution_type {
                    "rename-source-with-prefix" => Some(RenameSourceWithPrefix(ps.to_string())),
                    "rename-source-with-suffix" => Some(RenameSourceWithSuffix(ps.to_string())),
                    "rename-destination-with-prefix" => Some(RenameDestinationWithPrefix(
                        ps.to_string())),
                    "rename-destination-with-suffix" => Some(RenameDestinationWithSuffix(
                        ps.to_string())),
                    _ => None
                }
            }
        }
    }
}

pub trait Merge {
    fn merge_checked(self, conflict_resolution: ConflictResolution, other: Self, osudb: OsuDb)
        -> Self;
    fn merge_unchecked(self, conflict_resolution: ConflictResolution, other: Self) -> Self;
}