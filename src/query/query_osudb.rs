use crate::databases::osu::{OsuDb, Beatmap, TimingPoint, RankedStatus, ByteSingle, GameplayMode};
use std::time::SystemTime;

pub struct OsuDbQuery {
    version: bool,
    folder_count: bool,
    account_unlocked: bool,
    account_unlock_date: bool,
    player_name: bool,
    number_of_beatmaps: bool,
    beatmap: Option<BeatmapQuery>,
    unknown_int: bool
}

impl OsuDbQuery {
    pub fn new(bools: [bool; 7], beatmap_query: Option<BeatmapQuery>) -> Self {
        OsuDbQuery {
            version: bools[0],
            folder_count: bools[1],
            account_unlocked: bools[2],
            account_unlock_date: bools[3],
            player_name: bools[4],
            number_of_beatmaps: bools[5],
            beatmap: beatmap_query,
            unknown_int: bools[6]
        }
    }
}

impl OsuDb {
    pub fn query(&self, q: OsuDbQuery) -> String {
        let mut out = String::new();
        if q.version {
            out = format!("Version: {}", self.version);
        }
        if q.folder_count && out.len() > 0 {
            out = format!("\nFolder count: {}", self.folder_count);
        } else if q.folder_count {
            out = format!("Folder count: {}", self.folder_count);
        }
        if q.account_unlocked && out.len() > 0 {
            out = format!("\nAccount unlocked: {}", self.account_unlocked);
        } else if q.account_unlocked {
            out = format!("Account unlocked: {}", self.account_unlocked);
        }
        out
    }
}

pub enum BeatmapQuery {
    EntrySize(Option<i32>),
    AristName(String),
    ArtistNameUnicode(String),
    SongTitle(String),
    SongTitleUnicode(String),
    CreatorName(String),
    Difficulty(String),
    AudioFileName(String),
    MD5BeatmapHash(String),
    DotosuFileName(String),
    RankedStatus(RankedStatus),
    NumberOfHitcircles(i16),
    NumberOfSliders(i16),
    NumberOfSpinners(i16),
    LastModificationTime(SystemTime),
    ApproachRate(ByteSingle),
    CircleSize(ByteSingle),
    HpDrain(ByteSingle),
    OverallDifficulty(ByteSingle),
    SliderVelocity(f64),
    NumModComboStarRatingsStandard(Option<i32>),
    ModComboStarRatingsStandard(Option<Vec<(i32, f64)>>),
    NumModComboStarRatingsTaiko(Option<i32>),
    ModComboStarRatingsTaiko(Option<Vec<(i32, f64)>>),
    NumModComboStarRatingsCtb(Option<i32>),
    ModComboStarRatingsCtb(Option<Vec<(i32, f64)>>),
    NumModComboStarRatingsMania(Option<i32>),
    ModComboStarRatingsMania(Option<Vec<(i32, f64)>>),
    DrainTime(i32),
    TotalTime(i32),
    PreviewOffsetFromStartMs(i32),
    NumTimingPoints(i32),
    TimingPoints(Vec<TimingPoint>),
    BeatmapId(i32),
    BeatmapSetId(i32),
    ThreadId(i32),
    StandardGrade(u8),
    TaikoGrade(u8),
    CtbGrade(u8),
    ManiaGrade(u8),
    LocalOffset(i16),
    StackLeniency(f32),
    GameplayMode(GameplayMode),
    SongSource(String),
    SongTags(String),
    OnlineOffset(i16),
    FontUsedForSongTitle(String),
    Unplayed(bool),
    LastPlayed(SystemTime),
    IsOsz2(bool),
    BeatmapFolderName(String),
    LastCheckedAgainstRepo(SystemTime),
    IgnoreBeatmapSound(bool),
    IgnoreBeatmapSkin(bool),
    DisableStoryboard(bool),
    DisableVideo(bool),
    VisualOverride(bool),
    UnknownShort(Option<i16>),
    OffsetFromSongStartInEditorMs(i32),
    ManiaScrollSpeed(u8)
}